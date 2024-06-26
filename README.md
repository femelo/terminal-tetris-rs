# Terminal Tetris

Play Tetris on a terminal emulator (Rust implementation).
Based on this [video](https://www.youtube.com/watch?v=8OK8_tHeCIA).

![terminal-tetris](https://github.com/femelo/terminal-tetris-rs/assets/28808345/7ce3ce3e-b733-4837-b348-a2cc53cd60a3)

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
