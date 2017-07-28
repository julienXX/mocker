# mocker: an HTTP mock tool

## Usage
Configure your mock service with a .toml file like this:
```
port = 1312                               # default is 8080
route = "/users/*"                        # default is /
response = '{"id": 42, "name": "Karl"}'   # default is ""
content_type = 'application/json'         # default is 'application/json'
```
Serve the mock with the config you defined:
```
λ mocker file.toml

Starting mocker on 0.0.0.0:1312...
Active route is: '/users/*'
```
Test it:
```
λ curl -i "0.0.0.0:1312/users/3"
HTTP/1.1 200 OK
Content-Length: 26
Content-Type: application/json
Date: Fri, 28 Jul 2017 08:39:34 GMT

{"id": 42, "name": "Karl"}%
```
## TODO
- [x] handle different content types
- [ ] handle query params
- [ ] handle requests other than GET
- [ ] define multiple routes in a config file
