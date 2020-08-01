const {
    base64Encode,
    base64Decode,
} = require('../pkg/ssvm_nodejs_starter_lib.js')

const http = require('http')
const url = require('url')
const hostname = '0.0.0.0'
const port = 3000

const server = http.createServer((req, res) => {
    const queryObject = url.parse(req.url, true).query
    if (queryObject['encodeStr']) {
        res.end(base64Encode(queryObject['encodeStr']) + '\n')
    } else if (queryObject['decodeStr']) {
        console.log(queryObject['decodeStr'])
        res.end(base64Decode(queryObject['decodeStr']) + '\n')
    } else {
        res.end(
            `Please use command curl http://${hostname}:${port}/?encodeStr=string
or
http://${hostname}:${port}/?decodeStr=string \n`
        )
    }
})

server.listen(port, hostname, () => {
    console.log(`Server running at http://${hostname}:${port}/`)
})
