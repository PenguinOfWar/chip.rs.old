# chip.rs
Video game emulation and hardware drivers.

## Development Libraries
To compile this project locally and check it out for yourself you will need to install the following libraries.

### [Rust](https://www.rust-lang.org)

```
% curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### [Homebrew](https://brew.sh)

```
% /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### SDL2 & Dependencies
SDL2 >= 2.0.8 is required.

```
% brew install sdl2
% brew install sdl2_ttf
% brew install sdl2_gfx
```

## Run Locally

Press `Esc` key to eject or `Ctrl + c` the terminal process.

```
% cargo run
```

## Compile Locally

```
% cargo build
```

If you get stuck, try turning it off and on again.

```
% cargo clean && cargo build
```

## Compile Release

```
% cargo build --release
```