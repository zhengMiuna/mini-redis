#![feature(impl_trait_in_assoc_type)]

use anyhow::{anyhow, Ok, Result};
use std::{collections::HashMap, fs::{OpenOptions, File}, io::Write, sync::Mutex};
use volo::FastStr;
use volo_gen::volo::example::{GetItemResponse, RedisCommand};

pub struct S {
    pub map: Mutex<HashMap<String, String>>,
    pub aof_file: Option<Mutex<File>>,
}

impl S {
    pub fn new(map:HashMap<String, String>) -> Result<Self> {
        let mut s = S {
            map: Mutex::new(map),
            aof_file: None,
        };
        s.open_aof_file()?;
        Ok(s)
    }

    pub fn open_aof_file(&mut self) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("aof.log")
            .map_err(|e| anyhow!("Failed to open AOF file: {}", e))?;
        self.aof_file = Some(Mutex::new(file));
		//println!("Open AOF File Success");
        Ok(())
    }

    pub fn append_to_aof(&self, command: &str) -> Result<()> {
        if let Some(file_mutex) = &self.aof_file {
			let mut file = file_mutex.lock().unwrap();
			//println!("Append Success");
            writeln!(file, "{}", command)
                .map_err(|e| anyhow!("Failed to write AOF file: {}", e))?;
            file.sync_data()
                .map_err(|e| anyhow!("Failed to sync AOF file: {}", e))?;
        }
        Ok(())
    }
}

#[volo::async_trait]
impl volo_gen::volo::example::ItemService for S {
    async fn get_item(
        &self,
        _req: volo_gen::volo::example::GetItemRequest,
    ) -> ::core::result::Result<volo_gen::volo::example::GetItemResponse, ::volo_thrift::AnyhowError>
    {
        match _req.command {
            RedisCommand::Set => {
                let key = _req.key.unwrap().into_string();
                let value = _req.value.unwrap().into_string();
				//println!("SET {} {}", key, value);
				self.append_to_aof(format!("SET {} {}", key, value).as_str())?;
                self.map.lock().unwrap().insert(key.clone(), value.clone());
                Ok(GetItemResponse {
                    flag: true,
                    res: Some("OK".into()),
                })
            }
            RedisCommand::Get => {
                match self
                    .map
                    .lock()
                    .unwrap()
                    .get(&_req.key.unwrap().into_string())
                {
                    Some(v) => Ok(GetItemResponse {
                        flag: true,
                        res: Some(FastStr::from(v.clone())),
                    }),
                    None => Ok(GetItemResponse {
                        flag: false,
                        res: Some("None".into()),
                    }),
                }
            }
            RedisCommand::Del => {
                let key = _req.key.unwrap().into_string();
                match self.map.lock().unwrap().remove(&key) {
                    Some(_) => {
						self.append_to_aof(format!("DEL {}", key).as_str())?;
                        Ok(GetItemResponse {
                            flag: true,
                            res: Some("OK".into()),
                        })
                    }

                    None => Ok(GetItemResponse {
                        flag: false,
                        res: Some("None".into()),
                    }),
                }
            }
            RedisCommand::Ping => Ok(GetItemResponse {
                flag: true,
                res: Some("PONG".into()),
            }),
            RedisCommand::Publish => Ok(Default::default()),
            RedisCommand::Subscribe => Ok(Default::default()),
        }
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

#[derive(Clone)]
pub struct FliterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FliterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let info = format!("{:?}", req);
        if info.contains("fff") {
            return Err(anyhow::anyhow!("fff is not allowed").into());
        }
        self.0.call(cx, req).await
    }
}
pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
    type Service = FliterService<S>;

    fn layer(self, inner: S) -> Self::Service {
        FliterService(inner)
    }
}
