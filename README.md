# mocker: an HTTP mock tool

## Usage
Configure your mock service with a .toml file like this:
```
port = 1312
route = "/users"
response = '{"users": []}'
```
Serve the mock with the config you defined:
```
mocker file.toml
```

## TODO
- [ ] handle query params
- [ ] handle requests other than GET
- [ ] handle different content types
- [ ] define multiple routes in a config file
