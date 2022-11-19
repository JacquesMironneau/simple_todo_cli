# Simple todo cli app in rust

## Usage

```bash
todo --add "Buy milk" 3
todo --list
todo --urgent 3
```  

Alias for `--add` is `-a`, `--list` is `-l` and `--urgent` is `-u`.  

## Installation

```bash
cargo build --release
cp target/release/rust-todo-cli /usr/local/bin
```

## Next steps  

- [ ] Add tests
- [ ] Add delete one and delete all task (thus task will prob need an id)
- [ ] Add web server to serve the todo list
- [ ] Add web client 