# Visplay

Visplay is a best described as a sort-of remote-control [X window manager](https://en.wikipedia.org/wiki/X_Window_System) + event scheduler. It is designed for presenting live content on a Linux machine without anyone needing to be physically present. This works regardless of what form the displayed content takes. It could be a sequence of websites, images, videos, shell scripts, GUI apps, sidebars, notifications, calenders, etc. If it can be opened in an X window, Visplay can present it. 

## Usage

Visplay is run on the presenting computer (the host) and controlled over a TCP stream by some external software application (the client). The control protocol is a back-and-forth stream of [protobuf](https://developers.google.com/protocol-buffers/) messages, defined in `protocol.proto`.

Remote (and optionally local) connections are authenticated using SHA256 signatures. The paths of PEM public key files are listed in the config.

## Configuration

Visplay is configured by passing a JSON file with `--config [file]`. A file containing the default values can be generated with `--export_defaults [file]`.

### Example:
```json
{
	"keys": [ "public_key.pem" ],
	"log_level": 1,
	"log_file": ".visplay_log.txt",
	"require_local_auth": true,
	"require_remote_auth": true
}
```

## Writing a Client

The host will send one `HandshakeFromHost` message and then many `Response` messages. The client will
send one `HandshakeFromClient` message and then many `Command` messages. **All messages are prefixed by an network endian unsigned 32-bit integer containing the size of the message in bytes.**

### Connecting
Connecting to and controlling a Visplay host requires the following steps:
1. Open a TCP stream to the host.
2. Listen for a `HandshakeFromHost` message.
3. Sign the data in the `tosign` field using SHA256. OpenSSL works well for this. If the field is missing, then authentication is not required.
4. Send a `HandshakeFromClient` message with the signature in the `signature` field (the field can be missing or blank if `tosign` was not given). On authentication failure, the host will terminate the stream and ignore all additional messages.
5. Send `Command` messages, listening for a single `Response` message after each.

### Sequences, Triggers, and Tasks
[todo]