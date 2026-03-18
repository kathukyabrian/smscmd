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

## SMPP 
[link to docs](https://github.com/kathukyabrian/smpp-client/blob/main/docs/smpp.md)

## Key Words
- inspired by __nginx__
  - master process
  - single threaded worker processes
  - event driven 
  - non-blocking I/O model

### Master Process
- starts and manages worker processes
- handles things like:
  - reading config files
  - restarting workers
  - graceful shutdowns
- __it does NOT handle requests directly__

### Single Threaded Worker Processes
- each worker process is:
  - single threaded - it runs on one thread
  - can handle thousand of connections at once
- nginx mainly starts a thread per CPU core

### Event-Driven Architecture
- instead of
  - _wait for one request to finish before handling another_
- nginx:
  - _handle many requests by reacting to events_
- events include:
  - new connection
  - data received
  - response ready

### Blocking vs Non Blocking IO
#### Blocking IO
1. A request comes in
1. The server tries to read data
1. It waits until the operation finishes
2. Only when it finishes that it moves to the next task

- problem:
  - while waiting, the thread does nothing
  - waste of CPU resources
  - doesn't scale well
  
#### Non-Blocking IO
1. a thread starts  reading data
2. if data is not ready yet, it does not wait
3. it moves to handle other connections
4. when the data is ready -> it gets notified(event)
    - it listens for events

## Details of Event Driven Architecture
> the system does not sit and wait, __it reacts when something happens__

### What is an event
- low level things like:
  - a new client connects
  - a socket is ready to read data
  - data is ready to be written
  - a connection closes
  - a timeout occurs
  - __these are signals from the OS, not user level business events__

### How are events detected?
- you don't constantly check every connection
- instead, use OS-level mechanisms like:
  - epoll(linux)
  - kqueue(BSD/macOS)
  - select/poll(fallback)

### Example
1. client connects
    - OS notifies nginx "New Connection"
    - worker accepts it
2. wait for data
    - nginx registers: notify me when this socket has data
    - it does NOT wait
3. event triggered
    - OS says: this socket is ready to read
4. processing
    - maybe:
      - serve static file
      - forward to backend
        - if backend is slow, nginx moves on
5. response ready
    - OS says: socket ready to write
    - worker sends response
6. connection closed
   - clesn up resources

### State Machines
- each connection is NOT a blocking function
- instead it's like
```text
Connection State:
- waiting_for_request
- reading_request
- processing
- sending_response
- done
```

## Where does the request reside as it is processed?
- answer: memory(RAM) - mainly in buffers and connection structures

### Each Connection has its own memory
- when a worker accepts a request
  - it creates a connection object
  - inside it, nginx allocates:
    - request metadata(headers, methods, URI) -> state
    - buffers for incoming/outgoing data

### Buffers
#### Types of Buffers
- client request buffers
  - store incoming data
- response buffers
  - store outgoing data
- proxy buffers
  - if using upstream servers

### While Waiting:
- if data is incomplete:
  - nginx does not wait
  - keeps the partial data in memory buffers
  - moves on to other connections

### State + Data Together
- each request is basically
  - state machine
  - buffers

