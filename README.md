## Use Docker to build and run

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # ssvmup build
(docker) # node node/app.js
```

From a second terminal window, you can test the local server.

```
$ curl http://localhost:3000/?encodeStr=HelloWorld
SGVsbG9Xb3JsZA==

$ curl http://localhost:3000/?decodeStr=SGVsbG9Xb3JsZA==
HelloWorld
```
