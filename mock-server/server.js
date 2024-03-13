const express = require('express');
const bodyParser = require('body-parser'); 

const app = express();

app.use(bodyParser.json()); 
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', 'http://localhost:5173');
  res.header('Access-Control-Allow-Methods', 'GET,POST'); 
  res.header('Access-Control-Allow-Headers', 'Content-Type'); 
  next();
});

app.post('/', function(request, response) {
  console.log('POST /');
  console.dir(request.body);
  response.setHeader('Access-Control-Allow-Origin', 'http://localhost:5173');
  response.writeHead(200, {'Content-Type': 'text/html'});
  response.end('thanks');
});

const PORT = 8005;
app.listen(PORT, () => console.log(`listening on ${PORT}`));