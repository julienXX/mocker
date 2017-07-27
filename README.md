# mocker: an HTTP mock tool

## Usage
Configure a mock with a .toml file like this:
```
port = 1312
route = "/users"
response = '{"users": []}'
```
Run the mock server:
```
mocker file.toml
```

## TODO
- [ ] handle query params
- [ ] handle requests other than GET
- [ ] handle different content types
- [ ] define multiple routes in a config file
