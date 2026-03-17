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

# SMPP(Short Message Peer-to-Peer Protocol)
## Protocol versions
- 5.0 -> 2003
- 3.4 -> 1999
- 4.0 -> 1997
- 1.0 - 1.3 -> 1994 - 1997
- 1.0 - 3.3 -> 1991 - 1997


## Sessions
### Forms of ESME-initiated sessions
- TX
- RX
- TRX

MC can establish a SMPP session by connecting to the ESME - **outbind session**


## Operations and PDUs
- basically a set of operations each taking form of **request-response** PDU(protocol data unit)
- operations are broadly categorized into following groups
    - session management
    - message submission
    - message delivery
    - message broadcast
    - ancillary operations

### Session Management Operations
- bind_transmitter
    - auth PDU used by a transmitter ESME to bind to the MC

- bind_transmitter_resp
    - MC response to EMSE bind_transmitter. indicates success or failure

- bind_receiver
    - auth PDU used by a receiver ESME to bind the MC

- bind_receiver_resp
    - response to bind_receiver

- bind_transceiver
    - auth PDU used by receiver/transmitter ESME to bind the MC

- bind_transceiver_resp
    - response to bind_transceiver

- outbind
    - auth PDU by MC to outbind to an ESME to inform it that messages are present in the MC
    - ESME would respond with bind_receiver or bind_transceiver to begin process of binding into the MC

- unbind
    - sent by ESME or MC as a means of terminating the session

- unbind_resp
    - response to unbind
    - comes from either MC or ESME

- enquire_link
    - sent by ESME or MC to test the network connection
    - recipient expected to acknowledge the PDU as a means of verifying the test

- alert_notification
    - MC sends this to ESME as a means of alerting it to the availability of an ESME

- general_nack
    - sent by ESME or MC as a means of indicating the receipt of an invalid PDU
    - receipt of general_nack indicates that the remote peer either cannot identify the PDU or has deemed it an invalid PDU due to its size or content

### Message Subsmission Operations
- submit_sm
    - transmitter or transceiver ESME wishing to submit a short message can use this PDU to specify the sender, receiver and text of the short message. other attributes include
        - message priority
        - data coding scheme
        - validity period etc

- submit_sm_resp
    - MC response to a submit_sm PDU indicating the success or failure of the request.
    - also includes message_id that can be used in subsequent operations to query, cancel or repace the message in the MC

- submit_multi
    - variation of submit_sm PDU that supports up to 255 recipients of the given message

- submit_multi_resp
    - MC response to submit_multi PDU
    - diff is that if there were failures, the PDU can specify  the list of failed recipients appending a specific error code for each one.
    - also includes MC message_id that can be used in subsequent operations to query, cancel or replace the contents of an undelivered message

- data_sm
    - streamlined version of the submit_sm operation designed for packet-based applications that do not demand extended functionality that is available to submit_sm
    - EMEs implementing WAP over a SMS bearer use this

- data_sm_resp
    - MC response to data_sm PDU. also includes a MC message_id

### Message Delivery Operations
- deliver_sm
    - symmetric opposite of submit_sm - used by MC to deliver a message to a receiver or transceiver ESME

- deliver_sm_resp
    - indicates ESME's acceptance or rejection of the delivered message.
    - error returned by ESME can cause the message to be retried at a later date or rejected there and then

- data_sm
    - used for message delivery form MC to the ESME
    - ESMEs implementing WAP over SMS use this

- data_sm_resp
    - response from ESME to MC for data_sm PDU

### Message Broadcast Operations
- broadcast_sm
    - A broadcast ESME wishing to broadcast a short message can use this PDU to specify the alias, geographical areas and text of the short message

- broadcast_sm_resp
    - MC response to a broadcast_sm PDU indicating the success or failure of the request
    - included a MC message_id that can be used for subsequent operations

### Anciliary Submission Operations
- cancel_sm
    - used to cancel a previously submitted message
    - PDU contains the source address of the original message and the message_id returned from the original submit_sm_resp, submit_multi_resp or data_sm_resp PDU
    - May omit the message_id and instead contain a source address, destination address and optional service type field as a means of cancelling a range of messages sent from one address to another

- cancel_sm_resp
    - MC returns this PDU to indicate success or failure of cancel_sm PDU

- query_sm
    - used to query the state of a previously submitted. contains the source address of the original message and the message_id returned in either of submit_sm_resp, submit_multi_resp pr data_sm_resp PDU

- query_sm_resp
    - MC returns a query_sm_resp PDU as a means of indicating the result of a message query attempt.
    - if successful, it will include the current state of the message

- replace_sm
    - used by ESME to pass a message_id of a previously submitted message along wth several other fields used to update the text, validity period and other attributes of the message

- replace_sm_resp
    - indicates success or failure of a replace_sm PDU

### Anciliary Broadcast Operations
- cancel_broadcast_sm
    - used to cancel the state of a previously broadcast message

- cancel_broadcast_sm_resp
    - response to cancel_broadcast_sm PDU

- query_broadcast_sm
    - used to query the state of a previously broadcast message
    - contains src address, message_id returned in the original broadcast_sm_resp PDU

- query_broadcast_sm_resp
    - response to query_broadcast_sm
    - indicates the success or failure of the attempt and for successful ones returns the state of the message

# SMPP Sessions
- Application layer protocol
    - physical - basic eletronic data communications
    - link level - transmission of ocrers
    - network - fully formed network packets
    - transport - manages packets and the reliable transport of data

## Establishing SMPP session
- first requires the ESME to connect to the MC
- achieved using TCP/IP or X.25 connection
- MC will be listening for connections on one or more TCP/IP ports or X.25 interfaces
- IANA has standardized port 2775 for SMPP

## Session States
- once a connection is established, SMPP refers to that connection as a session
- it can have several states

### Open
- EMSE has established a connection to the MC but has not yet issues a bind request
- MC is only aware of the TCP connection

### Bound_TX
- connected ESME has requested to bind as a transmitter - bind_transmitter PDU and has received a bind_transmitter_resp authorizing request.
- such an ESME may send short messages to a MC for onward delivery to a MS or to another ESME
- it may also replace, query or cancel a previously submitted short message

### Bound_RX
- connected ESME has requested to bind as a receiver - bind_receiver PDU and has received a bind_receiver_resp authorizing the request
- such an ESME may receive short messages from MC which may be originated by a mobile station, by another ESME or by the MC itself

### Bound_TRX
- connected ESME has requested to bind as a transceiver - bind_transceiver PDU and has received a bind_transceiver_resp authorizing the request
- such an ESME may do what both RX and TX can do

### Unbound
- ESME bound as a TX, RX or TRX has issued an unbind request to the MC requesting termination of the SMPP session.
- MC may also issue an unbind request to the ESME
- receiving peer responds with unbind_resp acknowledging the request to end the session

### Closed
- ESME or MC has closed the network connection. This is a follow up to Unbound state

### Outbound
- purpose: allow MC initiate a SMPP session
- could happen if the MC had outstanding messages for delivery to the ESME

## PDU Sequencing
- so far it looks like SMPP is a handshake protocol where each request is first acknowledged before issuing the next request - this is however not the case.

### PDU Sequence Number
- each SMPP request PDU has an identifier called a sequence number that is used to uniquely identify the PDU in the context of its originating entity and the current SMPP session
- resulting response PDU which must be retuned on the same SMPP session is expected to mirror the sequence number of the original request
- each SMPP session involves 2 sets of sequence numbers, those coming from ESME and those coming from MC

### Why use Monotinically Increasing Sequence Numbers
- easy to implement and handle
- provide a running operation count for the session - seq number 100 identifies the 100th PDU issued within the session
- other approaches
    - random numbers
    - using a selected pool of numbers and reusing whenever a response has been given
- in any case, the responding peer is expected to honour those values and use the same seq number in the response PDU

### Sequence Numbers Across Sessions
- designed for use within a single session
- recommended approach is to begin each session with sequence number of 1 and increase monotonically from that point
- the number will not affect any sessions that may already exist between the ESME and MC - PDUs fro one session cannot be acknowledged through another
- if the ESME-MC connection is closed or lost - then recovery would be:
    - ESME or MC re-establish the session
    - All unacknowledged PDUs from the lost session will not be acknowledged in the new recovery session i.e if the session was lost at a point where the MC was yet to send a submit_sm_resp to the ESME then a new session established between the ESME and MC will not result in this response PDU being returned

### Sync vs Async
- SMPP is an async protocol
- means that an EMSE or MC can send several requests at a time to the other party.
- PDU sequence number plays a crucial rple in this
- note that in such a case, the MC may not necessarily respond to the PDUs in the order they were sent from the ESME
- MC must support the ability to process response PDUs in non-contiguous order
- PDU sequence number make the request/response matching possible - means that each PDU should have a unique sequence number within the context of the session.

### Why Async
- avoids queues

## Session Timers
- SMPP operations are based on the exchange of operation PDUs between ESME and MC
- in order to control the amount of time spent waiting for a response to arrive or particular operation to occur, the following timers are defined
    - Session Init Timer
        - specifies the time lapse allowed between a network connection being established by an ESME and a bind_transmitter, bind_receiver or  bind_transceiver request being sent to the MC

        - can also be used by a MC supporting outbind and applied at the time interval between the outbind request being sent to an ESME and its response with a bind request

    - Enquire Link Timer
        - specifies the time lapse between operations after which a SMPP entity should interrogate whether its peer still has an active session
        - may be active on either peer

    - Inactivity Timer
        - specifies the maximum time lapse allowed between transactions after which period of inactivity a SMPP entity may assume that the session is no longer active
        - resulting behaviour is to either issue an unbind request or close the session

    - Response Timer
        - specifies the time lapse allowed between a SMPP request and a corresponding SMPP response
        - may be activity on either entities

## Error Handling
### Handling Connection Failure
- EMSE or MC may experience a failed connection attempt or may suddenly loose connection to a peer. reasons may include
    - IP address and port details are incorrect
    - remote MC or ESME is down or unable to accept the connection
    - network between the 2 hosts is down

- recommended approach is to continually trybto connect or reconnect again at intervals.
- most SMPP sessions will be ESME-initiated but we may have outbind. in this case the MC can itself be in a position to connect to an ESME that is configured for outbind

### Operation Failure
- possible reasons why MC/ESME may reject an operation request PDU
    - The PDU is unrecognised
        - typical response in this case is general_nack PDU with the sequence number of the offending PDU
        - command status is usually ESME_RINVCMDID which indicated that the EMSE or MC cannot recognize the PDU
    - The PDU is malformed
        - sending entity is at fault for sending non standard PDU
        - typical responses will depend on how the malformed PDU is detected.
        - if the command_id is the reason for rejection, the receiving peer should respond with general_nack and command status set to ESME_RINVCMDID
        - if the command_length of the PDU appeared too large, the ESME or MC should respond with general_nack and command status set to ESME_RINVCMDLEN
    - Invalid Field Length
        - if any PDU field is too long or too short then it is malformed but an ESME or MC may indeed recognize the PDU and as such will respond with a submit_sm_resp or whatever is the appropriate error
        - e.g if an ESME submits a message with a 20 character scheduled delivery time, the rejection should be a command_status set to ESME_RINVSCHED
    - PDU data is unexpected and deemed invalid
        - application compliance issue
        - typical error code would be ESME_RX_R_APPN
    - PDU is not allowed in the current session state
        - violation of the rules of the SMPP sessions e.g an ESME in Bound_RX state attempting to submit a message by sending a submit_sm PDU
        - expected command status is ESME_EINVBNDSTS
    - ESME or MC is restricting the use of certain PDUs or features SMPP has a broad scope of functionality and some Message Centres or ESMEs may deliberately provide mechanisms to disable certain features.
    - For example if an operator configured a message centre to reject attempts by ESMEs to request delivery receipts for messages, it would force the MC to reject the message with a command status set to ESME_RINVREGDLVFLG.
    - Although the field may be correctly encoded, its usage is disabled and the MC is authorised to reject the message using that error code.

## Flow Control and Congestion Avoidance
- common misconception that windowing provides full flow control
- all that is gained with windowing is a finite limit to an async window
- a maxed out window would still prevent the originator from issuing more requests  until responses arrive - this is however not flow control
- flow control relates to the concept of a receiver informing the sender that it can't accept more data
- in TCP, this concept is supported by a 'receiver buffer advertisement' which can be passed with every packet ACK. the sender uses this data as a means of judging how much more data can be sent in subsequent transmissions. this works for TCP given that congestion is based mostly on volumes of data being sent across busy networks or a to a congested receiver
- in SMPP, if an ESME or MC submits or delivers messages at a rate that exceeds the capabilities of its peer, congestion may occur. relying on windowing to solve the problem is not enough. the ESME will continue to top up its window of unACKed requests keeping the MC under load to process these requests.
- to better assist a peer in avoiding congestion,the peer needs a mechanism to provide the receiving peer with an indication of its state of congestion
- this is accomplished with the addition of an optional __congestion_state__ TLV. this parameter may be optionally included in any response PDU sent between an ESME and MC, it contains a simple integer 0-100 to indicate the congestion state ranging from idle to congested.
- the ESME or MC can use this value to detect increased load scenarios and reduce input rates. the MC or EMSE should try to maintain the congestion state between 80-90

## Session Security and Encryption
- SMPP doesn't define any native encryption mechanisms - the content exchanged is open to unauthorized interception
- to combat this risk, there are 2 recommended approaches

### 1. Leased Lines
- avoid publicly accessible media such as the internet and the risk of security breach is reduced.

### 2. Secure Transport Layer(SSL)
- establish a secure connection before commencing the SMPP session

### 3. Secure VPN

### 4. Secure Tunnel


# SMPP Parameter and PDU Format
## Parameter Type Definitions
1. integer
    - unsigned integer
    - can be 1,2 or 4 octets
    - encoded in Most Significant Bit - Big Endian

1. C-octet string
    - sequence of ASCII characters terminated with a NULL octet(0x00)
    - 'hello' = 6 octets - 5 + 1 null
    - two variants
        - decimal
        - hexadecimal

1. Octet string
    - sequence of octets not necessarily terminated with a NULL octet
    - can be used to encode raw binary data
    - either
        - fixed length
        - explicit length - another field indicates the length of the octet string field

1. Tagged Length Value(TLV)
    - special composite field with 3 parts
        - Tag - 2 octet integer(0-65535)
            - identifies the parameter

        - Length - 2 octet integer(0-65535)
            - indicates the length of the value field in octets
            - does not include the length of the tag and length fields

        - Value
            - actual data for the TLV field

    - examples
        - 0x0007000104
            - 0007 - tag = 7
            - 0001 - length = 1 - in this case we expect 1 octet(byte) in the value field - 04 for this matter
            - 04 - value = 4 - as defined in length above(1 octet)

### NULL Settings
- means that a field is not carrying any value
- must still be encoded in the PDU

1. Integer
    - 1 octet = 0x00
    - 2 octets = 0x0000
    - 4 octets = 0x00000000

2. C-octet String - 0x00
3. Octet String - not encoded. Explicit length field that indicates length should be set to 0
4. TLV
    - 2 types:
        - may not carry a value part
            - when the value is NULL - only tag and length are encoded
                - length will be set to 0
        - tlv not required at all
            - it is not encoded at all

## General PDU Format
### PDU Format
#### Command_length: 4 octets(int)
- represents actual size of the PDU including PDU header and body
- note that command_length is included
- SMPP is a binary protocol and also supports async transmission
- this means that MC or ESME must support the ability to decode a PDU from a network connection buffer that may contain several PDUs
- key to achieving the means of decoding each PDU within a buffer is based on the knowledge of how big each PDU is.
- command_length represents the first field of a PDU and its value contains the overall size of the PDU
- an application can easily decode a PDU from a buffer by extracting the firest  octets, assuming this to represent the command_length then deduce from the value the overall size of the PDU
- get the length from the command_length field and deduct 4 then proceed to read the remaining bytes from the buffer
- an alternative is to wait for 16 octets(PDU header) - an application wishing to decode a PDU for processing must wait until there are atleast 16 octets available in the network connection buffer or continually read octets of data untol 16 octets have been read
- having read the 16 octets, by subtracting 16 from the command_length value, the application can evaluate the size of the PDU body and use the same means of reading data from its buffers until the remaining data has been received.

#### Command_id: 4 octets(int)
- identifies the SMPP operation e.g submit_sm, bind_transmitter
- command ids for request PDUs allocated from a range of numbers between 0x00000000 to 0x000001FF
- command ids for response PDUs allocated from a range of numbers between 0x80000000 to 0x800001FF
- difference between request and response for the same command is that bit 31 is cleared for the request and set for response e.g
    - replace_sm = 0x00000007
    - replace_sm_resp = 0x80000007

#### Command_status: 4 octets(int)
- represents the means by which an ESME or MC sends an error code to its peer
- only relevant in response PDUs
- PDU requests are set to NULL(0x00000000)
- when a response PDU carries a non-NULL command_status field, it is indicating some form or error or rejection of the original request PDU. in such a circumstances, a PDU body should not be included in the PDU and the command_length of the PDU should therefore be set to 16 - 0x00000010(pdu header only)
- however, some ESMEs pr MC may always include a PDU body regardless of the command_status being returned - the peer should ignore the contents based on the knowledge that the request failed.

#### Sequence_number: 4 octets(int)
- represents a means of uniquely identifying each PDU within an SMPP session
- provides a means of correlating request and response PDUs based on matching sequence numbers

#### Standard Parameters
- combinates of:
    - integer
    - c-octet string
    - octet string


#### TLV Parameters

### A Sample PDU
```
00 00 00 2F 00 00 00 02 00 00 00 00 00 00 00 01
53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
30 3B 00 53 55 42 4D 49 54 31 00 50 01 01 00
```
header - first row
```
00 00 00 2F -> command length = 47
00 00 00 02 -> command id = 2(bind_transmitter)
00 00 00 00 -> command status = 0(request PDU default to 0)
00 00 00 01 -> sequence number = 1
```

pdu body - sepecifically for bind_transmitter request
```
53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
30 3B 00 53 55 42 4D 49 54 31 00 50 01 01 00
```


# SMPP PDU Definitions
## Session Management Operations
### bind_transmitter/bind_receiver/bind_transceiver
- header
    - command_id = 0x00000002(transmitter);
    - command_id = 0x00000001(receiver);
    - command_id = 0x00000009(transceiver);
- system_id
    - length: max of 16
    - type: c-octet string
    - description - identifies the ESME system requesting to bind with MC
- password
    - length: max of 9
    - type: c-octet string
    - description: used by MC to authenticate the ESME requesting to bind
- system_type*
    - length: max of 13
    - type: c-octet string
    - description: identifies the type of ESME system requesting to bind  MC
- interface_version
    - length: 1
    - type: Integer
    - description: indicates the version of the SMPP protocol supported by the ESME
- addr_ton*
    - length: 1
    - type: Integer
    - description: indicates the type of number of the ESME address, if not known set to NULL(0x00)
- addr_npi*
    - length: 1
    - type: Integer
    - description: numbering plan indicator for EMSE address, if not known set to NULL(0x00)
- address_range*
    - length: max of 41
    - type: c-octet string
    - description: ESME address - if not known set to NULL

### bind_transmitter_resp/bind_receiver_resp/bind_transceiver_resp
- header
    - command_id = 0x80000002(bind_transmitter_resp);
    - command_id = 0x80000001(bind_receiver_resp);
    - command_id = 0x80000009(bind_transceiver_resp);
- system_id
    - length: max of 16
    - type: c-octet string
    - description: identifies the MC to the ESME
- optional TLVs:
    - sc_interface_version
        - type: TLV
        - description: SMPP version supported by the MC

### outbind
- header
    - command_id = 0x0000000B;
- system_id
    - length: max of 16
    - type: c-octet string
    - description: identifies the MC to the ESME
- password
    - length: max of 9
    - type: c-octet string
    - description: authenticate MC originating the outbind

### unbind
- header
    - command_id = 0x00000006;

### unbind_resp
- header
    - command_id = 0x80000006;

### enquire_link
- header
    - command_id = 0x00000015;

### enquire_link_resp
- header
    - command_id = 0x80000015;

### alert_notification
- header
    - command_id = 0x00000102;
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator
- source_addr
    - length: max of 65
    - type: c-octet string
    - description: address of alert SME
- esme_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for ESME address which requested the alert
- esme_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for ESME address which requested the alert
- esme_addr
    - length: max of 65
    - type: c-octet string
    - description: address of ESME which requested the alert
- optional TLVs
    - ms_availability_status
        - type: TLV
        - description: status of the mobile station

### general_nack
- header
    - command_id: 0x80000000

## Message Submission Operations
### submit_sm
- header
    - command_id = 0x00000004;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for source address
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: address of SME which originated this message, if unknown set to NULL
- dest_addr_ton
    - length: 1
    - type: Integer
    - description: type of number of destination
- dest_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- destination_addr
    - length: max of 21
    - type: c-octet string
    - description: destination address of this short message, for MTs this is the directory number of the recipient MS
- esm_class
    - length: 1
    - type: Integer
    - description: indicates message mode and message type
- protocol_id
    - length: 1
    - type: Integer
    - description: protocol identifier
- priority_flag
    - length: 1
    - type: Integer
    - description: designates priority level of the message
- schedule_delivery_time
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL for immediate message delivery
- validity_period
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL to request the MC default validity period. superseded by the qos_time_to_live TLV if specified
- registered_delivery
    - length: 1
    - type: Integer
    - description: indicator to signify if a MC DLR, manual ACK, delivery ACK or an intermediate notification is required
- replace_if_present_flag
    - length: 1
    - type: Integer
    - description: indicator if the submitted message should replace an existing message
- data_coding
    - length: 1
    - type: Integer
    - description: defines encoding scheme of the short message user data
- sm_default_msg_id
    - length: 1
    - type: Integer
    - description: indicates the short message to send from a list of pre-defined short messages stored on the MC, if not using a MC canned message set to NULL
- sm_length
    - length: 1
    - type: Integer
    - description: length in octets of the short_message user data
- short_message
    - length: 0-255
    - type: octet-string
    - description: up to 255 octets of short message user data. usually superceded by the message_payload TLV if specified
- Message Submission TLVs
    - length: var
    - type: TLV

### submit_sm_resp
- headers
    - command_id = 0x80000004;
- message_id
    - length: max 65
    - type: c-octet string
    - description: MC message ID of the submitted message
- Message Submission TLVs
    - length: var
    - type: TLV

### data_sm
- header
    - command_id = 0x00000103;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for source address
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: address of SME which originated this message, if unknown set to NULL
- dest_addr_ton
    - length: 1
    - type: Integer
    - description: type of number of destination
- dest_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- destination_addr
    - length: max of 21
    - type: c-octet string
    - description: destination address of this short message, for MTs this is the directory number of the recipient MS
- esm_class
    - length: 1
    - type: Integer
    - description: indicates message mode and message type
- registered_delivery
    - length: 1
    - type: Integer
    - description: indicator to signify if a MC DLR, manual ACK, delivery ACK or an intermediate notification is required
- data_coding
    - length: 1
    - type: Integer
    - description: defines encoding scheme of the short message user data
- Message Submission TLVs
    - length: var
    - type: TLV

### data_sm_resp
- headers
    - command_id = 0x80000103;
- message_id
    - length: max 65
    - type: c-octet string
    - description: MC message ID of the submitted message
- Message Submission TLVs
    - length: var
    - type: TLV

### submit_multi
- headers
    - command_id - 0x00000021;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for source address
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: address of SME which originated this message, if unknown set to NULL
- number_of_dests
    - length: 1
    - type: Integer
    - description: indicates number of destinations that are to follow, max of 255 allowed
- dest_address(SME Format Destination Address)
    - length: max of 24
    - type: composite field
        - spec:
            - dest_flag
                - length: 1
                - type: Integer
                - description: 0x01(SME addresss)
            - dest_addr_ton
                - length: 1
                - type: Integer
                - description: type of number for destination
            - dest_addr_npi
                - length:  1
                - type: Integer
                - description: NPI for destination
            - destination_addr
                - length: max of 21
                - type: c-octet string
                - description: destination address of this short message
- dest_address(Distributed List Format Destination Address)
    - length: max of 23
    - type: composite field
    - spec:
        - dest_flag
            - length: 1
            - type: Integer
            - description: 0x02(Distribution List)
        - dl_name
            - length: max of 21
            - type: c-octet string
            - description: name of distribution list
- esm_class
    - length: 1
    - type: Integer
    - description: indicates message mode and message type
- protocol_id
    - length: 1
    - type: Integer
    - description: protocol identifier
- priority_flag
    - length: 1
    - type: Integer
    - description: designates priority level of the message
- schedule_delivery_time
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL for immediate message delivery
- validity_period
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL to request the MC default validity period. superseded by the qos_time_to_live TLV if specified
- registered_delivery
    - length: 1
    - type: Integer
    - description: indicator to signify if a MC DLR, manual ACK, delivery ACK or an intermediate notification is required
- replace_if_present_flag
    - length: 1
    - type: Integer
    - description: indicator if the submitted message should replace an existing message
- data_coding
    - length: 1
    - type: Integer
    - description: defines encoding scheme of the short message user data
- sm_default_msg_id
    - length: 1
    - type: Integer
    - description: indicates the short message to send from a list of pre-defined short messages stored on the MC, if not using a MC canned message set to NULL
- sm_length
    - length: 1
    - type: Integer
    - description: length in octets of the short_message user data
- short_message
    - length: 0-255
    - type: octet-string
    - description: up to 255 octets of short message user data. usually superceded by the message_payload TLV if specified
- Message Submission TLVs
    - length: var
    - type: TLV

### submit_multi_resp
- headers
    - command_id = 0x80000021;
- message_id
    - length: max of 65
    - type: c-octet string
    - description: MC message id
- no_unsuccess
    - length: 1
    - type: Integer
    - description: number of messages that were unsuccessfully submitted to the MC
- unsuccess_sme(unsuccessful SME)
    - length: max of 27
    - type: composite field
    - spec:
        - dest_addr_ton
            - length: 1
            - type: Integer
            - description: type of number for destination
        - dest_addr_npi
            - length: 1
            - type: Integer
            - description: NPI for destination
        - destination_addr
            - length: max of 21
            - type: c-octet string
            - description: destination address of SME
        - error_status_code
            - length: 4
            - type: Integer
            - description: indicates success/failure of the submit_multi request to this SME address
- Message Submission TLVs
    - length: var
    - type: TLV

## Message Submission Request TLVs
|Name|Description|
|---|---|
|alert_on_msg_delivery|request an MS alert signal  be invoked on message delivery|
|billing_identification|billing info passed from ESME to MC|
|callback_num|callback number associated with the short message|
|callback_num_atag|associates a displayable alphanumeric ta with the callback number|
|callback_num_pres_ind|defines the callback number presentation and screening|
|dest_addr_np_country|E.164 information to the operator country code|
|dest_addr_np_information|number portability information for the destination address|
|dest_addr_np_resolution|number portability query indicator|
|dest_addr_subunit|subcomponent in the destination device for which the user data is intended|
|dest_bearer_type|correct bearer type for delivering in the user data to the destination|
|dest_network_id|identification of destination network|
|dest_network_type|correct network for the destination service|
|dest_node_id|identification of destination node|
|dest_subaddress|sub-address of the message destination|
|dest_telematics_id|telematics identifier associated with the destination|
|dest_port|indicates app port number associated with the destination address of the message|
|display_time|provides the receiving MS with a display time associated with the message|
|its_reply_type|MS user's reply method to an SMS delivery message received from the network|
|its_session_info|session control info for interactive teleservice|
|language_indicator|indicates language of alphanumeric text message|
|message_payload|contains the extended short message user data. up to 64k octets can be transmitted|
|more_messages_to_send|indicates that there are more messages to send|
|ms_msg_wait_facilities|controls the inidcation and specifies the  message type at the MS|
|ms_validity|indicates the validity information for this message to the recipient MS|
|number_of_messages|indicates number of messages stored in a mailbox|
|payload_type|defines type of payload eg WDP, WCMP|
|privacy_indicator|indicates level of privacy associated with the message|
|qos_time_to_live|time to live as a relative time in seconds from submission|
|sar_msg_ref_num|ref number of a particular concatenated short message|
|sar_segment_seqnum|indicates the seq number of a particular short message fragment within the concatenated short message|
|sar_total_segments|indicates the total number of short message segments within the concatenated short message|
|set_dpf|indicator for setting delivery pending flag on delivery failure|
|sms_signal|indicates alerting mechanism when the message is received by an MS|
|source_addr_subunit|subcomponent in the destination device which created the user data|
|source_bearer_type|correct bearer type for delivering the user data to the destination|
|source_network_id|identification of source network|
|source_network_type|correct network associated with the originating device|
|source_node_id|identification of source node|
|source_port|app port number associated with the source address of the message|
|source_subaddress|sub-address of message originator|
|source_telematics_id|telematics identifier associated with the source|
|user_message_reference|ESME assigned message ref number|
|user_response_code|user response code|
|ussd_service_op|identify the required USSD service type|

## Message Submission Response TLVs
|Name|Description|
|---|---|
|additional_status_info_text|ASCII text giving a description of the meaning of the response|
|delivery_failure_reason|indicates reason for delivery failure|
|dpf_result|indicates whether Delivery Pending Flag was set|
|network_error_code|error code specific to a wireless network|

## Source and Destination Addressing
- submit_sm and data_sm include provision for both __`source address(message sender)`__ and __`destination address(message recipient)`__
- common concept of a source and destination address is a sequence of digits representing a mobile or fixed-line number
- it is however more complex:
- both addresses comprise of 3 parts
    - TON(Type of Number)
    - NPI(Numbering Plan Indicator)
    - Address(Digit Sequence)
- this is visible in the following fields
    - source_addr_ton
    - source_addr_npi
    - source_addr
    - ---
    - dest_addr_ton
    - dest_addr_npi
    - destination_addr

### TON(Type of Number)
#### International and National Format
- **international TON** means that the number starts with the country code followed by the national destination code(NDC) and the subscriber number
- NDC is also known as the operator code, operator prefix
- common for numbering plans to have multiple national destination codes
---
- **national TON** means that the number starts with the national destination code followed by the subscriber number - it is effectively the international number with the country code stripped off

#### Alphanumeric Format
- provides a means of using human-readable names for addresses
- in SMPP, an alphanumeric address can carry any digit 0-9 and alphabetical character a-z or A-Z

### Numbering Plan Indicator
- generally set to 1 by mobile devices
- purpose is to specify the numbering plan of the target device but because these generally tend to be mobiles, the value is generally set to 1

### ESME Addresses
- an ESME will typically use one of the following approaches
    - Service Short Code
        - e.g 20503
        - TON = 0
    - International Number
        - TON = 1
        - NPI = 1
    - NULL Address
        - in this case the MC may then substitute a default address for that particular ESME

## Message Replace operation in submit_sm
- though SMPP offers a dedicated **replace_sm operation** the **submit_sm** operation also facilitates replacement of a short message which has been previously submitted but has not yet been delivered to the designated destination.
- this feature is designed for applications needing the ability to update an undelivered message, a common application is a voicemail system
    + first message -you have 1 message in your mail box
    ---
    + if the mobile is unavailable the message will remain undelivered and in retry within the MC
    + if another message is left for the same subscriber:
    ---
    + second message: you have 2 messages in your mail box
    + if the replace_flag of the submit_sm PDU is set to 1, then this message should replace the undelivered first message - result is that the subscriber gets the latest message instead of all messages
- alternatively a MC administrator may define a specific service_type to provide replace-if-present functionality. in this case, the replace function can be activated in the submit_sm PDU by setting the service_type field to the defined value.

## Message Length
- each network variation is limited to some fixed max length
    - GSM
        - 140 octets(8 bits each)
        - GSM 7-bit - 160 characters
    - CDMA/TDMA
        - quite complex

## Message Types(field = registered_delivery)
### Registered
- allows an ESME to request a delivery receipt for the message
- under normal circumstances, a receipt is typically sent to the ESME when the message reached a final delivery state regardless of whether the message was actually delivered or not
- the registered_delubery field provides a number of settings that dictate the requirements for generating the receipt

### Scheduled
- a scheduled message is one that is not immediately dispatched for delivery to the destination SME
- instead, the scheduled date provided with the message dictates the time that the message will become eligible for delivery

### Pre-defined
- canned message that is provisioned on a MC
- ESME specifies the message by providing its ID in the sm_default_msg_id field

## Message Modes(field = esm_class)
- the esm_class field provides a message mode feature which if supported on the MC allows an ESME to select the MC message submission/delivery mechanism
- options:
    - default
        - typically store and forward
    - datagram
    - transaction
    - store and forward

### Default Message Mode
- default MC mode applies

### Store and Forward Mode
- store the message in a MC storage area eg message database before forwarding the message for delivery to the recipient SME
- the messages remains securely stored until the MC has made all delivery attempts or until the message expires
- with store and forward mode messages can be
    - cancelled - cancel_sm
    - queried - query_sm
    - replaced - replace_sm, submit_sm(replace mode)

### Datagram Message Mode
- emulates the datagram paradigm used in other data communication protocols such as UDP
- focuses on high message throughput without the associated secure storage and retry guarantee in store and forward message mode
- message originator i.e ESME does not receive any form of delivery ACK

### Transaction Mode
- allows EMSE to receive a form of delivery ACK that indicates if the message has been successfully or unsuccessfully delivered to the destination MS within the SMPP response PDU
- designed for applications that involve real-time messaging where an ESME requires a sync end-to-end delivery outcome without the need for long term MC storage

## Message Delivery Options
- provide means of delivering short messages from MC to an ESME - typically originate from an MS

### deliver_sm
- issued by the MC to send a message to an ESME
- header
    - command_id = 0x00000005;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for source address
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: address of SME which originated this message, if unknown set to NULL
- dest_addr_ton
    - length: 1
    - type: Integer
    - description: type of number of destination
- dest_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for destination address
- destination_addr
    - length: max of 21
    - type: c-octet string
    - description: destination address of this short message, for MTs this is the directory number of the recipient MS
- esm_class
    - length: 1
    - type: Integer
    - description: indicates message mode and message type
- protocol_id
    - length: 1
    - type: Integer
    - description: protocol identifier
- priority_flag
    - length: 1
    - type: Integer
    - description: designates priority level of the message
- schedule_delivery_time
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL for immediate message delivery
- validity_period
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL to request the MC default validity period. superseded by the qos_time_to_live TLV if specified
- registered_delivery
    - length: 1
    - type: Integer
    - description: indicator to signify if a MC DLR, manual ACK, delivery ACK or an intermediate notification is required
- replace_if_present_flag
    - length: 1
    - type: Integer
    - description: indicator if the submitted message should replace an existing message
- data_coding
    - length: 1
    - type: Integer
    - description: defines encoding scheme of the short message user data
- sm_default_msg_id
    - length: 1
    - type: Integer
    - description: indicates the short message to send from a list of pre-defined short messages stored on the MC, if not using a MC canned message set to NULL
- sm_length
    - length: 1
    - type: Integer
    - description: length in octets of the short_message user data
- short_message
    - length: 0-255
    - type: octet-string
    - description: up to 255 octets of short message user data. usually superceded by the message_payload TLV if specified
- Message Submission TLVs
    - length: var
    - type: TLV

### deliver_sm_resp
- headers
    - command_id = 0x80000005;
- message_id
    - length: max 65
    - type: c-octet string
    - description: unused and should be set to null
- Message Delivery Response TLVs
    - length: var
    - type: TLV

### data_sm
- symetrically used for delivery as it is used to submit messages

### Message Delivery Request TLVs
|name|desc|
|---|---|
|callback_num|callback number associated with the short message|
|callback_num_atag|associates a displayable alphanumeric tag with the callback number|
|callback_num_pres_ind|defines the callback number presentation and screening|
|dest_addr_np_country|E.164 info to the operator country code|
|dest_addr_np_information|number portability info for the destination address|
|dest_addr_np_resolution|number portability query indicator|
|dest_addr_subunit|subcomponent in the destination service for which the user data is intended|
|dest_network_id|identification of destination network|
|dest_node_id|identification of destination node|
|dest_subaddress|subaddress of the destination node|
|dest_port|indicates application port number associated with the destination address of the message|
|dpf_result|indicates whether the delivery pending flag was set|
|its_reply_type|controls the MS user's reply method to an SMS delivery message received from the network|
|its_session_info|session control info for interactive teleservice|
|language_indicator|indicates the language of the alphanum text message|
|message_payload|containes the extended short message user data - upto 64k octets can be transmitted|
|message_state|should be present for MC dlrs and intermediate notifications|
|network_error_code|may be present for dlrs and intermediate notifications|
|payload_type|defines the type of payload|
|privacy_indicator|indicates level of privacy associated with the message|
|receipted_message_id|MC message id of message being receipted|
|sar_msg_ref_num|ref number for particular concatenated short message|
|sar_segment_seqnum|ref number for a particular short message fragment within the concatenated short message|
|sar_total_segments|indicates the total number of short message segments within the concatenated short message|
|source_addr_subunit|subcomponent of the destination service which created the user dat|
|source_network_id|identification of source network|
|source_node_id|identification of source node|
|source_port|indicates app port number associated with the source address of the message. should be present for WAP applications|
|source_subaddress|subaddress of message originator|
|user_message_reference|ESME assigned message ref number|
|user_response_code|user response code|
|ussd_service_op|identifes the required USSD service type when interfacing to a USSD system|

### Message Delivery Response TLVs
|additional_status_info_text|ASCII text giving a description of the meaning of the response|
|delivery_failure_reason|indicates reason for delivery failure|
|network_error_code|error code specific to a wireless network|

### Delivery Message Types
- normal message
- MC delivery receipt
- intermediate notification
- SME user/manual ACK
- SME delivery ACK
- conversation abort
- MC-MC handover message

#### MC Delivery Receipt
- carries MC DLR
- the MC on detecting the final state of a registered message, would normally generate a new receipt message addressed to the originator of the message
- delivered through deliver_sm or data_sm operation
- relevant fields
    - source address(destination address of the original SM)
        - ton
        - npi
        - addr
    - destination address(source address of thne original SM)
        - ton
        - npi
        - addr
    - esm_class
        - bit 2 set to 1 to indicate its a DLR
    - message_state TLV
    - network_error_code TLV
    - receipted_message_id TLV

#### Intermediate Notification
- special form of message that the MC may send to an ESME for an MT message - provides an intermediate status of a message delivery attempt
- typical uses are to report the outcome of delivery attempts made during the message's retry lifetime within the MC
- relevant TLVs
    - message_state TLV
    - network_error_code TLV
    - receipted_message_id TLV

#### SME Delivery ACK
- indication from the recipient SME that the user has read the short message
- for an MS-based SME, an SME delivery ACK is sent when the MS user or MS application has read the message from the SMS storage unit
- for a fixed SME the circumstances in which an SME delivery ACK may be sent are beyond the scope of this doc

#### SME Manual/User ACK
- application generated reply message sent in response to an application request message

#### Conversation Abort
- unique to interactive teleservice defined by Korean CDMA carriers
- sent by a MS-based SME to indicate the unexpected termination of an interactive session
- may be carried in a deliver_sm or data_sm PDU

## Message Broadcast Operations
- provide cell broadcast services to ESMEs

### broadcast_sm
- issued by ESME to submit a message to the MC for broadcast to a specified geographical area or set of geographical areas

- header
    - command_id = 0x00000111;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- source_addr_ton
    - length: 1
    - type: Integer
    - description: type of number for source address
- source_addr_npi
    - length: 1
    - type: Integer
    - description: numbering plan indicator for source address
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: address of SME which originated this message, if unknown set to NULL
- message_id
    - length: max of 65
    - type: c-octet string
    - description: used when replacing a message previously submitted or broadcast
- priority_flag
    - length: 1
    - type: Integer
    - description: designates priority level of the message
- schedule_delivery_time
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL for immediate message delivery
- validity_period
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL to request the MC default validity period. superseded by the qos_time_to_live TLV if specified
- replace_if_present_flag
    - length: 1
    - type: Integer
    - description: indicator if the submitted message should replace an existing message
- data_coding
    - length: 1
    - type: Integer
    - description: defines encoding scheme of the short message user data
- sm_default_msg_id
    - length: 1
    - type: Integer
    - description: indicates the short message to send from a list of pre-defined short messages stored on the MC, if not using a MC canned message set to NULL
- broadcast_area_identifier
    - length: var
    - type: TLV
    - description: identifies the target broadcast area for the requested message broadcast
- broadcast_content_type
    - length: var
    - type: TLV
    - description: specifies content type of message
- broadcast_frequency_interval
    - length: var
    - type: TLV
    - description: indicates frequency interval at which the broadcasts of a message should be reperted
- broadcast request optional TLVs
    - length: var
    - type: TLV

### broadcast_sm_resp
- headers
    - command_id = 0x80000111;
- message_id
    - length: max 65
    - type: c-octet string
    - description: unused and should be set to null
- broadcast response optional TLVs
    - length: var
    - type: TLV


### message replacement with broadcast_sm
- broadcast_sm can be used to replace an existing message which has been previously submitted to MC
- **setting the replace_if_present to 1** activates this mode
- additionally, it is necessary to supply a value for the message_id param or provide the user_message_reference TLV

## Ancilliary Submission Operations
### cancel_sm
- issued by ESME to cancel one or more previously submitted short messages that are pending delivery
- command may specify a particular message to cancel or all messages matching a particular source, destination and service_type
- if message_id is set to the ID of the previously submitted message then provided the source address supplied by the ESME matches that of the stored message, that message will be cancelled.
- if message_id is NULL all outstanding undelivered messages with matching source and destination addresses and service_type if specified are cancelled.

- header
    - command_id = 0x00000008;
- service_type
    - length: max of 6
    - type: c-octet string
    - description: indicates SMS application service associated with the message, set to NULL for default MC settings
- message_id
    - length: max of 65
    - type: c-octet string
    - description: message ID of the message to be cancelled. set to NULL if cancelling a group of messages
- source_addr_ton
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr_npi
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: used for verification
- dest_addr_ton
    - length: 1
    - type: Integer
    - description: used for verification
- dest_addr_npi
    - length: 1
    - type: Integer
    - description: used for verification
- destination_addr
    - length: max of 21
    - type: c-octet string
    - description: used for verification

### cancel_sm_resp
- header
    - command_id = 0x80000008;

### query_sm
- used to query status of previously submitted SM

- header
    - command_id = 0x00000003;
- message_id
    - length: max of 65
    - type: c-octet string
    - description: message ID of the message to be cancelled. set to NULL if cancelling a group of messages
- source_addr_ton
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr_npi
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: used for verification

### cancel_sm_resp
- header
    - command_id = 0x80000003;
- message_id
    - length: max of 65
    - type: c-octet string
    - description: message ID of message being queried
- final_date
    - length: 1 or 17
    - type: c-octet string
    - description: date and time when the queried message reached a finall state. for messages that have not reached a final state yet, this field will contain a single NULL octet
- message_state
    - length: 1
    - type: integer
    - description: shows state of queried message
- error_code
    - length: 1
    - type: integer
    - description: holds a network error code defining the reason for failure of message delivery

### replace_sm
- replace a message that was submitted

- header
    - command_id = 0x00000007;
- message_id
    - length: max of 65
    - type: c-octet string
    - description: message ID of the message to be replaced. set to NULL if cancelling a group of messages
- source_addr_ton
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr_npi
    - length: 1
    - type: Integer
    - description: used for verification
- source_addr
    - length: max of 21
    - type: c-octet string
    - description: used for verification
- schedule_delivery_time
    - length: 1 or 17
    - type: c-octet string
    - description: set to NULL for immediate message delivery
- validity_period
    - length: 1 or 17
    - type: c-octet string
    - description: new expiry time for the short message. set to NULL to preserver original value
- registered_delivery
    - length: 1
    - type: Integer
    - description: indicator to signify if a MC DLR, manual ACK, delivery ACK or an intermediate notification is required
- sm_default_msg_id
    - length: 1
    - type: Integer
    - description: indicates the short message to send from a list of pre-defined short messages stored on the MC, if not using a MC canned message set to NULL
- sm_length
    - length: 1
    - type: Integer
    - description: length in octets of the short_message user data
- short_message
    - length: 0-255
    - type: octet-string
    - description: up to 255 octets of short message user data. usually superceded by the message_payload TLV if specified
- Message Replacement TLVs
    - length: var
    - type: TLV

### replace_sm_resp
- header
    - command_id = 0x80000007;

### message replacement TLVs
|name|size|type|description|
|---|---|---|---|
|message_payload|var|TLV|containes the extended short message user data. up to 64 octets can be transmitted|

## Ancilliary Broadcast Operations
> ignore for now

## PDU Field Definitions
### addr_ton, source_addr_ton, dest_addr_ton, esme_addr_ton
|TON|Value|
|---|---|
|Unknown|00000000|
|International|00000001|
|National|00000010|
|Network Specific|00000011|
|Subscriber Number|00000100|
|Alphanumeric|00000101|
|Abbreviated|00000110|
|Reserved||

### addr_npi, source_addr_npi, dest_addr_npi, esme_addr_npi
|TON|Value|
|---|---|
|Unknown|00000000|
|ISDN(E113/E164)|00000001|
|Data(X.121)|00000011|
|Telex(F.69)|00000100|
|Land Mobile(E.212)|00000110|
|National|00001000|
|Private|00001001|
|ERMES|00001010|
|Internet(IP)|00001110|
|WAP Client Id|00010010|
|Reserved||

### address_range
- used in bind_receiver and bind_transceiver command to specify a set of SME addresses serviced by the ESME client
- a single SME address may also be specified in the address_range param
- UNIX regex should be used to specify a range of addresses

#### UNIX Regex

- ^1234 - beginning with
- 5678$ - ending with
- ^1234567$ - absolute address
- [13569]$ - ending with any of 1,3,5,6,9

### command_id
|command|value|
|---|---|
|bind_receiver|0x00000001|
|bind_transmitter|0x00000002|
|query_sm|0x00000003|
|submit_sm|0x00000004|
|deliver_sm|0x00000005|
|unbind|0x00000006|
|replace_sm|0x00000007|
|cancel_sm|0x00000008|
|bind_transceiver|0x00000009|
|outbind|0x0000000B|
|enquire_link|0x00000015|
|submit_multi|0x00000021|
|alert_notification|0x00000102|
|data_sm|0x00000103|
|broadcast_sm|0x00000111|
|query_broadcast_sm|0x00000112|
|cancel_broadcast_sm|0x00000113|
|generic_nack|0x80000000|
|bind_receiver_resp|0x80000001|
|bind_transmitter_resp|0x80000002|
|query_sm_resp|0x80000003|
|submit_sm_resp|0x80000004|
|deliver_sm_resp|0x80000005|
|unbind_resp|0x80000006|
|replace_sm_resp|0x80000007|
|cancel_sm_resp|0x80000008|
|bind_transceiver_resp|0x80000009|
|enquire_link_resp|0x80000015|
|submit_multi_resp|0x80000021|
|data_sm_resp|0x80000103|
|broadcast_sm_resp|0x80000111|
|query_broadcast_sm_resp|0x80000112|
|cancel_broadcast_sm_resp|0x80000113|
|Reserved for MC Vendor|0x00010200 - 0x000102FF and 0x80010200 - 0x800102FF|
