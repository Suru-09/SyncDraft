const express = require('express');
const { ExpressPeerServer } = require('peer');
var cors = require('cors')

const app = express();
app.use(express.json());
app.use(cors());
const port = 3600;

const sessions = [];

app.get('/', (req, res) => {
  res.send('Hello World!')
});


const createSession = (req, res) => {
  const session = {
    "_id": req.body._id,
    "session_owner": req.body.doc_owner,
    "users_list": [
        {
          "username": req.body.doc_owner,
          "webrtc_id": req.body.webrtc_id
        }
    ]
  };

  sessions.push(session);
  console.log(sessions);
  res.status(200).json({
    status: "success",
    message: "Created a new session"
  });
};;


const getUsersList = (req, res) => {
  res.status(200).json({
    "users_list": getSession(req.query._id).users_list
  });
};

const appendUserToSession = (req, res) => {
  let session = getSession(req.body._id);
  
  if (!session || session === undefined) {
    throw new Error(`Session with given ID(${sessionID}) does not exist`);
  }

  const isUserAlreadyConnected = session.users_list.find((user) => user.username == req.body.doc_owner);
  if (!isUserAlreadyConnected)
  {
    session.users_list.push({
      "username": req.body.doc_owner,
      "webrtc_id": req.body.webrtc_id
    });

    res.status(200).json({
      status: "success",
      message: "User appended to the session"
    });
  }
  else {
    res.status(501).json({
      status: "failure",
      message: "User is already connected to the session"
    });
  }
};


const removeUserFromSession = (req, res) => {
  let session = getSession(req.body._id);
  
  if (!session || session === undefined) {
    throw new Error(`Session with given ID(${sessionID}) does not exist`);
  }

  const isUserAlreadyConnected = session.users_list.find((user) => user.username == req.body.doc_owner);
  if (!isUserAlreadyConnected)
  {
    res.status(501).json({
      status: "failure",
      message: "User does not exist in the session..."
    });
  }
  else {
    session.users_list = session.users_list.filter((user) => user.username != req.body.doc_owner);
    res.status(200).json({
      status: "success",
      message: "User removed from the session"
    }); 
  }
  
};

const removeSession = (req, res) => {
  const sessionID = req.body._id;
  console.log(sessionID);

  const index = sessions.indexOf((session) => session._id == sessionID);
  const _ = sessions.splice(index, 1);
  res.status(200).json({
    status: "success",
    message: "Session has been removed"
  }); 
};

const getSession = (id) => {
  const sessionID = id;
  console.log(sessionID);
  const foundSession = sessions.find((session) => session._id == sessionID);
  console.log(foundSession);

  if (!foundSession || foundSession === undefined) {
    throw new Error(`Session with given ID(${sessionID}) does not exist`);
  }

  return foundSession;
};

app.post('/create-session', createSession);
app.get('/get-users-list', getUsersList);
app.post('/append-user-to-session', appendUserToSession);
app.post('/remove-user-from-session', removeUserFromSession);
app.post('/remove-session', removeSession);


const server = app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
});

const peerServer = ExpressPeerServer(server, {
	debug: true,
	path: "/myapp",
});

app.use("/peerjs", peerServer);