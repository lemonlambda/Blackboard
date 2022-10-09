# Blackboard
A toml based build tool for C

## Installation
Install by running
```
cargo install --git https://github.com/Chalk-Organization/Blackboard.git
```

## Setup
Create `blackboard.toml` in your C dir and put
```toml
[package]
name = "example"
version = "1.0.0"
```
See [filled out toml](./examples/example_proj/blackboard.toml)

## Running
Make sure you have `~/.cargo/bin` on path if you're on linux.
Now you can just run
```
blackboard
```
