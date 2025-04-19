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

> [!NOTE]
> By default, Git will automatically convert LF line endings into CRLF when
> cloning repo onto Windows computer. Generally, this is no problem because it
> guarantees that Windows developers can work well with developers who are using
> other systems (see
> [this doc](https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration/)
> for more details).
>
> But since we use Deno to format our code, things have become a little
> different. Deno uses LF by default, whatever the platform you're using. When
> formatting files by using Deno on Windows, Deno will automatically convert
> CRLF into LF, and modifications on files will be captured by Git.
>
> Thus, this will lead to a result that, developers see Git listed these files
> when running `git diff`, but can't see any modifications in their contents.
> This might interfere developers when committing changes.
>
> To avoid such this interferences, we recommend Windows developers add
> `--config core.autocrlf=false` to avoid automatic line endings conversion when
> cloing repo.

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
