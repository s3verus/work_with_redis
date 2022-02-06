# work_with_redis
Using Redis in a Simple Rust web service

### Usage:
add to block list:
`
curl -d "hell.com" -X POST 127.0.0.1:1337/block
`

check block list:
`
curl -d "hell.com" -X POST 127.0.0.1:1337/check
`