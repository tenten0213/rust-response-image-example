run

```shell
$ cargo run
```

get image and open it.
```shell
$ curl http://localhost:3000 -o hoge.jpeg
$ open hoge.jpeg
```

get json and base64 decode it
```shell
$ curl http://localhost:3000/json | jq -r '.image' | base64 -d > fuga.jpeg 
$ open fuga.jpeg
```
