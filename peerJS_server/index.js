const express = require('express');
const { ExpressPeerServer } = require('peer');

const app = express();
const port = 3600;

app.get('/', (req, res) => {
  res.send('Hello World!')
});

const server = app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
});

const peerServer = ExpressPeerServer(server, {
	debug: true,
	path: "/myapp",
});

app.use("/peerjs", peerServer);