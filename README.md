# SMSCMD - SMS on the Command Line
A rust application to send SMS via the command line

## Usage
```bash
smscmd <phonenumber> "message content"
```

## Description
- __smscmd__ is a rust application to send SMS via the command line.
- It opens raw sockets and connects to [Tiara Connect](https://tiaraconnect.io) to send the SMS.
- It uses the A2P model - an SMS is sent from the application to the mobile subscriber.

## High Level Architecture
![High Level Architecture](public/images/smscmd-high-level-architecture.png)
- The client application(smscmd) connects to [Tiara Connect](https://tiaraconnect.io) via __SMPP__.
- Tiara Connect connects to the telco and submits the short message.
- The telco will then deliver the short message to the mobile handset.
- Just 2 hops and the message is there - and the good news: _Binary protocols only!_

## System Flow
- The client will establish a bind with Tiara Connect
- Through the established bind, the client will attempt to submit a short message
- The connection configuration shall reside in __config.properties__ file.