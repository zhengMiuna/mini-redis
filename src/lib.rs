#![feature(impl_trait_in_assoc_type)]
use std::collections::HashMap;
use std::sync::Mutex;
use volo::FastStr;

pub struct S {
	map: Mutex<HashMap<String,String>>
}

impl S {
	pub fn new() -> S {
		S{ map: Mutex::new(HashMap::new()) }
	}
}

#[volo::async_trait]
impl volo_gen::mini_redis::ItemService for S {
	async fn get_item(&self, 
		_req: volo_gen::mini_redis::GetItemRequest
	) -> ::core::result::Result<volo_gen::mini_redis::GetItemResponse, ::volo_thrift::AnyhowError>
	{
		let mut response = volo_gen::mini_redis::GetItemResponse {op: " ".into(),key: " ".into(), value: " ".into(), status: false};
		let _set =FastStr::from("set");
		let _get = FastStr::from("get");
		let _del = FastStr::from("del");
		let _ping = FastStr::from("ping");
		let _deny = FastStr::from("deny");

		if _req.op == _set {
			response.op = _set;
			let k = _req.key.to_string();
			let v = _req.value.to_string();
			if self.map.lock().unwrap().get(&k) == None {
				println!("set {}: {}",k,v);
				self.map.lock().unwrap().insert(k, v);
				response.status = true;
			}else {
				response.status = false;
			}
		}else if _req.op ==_get {
			response.op =_get;
			let k = _req.key.to_string();
			if let Some(v) = self.map.lock().unwrap().get(&k) {
				println!("get {}: {}",k,v);
				response.value = v.clone().into();
				response.status = true;
			}else {
				response.status = false;
				println!("no such {}",k);
			}
		}else if _req.op == _del {
			response.op = _del;
			let k = _req.key.to_string();
			if let Some(v) = self.map.lock().unwrap().remove(&k) {
				println!("delete {}: {}",k,v);
				response.status = true;
			}else {
				response.status = false;
				println!("no such {}",k);
			}
		}else if _req.op == _ping {
			 response.op = _ping;
			 let v = _req.value.to_string();
			 println!("ping: {}",v);
			 println!("Pong!");
			 response.status = true;
		}else if _req.op == _deny {
			panic!("set denied!");
		}else {
			panic!("invalid command");
		}

		Ok(response)
	}
}
