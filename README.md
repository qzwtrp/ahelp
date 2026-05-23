# ahelp

> AI Linux terminal assistant — multi-provider (Gemini, OpenAI, Anthropic, OpenRouter).

`ahelp` (alias `jarvis`) reads your live system context (OS, kernel, shell, CPU,
RAM, disk, PWD) and sends it to your chosen AI provider. You get back concise,
copy-paste ready commands.

## Quick start

```bash
# clone & build
git clone https://github.com/qzwtrp/ahelp.git
cd ahelp
cargo build --release

# store keys (format: provider:key)
./target/release/ahelp --config-key gemini:AIzaSy... --default
./target/release/ahelp --config-key openai:sk-...
./target/release/ahelp --config-key anthropic:sk-ant-...
./target/release/ahelp --config-key openrouter:sk-or-...

# list configured providers
./target/release/ahelp --list-providers

# ask anything
./target/release/ahelp "how to find 20 largest files in current dir"
./target/release/ahelp --provider openai "explain iptables NAT"
./target/release/ahelp --info
```

## Usage

```
ahelp [OPTIONS] [QUERY]

Options:
  -p, --provider <NAME>     AI provider: gemini, openai, anthropic, openrouter
      --config-key <P:K>   Store API key (provider:key)
      --default            Mark key provider as default
      --info               Print system context and exit
      --list-providers     Show stored keys & default provider
  -h, --help               Print help
  -V, --version            Print version
```

## System context collected

- OS name & kernel version
- Architecture & host name
- Shell type, `$TERM`, `$EDITOR`
- Current directory (`PWD`)
- CPU cores / threads / usage %
- RAM total / used / %
- Disk (`/`) total / used / %

## Supported providers

| Provider   | Default model                         |
|------------|---------------------------------------|
| gemini     | gemini-2.5-flash                      |
| openai     | gpt-4o-mini                           |
| anthropic  | claude-3-opus-20240229                |
| openrouter | anthropic/claude-3.5-sonnet           |

## License

MIT
