# Rust Redis Clone

This is my implementation of the following Codecrafters challenge:
["Build Your Own Redis" Challenge](https://codecrafters.io/challenges/redis).

I know, sounds like a copy-paste guide, but it's actually a pretty neat challenge without much guidance.

## How to run the tests
```
cargo test
```

## How to run the server
```
cargo run
```

After that, server will be running on `localhost:6379`, you can use redis-cli to connect to it. It supports the 
most essential commands like `PING`, `SET`, `GET` with passive TTL, in the future I will add more.

Also note that server is using memory storage, so all data will be lost after server shutdown.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.
