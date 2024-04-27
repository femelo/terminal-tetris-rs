# Terminal Tetris

Play Tetris on a terminal emulator (Rust implementation).
Based on this [video](https://www.youtube.com/watch?v=8OK8_tHeCIA).

https://github.com/femelo/terminal-tetris-rs/assets/28808345/e6ebf60c-8987-4b81-8dee-87abd143fc34


## Dependencies

- rust
- cargo
- cc = "1.0.95"
- libc = "0.2.153"
- ncurses = "6.0.0" (wrapper for libncurses)
- rand = "0.8.5"

## Install rust

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

```bash
rustup update
```

## Build

```bash
git clone https://github.com/femelo/terminal-tetris-rs.git
cd terminal-tetris-rs
```

```bash
cargo build --release
```

## Playing

```bash
./target/release/tetris
```

Commands:

- Left and right arrows to move pieces sideways
- Down arrow to accelerate piece descent
- Up arrow or space bar to rotate piece
