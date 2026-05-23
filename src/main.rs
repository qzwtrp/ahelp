use clap::Parser;
use colored::Colorize;
use std::io::{self, IsTerminal, Read};

mod config;
mod providers;
mod sysinfo;

use providers::ProviderName;

#[derive(Parser)]
#[command(name = "ahelp", about = "AI Linux assistant for the terminal", version = "0.2.0")]
struct Args {
    /// Your query
    query: Vec<String>,

    /// AI provider
    #[arg(short, long, value_name = "NAME")]
    provider: Option<String>,

    /// Set API key: PROVIDER:KEY
    #[arg(long, value_name = "PROVIDER:KEY")]
    config_key: Option<String>,

    /// Make this provider default when setting key
    #[arg(long, action)]
    default: bool,

    /// Print system context
    #[arg(long, action)]
    info: bool,

    /// List configured providers
    #[arg(long, action)]
    list_providers: bool,
}

fn print_logo() {
    let logo = r#"
    ┏━┓┏━┓╻ ╻┏━┓╻  ╏
    ┣━┫┣┳┛┃━┫┃ ┃┃  ╏
    ╹ ╹╹┗╸┗━┛┗━┛┗━╸╏
    AI Linux assistant · by qzwtrp"#;
    println!("{}", logo.cyan().bold());
}

fn parse_provider_key(s: &str) -> Option<(ProviderName, &str)> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 { return None; }
    let name = parts[0].parse::<ProviderName>().ok()?;
    Some((name, parts[1]))
}

fn main() {
    let args = Args::parse();

    if args.list_providers {
        println!("{}", config::list_keys());
        return;
    }

    if let Some(pair) = args.config_key {
        match parse_provider_key(&pair) {
            Some((provider, key)) => {
                config::set_key(provider, key, args.default);
                let action = if args.default {
                    format!("as default provider ({})", provider)
                } else {
                    format!("for {}", provider)
                };
                println!("{} API key saved {}{}{}",
                    "✓".green().bold(),
                    action,
                    " to ",
                    "~/.config/ahelp/config.toml".underline()
                );
            }
            None => {
                eprintln!("{} Invalid format. Use: ahelp --config-key gemini:AIzaSy...",
                    "Error:".red().bold());
                std::process::exit(1);
            }
        }
        return;
    }

    let ctx = sysinfo::collect();

    if args.info {
        println!("{}", sysinfo::format_context(&ctx));
        return;
    }

    let query: String = if io::stdin().is_terminal() {
        args.query.join(" ")
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).ok();
        let from_stdin = buf.trim().to_string();
        if from_stdin.is_empty() { args.query.join(" ") } else { from_stdin }
    };

    if query.trim().is_empty() {
        print_logo();
        println!("{}  ahelp {}", "Run:".dimmed(), "\"your question\"".bold());
        println!("{}  ahelp {}", "Or:".dimmed(), "--info".bold());
        println!("{}  ahelp {}", "Or:".dimmed(), "--provider openai \"question\"".bold());
        return;
    }

    let sys_text = sysinfo::format_context(&ctx);
    println!("{}", sys_text.dimmed());
    println!("\n{}\n", "Thinking...".yellow().bold());

    let provider_name = args.provider
        .as_deref()
        .and_then(|s| s.parse::<ProviderName>().ok())
        .unwrap_or_else(config::default_provider);

    let api_key = match config::get_key(provider_name) {
        Some(k) => k,
        None => {
            eprintln!("{} No API key for provider '{}'. Run: ahelp --config-key {}:YOUR_KEY",
                "Error:".red().bold(), provider_name, provider_name);
            std::process::exit(1);
        }
    };

    let provider = providers::build(provider_name, &api_key);
    match provider.generate(&sys_text, &query) {
        Ok(answer) => println!("{}\n{}", "Answer:".green().bold(), answer),
        Err(err) => {
            eprintln!("{} {}", "Error:".red().bold(), err);
            std::process::exit(1);
        }
    }
}
