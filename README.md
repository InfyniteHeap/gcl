# Grid Craft Launcher

> This project is under development at present.

Grid Craft Launcher (aka GCL) is a beautiful, fast and memory-safety Minecraft
launcher.

> [!IMPORTANT]
> This project uses [Tauri](https://tauri.app/) as its framework. This means
> that it doesn't ship with a Chromium core (contrary to what Electron does),
> and uses an existing WebView runtime instead. If your computer doesn't have a
> WebView runtime, you can't successfully run GCL.

## Features

- Fast, light-weight and memory safe!
- No historical burdens!
- Friendly to users!

## Build from Source

### Prerequisites

- **Rust**: This project is chiefly written in Rust. Its installation method can
  be found [here](https://www.rust-lang.org/tools/install/).
- **Deno**: This project uses Deno as its package manager. Its
  [official website](https://deno.com/) includes the commands that install it on
  your computer.

### Clone Repo

```bash
git clone https://github.com/InfyniteHeap/gcl.git
```

> [!TIP]
> If you only want to build this project, add `--depth 1` to avoid cloing whole
> commit histories.

### Install Packages

Enter in `gcl` folder, and run this command:

```bash
deno install
```

### Run Development Server

```bash
deno task tauri dev
```

### Build

```bash
deno task tauri build
```

## License

This software is distrubuted under GPL-3.0 license.
