# ahelp

> AI Linux terminal assistant — powered by Google Gemini.

`ahelp` (alias `jarvis`) reads your live system context (OS, kernel, shell,
CPU, RAM, disk, PWD) and sends it to Gemini together with your natural-language
query.  You get back concise, copy-paste ready commands.

## Quick start

```bash
# clone & build
git clone https://github.com/qzwtrp/ahelp.git
cd ahelp
cargo build --release

# store your API key once
./target/release/ahelp --config-key AIzaSy...

# ask anything
./target/release/ahelp "how to find 20 largest files in current dir"
./target/release/ahelp --info   # print system context only
```

## Usage

```
ahelp [OPTIONS] [QUERY]

Options:
  --config-key <KEY>  Store Gemini API key
  --info              Print system context and exit
  -h, --help          Print help
  -V, --version       Print version
```

## System context collected

- OS name & kernel version
- Architecture & host name
- Shell type, `$TERM`, `$EDITOR`
- Current directory (`PWD`)
- CPU cores / threads / usage %
- RAM total / used / %
- Disk (`/`) total / used / %

## License

MIT
