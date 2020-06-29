# SPEEDTEST SERVER

The speedtest server is written in Go. Its basic goal in life is to
source data for download tests, and to sink data for upload tests.

## Performance

    Processor: Intel Xeon E5405 @ 2.00GHz

    Download 1000 Mbit/s: 10% CPU
    Upload   1000 Mbit/s:  75% CPU

## API

### Get client IP address

    Request:
    GET http://servername/speedtest/ip
    <no body>

    Response:
    {
      "remoteip": "1.2.3.4",
      "remoteport": "4456"
    }

### Downloadtest

    Connect to: ws://servername/speedtest/source

    Request:
    {
      "messageSize": 100000
    }

    Reply:
    continuous stream of binary messages: blob[messageSize]

The first 8 bytes of each binary message is a 64-bit integer in
BigEndian format, that gives the time (epoch undefined) that
the packet was sent in microseconds.

End the session by closing the websocket.

### Uploadtest

    Connect to: ws://servername/speedtest/sink

    Request:
    send binary websocket message with random data

    Reply:
    {
      timestamp: 66443,
      messsagesize: 100000
    }

CLient sends a stream of continuous websocket messages to the server.
For each message received, the server replies with the time
(epoch undefined) that the message was received in microseconds,
and the size of the message.

