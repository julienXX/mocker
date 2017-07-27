# mocker: an HTTP mock tool

## Usage
```
mocker file.toml
```

## Configure a mock
Create a toml file like this:
```
port = 1312
route = "/users"
response = '{"users": []}'
```

## TODO
- [ ] handle query params
- [ ] handle requests other than GET
- [ ] handle different content types
- [ ] define multiple routes in a config file
