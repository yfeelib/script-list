use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "script-list")]
#[command(about = "📜 List npm scripts from package.json")]
#[command(version = "0.1.0")]
struct Cli {
    /// Path to package.json (default: ./package.json)
    #[arg(short, long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Show only script names without descriptions
    #[arg(short, long)]
    names_only: bool,

    /// Filter scripts by name (case-insensitive)
    #[arg(short, long, value_name = "PATTERN")]
    filter: Option<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "table")]
    format: OutputFormat,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum OutputFormat {
    Table,
    List,
    Json,
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: Option<String>,
    #[serde(default)]
    scripts: HashMap<String, String>,
    #[serde(default)]
    description: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let package_path = cli
        .path
        .unwrap_or_else(|| PathBuf::from("package.json"));

    let package = read_package_json(&package_path)?;

    if package.scripts.is_empty() {
        println!("{}", "⚠️  No scripts found in package.json".yellow());
        return Ok(());
    }

    let mut scripts: Vec<_> = package.scripts.clone().into_iter().collect();

    // Filter if specified
    if let Some(pattern) = cli.filter {
        let pattern = pattern.to_lowercase();
        scripts.retain(|(name, _)| name.to_lowercase().contains(&pattern));
    }

    // Sort by name
    scripts.sort_by(|a, b| a.0.cmp(&b.0));

    // Print header
    if !cli.names_only {
        print_header(&package);
    }

    // Output scripts
    match cli.format {
        OutputFormat::Table => print_table(&scripts, cli.names_only),
        OutputFormat::List => print_list(&scripts, cli.names_only),
        OutputFormat::Json => print_json(&scripts)?,
    }

    Ok(())
}

fn read_package_json(path: &PathBuf) -> Result<PackageJson> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Get current directory name
            let current_dir = env::current_dir().ok();
            let dir_name = current_dir
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            
            eprintln!();
            eprintln!("{}", dir_name.truecolor(139, 0, 0).bold()); // Dark red
            eprintln!();
            eprintln!("{}", "No package.json file found:".truecolor(128, 128, 128));
            eprintln!("{}", format!("  {}", current_dir.as_ref().map(|p| p.display().to_string()).unwrap_or_default()).truecolor(160, 160, 160));
            eprintln!();
            
            std::process::exit(1);
        }
        Err(e) => return Err(e).with_context(|| format!("Failed to read {}", path.display()))?,
    };

    let package: PackageJson = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {} as JSON", path.display()))?;

    Ok(package)
}

fn print_header(package: &PackageJson) {
    println!();
    if let Some(name) = &package.name {
        println!("{}", format!("📦 {}", name).bold().cyan());
    }
    if let Some(desc) = &package.description {
        println!("{}", desc.dimmed());
    }
    println!();
}

fn print_table(scripts: &[(String, String)], names_only: bool) {
    if names_only {
        for (name, _) in scripts {
            println!("{}", name.green());
        }
        return;
    }

    // Calculate column widths
    let name_width = scripts.iter().map(|(n, _)| n.len()).max().unwrap_or(10);
    let name_width = name_width.max(10);

    // Header
    println!("{:<width$}  {}", "Script".bold().underline(), "Command".bold().underline(), width = name_width);
    println!("{}", "─".repeat(name_width + 2 + 50).dimmed());

    // Rows
    for (name, command) in scripts {
        let display_cmd = if command.len() > 50 {
            format!("{}...", &command[..47])
        } else {
            command.clone()
        };
        println!("{:<width$}  {}", name.green().bold(), display_cmd.dimmed(), width = name_width);
    }

    println!();
    println!("{} {}", "ℹ️".cyan(), format!("Found {} script(s)", scripts.len()).dimmed());
}

fn print_list(scripts: &[(String, String)], names_only: bool) {
    for (name, command) in scripts {
        if names_only {
            println!("{}", name);
        } else {
            println!("{}: {}", name.green().bold(), command);
        }
    }
}

fn print_json(scripts: &[(String, String)]) -> Result<()> {
    let map: HashMap<&str, &str> = scripts
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    
    let json = serde_json::to_string_pretty(&map)?;
    println!("{}", json);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_package_json() {
        let json = r#"{
            "name": "test",
            "scripts": {
                "build": "cargo build",
                "test": "cargo test"
            }
        }"#;

        let package: PackageJson = serde_json::from_str(json).unwrap();
        assert_eq!(package.name, Some("test".to_string()));
        assert_eq!(package.scripts.len(), 2);
    }
}
