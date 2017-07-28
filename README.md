# mocker: an HTTP mock tool

mocker can serve `.toml` files as fake HTTP responses.
The mocked response can be served on a specific port and route, with a specific content type and body.
The main benefit of the tool right now is the possibility to share those `.toml` files mocks.

## Usage
Configure your mock service by creating a `file.toml` file like this:
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

## Example: an InfluxDB mock.
```
port = 8086
route = "/*query"
content_type = 'application/json'
response = '''
{
    "results": [
        {
            "statement_id": 0,
            "series": [
                {
                    "name": "cpu_load_short",
                    "columns": [
                        "time",
                        "value"
                    ],
                    "values": [
                        [
                            "2015-01-29T21:55:43.702900257Z",
                            2
                        ],
                        [
                            "2015-01-29T21:55:43.702900257Z",
                            0.55
                        ],
                        [
                            "2015-06-11T20:46:02Z",
                            0.64
                        ]
                    ]
                }
            ]
        }
    ]
}'''
```
Let's try it!
```
λ curl -i 'http://localhost:8086/query?pretty=true' --data-urlencode "db=mydb" --data-urlencode "q=SELECT \"value\" FROM \"cpu_load_short\" WHERE \"region\"='us-west'"

HTTP/1.1 200 OK
Content-Length: 801
Content-Type: application/json
Date: Fri, 28 Jul 2017 09:56:40 GMT

{
    "results": [
        {
            "statement_id": 0,
            "series": [
                {
                    "name": "cpu_load_short",
                    "columns": [
                        "time",
                        "value"
                    ],
                    "values": [
                        [
                            "2015-01-29T21:55:43.702900257Z",
                            2
                        ],
                        [
                            "2015-01-29T21:55:43.702900257Z",
                            0.55
                        ],
                        [
                            "2015-06-11T20:46:02Z",
                            0.64
                        ]
                    ]
                }
            ]
        }
    ]
}
```
## TODO
- [x] handle different content types
- [x] handle requests other than GET
- [ ] handle query params
- [ ] define multiple routes in a config file
