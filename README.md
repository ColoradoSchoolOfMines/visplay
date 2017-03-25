# Visplay
---

Visplay is a best described as a sort-of remote-control [X window manager](https://en.wikipedia.org/wiki/X_Window_System) + event scheduler. It is designed for presenting live content on a Linux machine without anyone needing to be physically present. This works regardless of what form the displayed content takes. It could be a sequence of websites, images, videos, shell scripts, GUI apps, sidebars, notifications, calenders, etc. If it can be opened in an X window, Visplay can present it. 

## Usage

Visplay is run on the presenting computer (the host) and controlled over a TCP stream by some external software application (the client). The control protocol is a back-and-forth stream of [protobuf](https://developers.google.com/protocol-buffers/) messages, defined in `protocol.proto`.

Remote (and optionally local) connections are authenticated using OpenSSL. The public key files of authorized clients (in PEM format) are listed in a JSON config file set by the `--config` parameter.

## Writing a Client

### Connecting
Connecting to and controlling a Visplay host requires the following steps:
1. Open a TCP stream to the host
2. Listen for a `HandshakeFromHost` message
3. Sign the data in the `urand` field
4. Send a `HandshakeFromClient` message with the signature in the `signature` field
5. Send `Command` messages, listening for `Response` messages after each

### Sequences, Triggers, and Tasks
[todo]