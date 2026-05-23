use clap::Parser;
use colored::Colorize;
use std::io::{self, IsTerminal, Read};

mod config;
mod gemini;
mod sysinfo;

#[derive(Parser)]
#[command(
    name = "ahelp",
    about = "AI Linux assistant for the terminal",
    long_about = "\n\nAI Linux terminal assistant powered by Google Gemini. \nYou ask in natural language, it reads your system context\n(OS, shell, CPU, RAM, disk, PWD) and returns concise,\ncopy-paste ready commands.\n",
    version = "0.1.0"
)]
struct Args {
    /// Your natural-language query
    query: Vec<String>,

    /// Store Gemini API key
    #[arg(long, value_name = "KEY")]
    config_key: Option<String>,

    /// Print system context and exit
    #[arg(long, action = clap::ArgAction::SetTrue)]
    info: bool,
}

fn print_logo() {
    let logo = r#"
    ┏━┓┏━┓╻ ╻┏━┓╻  ╏
    ┣━┫┣┳┛┃━┫┃ ┃┃  ╏
    ╹ ╹╹┗╸┗━┛┗━┛┗━╸╏
    AI Linux assistant · by qzwtrp
    "#;
    println!("{}", logo.cyan().bold());
}

fn print_system_context(ctx: &sysinfo::Context) {
    println!(
        "{}\n{}",
        "=== SYSTEM CONTEXT ===".dimmed(),
        sysinfo::format_context(ctx)
    );
}

fn main() {
    let args = Args::parse();

    if let Some(key) = args.config_key {
        config::set_key(key);
        println!(
            "{} API key saved to {}",
            "✓".green().bold(),
            "~/.config/ahelp/api_key".underline()
        );
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
        if from_stdin.is_empty() {
            args.query.join(" ")
        } else {
            from_stdin
        }
    };

    if query.trim().is_empty() {
        print_logo();
        println!("{}  ahelp {}", "Run:".dimmed(), "\"your question\"".bold());
        println!("{}  ahelp {}", "Or:".dimmed(), "--info".bold());
        return;
    }

    let sys_text = sysinfo::format_context(&ctx);
    println!("{}", sys_text.dimmed());
    println!("\n{}\n", "Thinking...".yellow().bold());

    let api_key = match config::get_key() {
        Some(k) => k,
        None => {
            eprintln!(
                "{} No Gemini API key configured.\nRun: {}",
                "Error:".red().bold(),
                "ahelp --config-key <YOUR_KEY>".bold()
            );
            std::process::exit(1);
        }
    };

    let answer = gemini::generate(&api_key, &sys_text, &query);

    println!("{}\n{}", "Answer:".green().bold(), answer);
}
