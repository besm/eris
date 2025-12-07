//! ERIS notation lookup CLI

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use eris::{
    get_all_definitions, get_all_symbols, lookup_symbol,
    get_operator_symbols, get_entity_symbols, lookup_operator, lookup_entity,
};
use eris::operators::get_all_definitions as get_operator_definitions;
use eris::entities::get_all_definitions as get_entity_definitions;
use eris::frame::{Frame, Role, Context, Task, get_role, get_context, get_task, get_workflow, list_workflows};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::path::PathBuf;

/// Type alias for symbol lookup: (defined symbols, lookup function)
type SymbolLookup = (HashSet<String>, fn(&str) -> Option<String>);

/// Print line, exiting gracefully on broken pipe
macro_rules! println_or_exit {
    ($($arg:tt)*) => {
        if writeln!(io::stdout(), $($arg)*).is_err() {
            std::process::exit(0);
        }
    };
}

#[derive(Parser, Debug)]
#[command(name = "eris")]
#[command(
    about = "Query ERIS notation symbol definitions",
    long_about = "ERIS Notation System - Query and explore symbol definitions

FOR LLM CONSUMERS:
Run `eris all` to ingest complete symbol definitions.
Use `eris <symbol>` for individual lookups.
Use `eris list` to see all defined symbols.
Use `eris closure` to find undefined symbols used in definitions.

FRAMES (composable):
Use `--role <code>` for role specification (nav, pln, evl, crt, itg).
Use `--context <code>` for context specification (ann, wfl, str, eps).
Use `--task <code>` for task specification (validate, tag, review, migrate, query).
Use `--roles`, `--contexts`, `--tasks` to list all of each type."
)]
struct Args {
    /// Role specification: nav, pln, evl, crt, itg
    #[arg(long)]
    role: Option<String>,

    /// Context specification: ann, wfl, str, eps
    #[arg(long)]
    context: Option<String>,

    /// Task specification: validate, tag, review, migrate, query
    #[arg(long)]
    task: Option<String>,

    /// List all roles
    #[arg(long)]
    roles: bool,

    /// List all contexts
    #[arg(long)]
    contexts: bool,

    /// List all tasks
    #[arg(long)]
    tasks: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List all defined symbols
    List,
    /// Output all symbol definitions
    All,
    /// Output all operator definitions
    Ops,
    /// Output all entity definitions
    Entities,
    /// Output definitions for ERIS symbols used in a file
    Define {
        /// Category filter: ops, entities, or all (default)
        #[arg(value_parser = ["ops", "entities", "all"])]
        category: Option<String>,
        /// Path to file to analyze
        file: PathBuf,
    },
    /// Find undefined symbols used in definitions
    Closure {
        /// Show usage counts for each undefined symbol
        #[arg(long, short)]
        count: bool,
    },
    /// Display workflow specification for LLM ingestion
    Workflow {
        /// Workflow name (tag, validate, etc.) or 'list' to show available
        name: String,
    },
    /// Look up a specific symbol
    #[command(external_subcommand)]
    Lookup(Vec<String>),
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle frame listing flags
    if args.roles {
        for role in Role::all() {
            println_or_exit!("{role}");
            println_or_exit!();
        }
        return Ok(());
    }
    if args.contexts {
        for ctx in Context::all() {
            println_or_exit!("{ctx}");
            println_or_exit!();
        }
        return Ok(());
    }
    if args.tasks {
        for task in Task::all() {
            println_or_exit!("{task}");
            println_or_exit!();
        }
        return Ok(());
    }

    // Handle composed frame flags
    if args.role.is_some() || args.context.is_some() || args.task.is_some() {
        let mut frame = Frame::new();

        if let Some(ref code) = args.role {
            match get_role(code) {
                Some(r) => frame = frame.with_role(r),
                None => return Err(anyhow!("Unknown role: '{}'. Valid: nav, pln, evl, crt, itg", code)),
            }
        }
        if let Some(ref code) = args.context {
            match get_context(code) {
                Some(c) => frame = frame.with_context(c),
                None => return Err(anyhow!("Unknown context: '{}'. Valid: ann, wfl, str, eps", code)),
            }
        }
        if let Some(ref code) = args.task {
            match get_task(code) {
                Some(t) => frame = frame.with_task(t),
                None => return Err(anyhow!("Unknown task: '{}'. Valid: validate, tag, review, migrate, query", code)),
            }
        }

        println_or_exit!("{frame}");
        return Ok(());
    }

    // Handle subcommands
    let command = args.command.unwrap_or_else(|| {
        eprintln!("error: requires a subcommand or frame flags");
        eprintln!("       use `eris --help` for usage");
        std::process::exit(2);
    });

    match command {
        Command::List => {
            let mut symbols: Vec<_> = get_all_symbols().into_iter().collect();
            symbols.sort();
            for symbol in symbols {
                println_or_exit!("{symbol}");
            }
        }
        Command::All => {
            let defs = get_all_definitions();
            for (i, text) in defs.iter().enumerate() {
                println_or_exit!("{text}");
                if i < defs.len() - 1 {
                    println_or_exit!();
                }
            }
        }
        Command::Ops => {
            let defs = get_operator_definitions();
            for (i, text) in defs.iter().enumerate() {
                println_or_exit!("{text}");
                if i < defs.len() - 1 {
                    println_or_exit!();
                }
            }
        }
        Command::Entities => {
            let defs = get_entity_definitions();
            for (i, text) in defs.iter().enumerate() {
                println_or_exit!("{text}");
                if i < defs.len() - 1 {
                    println_or_exit!();
                }
            }
        }
        Command::Define { category, file } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|e| anyhow!("Failed to read '{}': {}", file.display(), e))?;

            // Select symbol set and lookup function based on category
            let cat = category.as_deref().unwrap_or("all");
            let (defined, lookup_fn): SymbolLookup = match cat {
                "ops" => (get_operator_symbols(), lookup_operator),
                "entities" => (get_entity_symbols(), lookup_entity),
                _ => (get_all_symbols(), lookup_symbol),
            };

            // Collect symbols used in file that have definitions
            let mut used: HashSet<String> = HashSet::new();
            for ch in content.chars() {
                let s = ch.to_string();
                if defined.contains(&s) {
                    used.insert(s);
                }
            }

            // Output definitions for used symbols (sorted)
            let mut symbols: Vec<_> = used.into_iter().collect();
            symbols.sort();
            for (i, symbol) in symbols.iter().enumerate() {
                if let Some(text) = lookup_fn(symbol) {
                    println_or_exit!("{text}");
                    if i < symbols.len() - 1 {
                        println_or_exit!();
                    }
                }
            }
        }
        Command::Closure { count } => {
            let defined = get_all_symbols();
            let all_text = get_all_definitions().join("\n");

            let standard_punct: HashSet<&str> = [
                "'", "(", ")", ",", "-", "/", ":", "=", ">", "[", "]", "_", "|", "{", "}", "⦑",
                "⦒", "\"", ".", "?", "+", "<", "≤", "≥", "≠", "✓", "✗", "⁻",
            ]
            .into_iter()
            .collect();

            let mut symbol_counts: HashMap<String, usize> = HashMap::new();
            for ch in all_text.chars() {
                if !ch.is_ascii_alphanumeric() && !ch.is_whitespace() {
                    let s = ch.to_string();
                    if !standard_punct.contains(s.as_str()) && !defined.contains(&s) {
                        *symbol_counts.entry(s).or_insert(0) += 1;
                    }
                }
            }

            if count {
                let mut symbol_freq: Vec<_> = symbol_counts.into_iter().collect();
                symbol_freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                for (symbol, count) in symbol_freq {
                    println_or_exit!("{count:4} {symbol}");
                }
            } else {
                let mut undefined: Vec<_> = symbol_counts.keys().cloned().collect();
                undefined.sort();
                for symbol in undefined {
                    println_or_exit!("{symbol}");
                }
            }
        }
        Command::Workflow { name } => {
            if name == "list" {
                for wf in list_workflows() {
                    println_or_exit!("{wf}");
                }
            } else if let Some(text) = get_workflow(&name) {
                println_or_exit!("{text}");
            } else {
                return Err(anyhow!("Workflow '{}' not found. Use 'eris workflow list' to see available.", name));
            }
        }
        Command::Lookup(args) => {
            let symbol = args.join(" ");
            if let Some(text) = lookup_symbol(&symbol) {
                println_or_exit!("{text}");
            } else {
                return Err(anyhow!("Symbol '{symbol}' not found in ERIS definitions"));
            }
        }
    }

    Ok(())
}
