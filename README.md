# SyncDraft

- A web application consisting in a collaborative text editor.

## Modules

- The main modules of our web application will be the server which will provide discovery in case of WebRTC and further bzl in case of the WebSockets approach. The second component will be the client which will have the communication specific modules and the text editor module.

## Features that we can add to increase complexity after initial implementation.

- fallback on server.
- user registration.
- documents persistancy based on user login.(SQL/NOSQL database).
- statistics through a Kafka broker shown in tools like Kibana/Grafana.

## Approaches

1. WebSockets -> OT(Operation Transform)
    - advantages
      - easier to implement initially.
    - disadvantages 
      - Big fat function hard to mathematically prove correcteness.(see papers)
      - only correct OT's require a main server.
      - the server can trace your data.
2. WebRTC -> CmRDT(Communtative Replicated Data Types)
    - advantages
      - given a reliable client you have data privacy.
      - lower ping.
      - easier to scale.
    - disadvantages
      - STUN & TURN servers for peer discovery & relay(in case of TURN) caused by clients NATs.

After considering our options we would like to implement the WebRTC version and provide the option to fallback on the server instead of a peer and document persistancy. Furthermore, as we value privacy we would like to offer the user the chance to use only peer to peer modes and persistancy to be completely turned off.