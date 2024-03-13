const http = require("http");

const host = "localhost";
const port = 8005;

const server = http.createServer((req, res) => {
    res.statusCode = 200;
    res.setHeader('Content-Type', 'text/plain');
    res.end('Hello, World!');
  });
  
  server.listen(port, host, () => {
    console.log(`Server running at http://${host}:${port}/`);
  });