namespace rs mini_redis

struct GetItemRequest {
1: required string op,
2: required string key,
3: required string value,
}

struct GetItemResponse {
1: required string op,
2: required string key,
3: required string value,
4: required bool status,
}

service ItemService {
GetItemResponse GetItem (1: GetItemRequest req),
}

