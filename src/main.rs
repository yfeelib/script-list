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
    #[arg(short = 'f', long, value_name = "PATTERN")]
    filter: Option<String>,

    /// Output format
    #[arg(short = 'F', long, value_enum, default_value = "table")]
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
    #[serde(default)]
    scripts: HashMap<String, String>,
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

    // Output scripts
    match cli.format {
        OutputFormat::Table => print_scripts(&package, &scripts, cli.names_only),
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
            eprintln!("   {}", dir_name.red().bold());
            eprintln!();
            eprintln!("   No package.json file found:");
            eprintln!("     {}", current_dir.as_ref().map(|p| p.display().to_string()).unwrap_or_default());
            eprintln!();
            
            std::process::exit(1);
        }
        Err(e) => return Err(e).with_context(|| format!("Failed to read {}", path.display()))?,
    };

    let package: PackageJson = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {} as JSON", path.display()))?;

    Ok(package)
}

fn print_scripts(_package: &PackageJson, scripts: &[(String, String)], names_only: bool) {
    if names_only {
        for (name, _) in scripts {
            println!("{}", name);
        }
        return;
    }

    // Use current directory name (like rousan/sl does)
    let module_name = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    
    // Print module name (3 spaces prefix, green)
    println!();
    println!("   {}", module_name.green());
    println!();

    // Calculate max script name length for padding
    let max_len = scripts.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    // Print each script (3 spaces + " - " prefix)
    for (name, command) in scripts {
        let padded_name = format!("{:<width$}", name, width = max_len);
        println!("    - {} : {}", padded_name.truecolor(128, 128, 128), command);
    }

    println!();
}

fn print_list(scripts: &[(String, String)], names_only: bool) {
    println!();
    for (name, command) in scripts {
        if names_only {
            println!("   {}", name);
        } else {
            println!("   {}: {}", name, command);
        }
    }
    println!();
}

fn print_json(scripts: &[(String, String)]) -> Result<()> {
    let map: HashMap<&str, &str> = scripts
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    
    let json = serde_json::to_string_pretty(&map)?;
    println!();
    for line in json.lines() {
        println!("   {}", line);
    }
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_package_json() {
        let json = r#"{
            "scripts": {
                "build": "cargo build",
                "test": "cargo test"
            }
        }"#;

        let package: PackageJson = serde_json::from_str(json).unwrap();
        assert_eq!(package.scripts.len(), 2);
    }
}
