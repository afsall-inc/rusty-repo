use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "rusty-repo",
    version,
    about = "Scaffold new Rust projects from templates"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New {
        name: String,
        #[arg(short, long, default_value = "default")]
        template: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    Init {
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    Templates,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::New {
            name,
            template,
            output,
        } => {
            let dest = output
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(&name));
            cmd_new(&name, &template, &dest)
        }
        Commands::Init { template } => {
            let cwd = std::env::current_dir()?;
            let name = cwd
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project")
                .to_string();
            cmd_new(&name, &template, &cwd)
        }
        Commands::Templates => cmd_templates(),
    }
}

fn cmd_new(name: &str, template: &str, dest: &PathBuf) -> anyhow::Result<()> {
    let template_dir = template_path(template)?;
    if !template_dir.exists() {
        anyhow::bail!(
            "Template '{template}' not found at {}",
            template_dir.display()
        );
    }

    if dest.exists() {
        if dest.read_dir()?.next().is_some() {
            anyhow::bail!(
                "Destination '{}' exists and is not empty",
                dest.display()
            );
        }
    } else {
        std::fs::create_dir_all(dest)?;
    }

    copy_dir(&template_dir, dest, name)?;
    println!("Created project '{name}' at {}", dest.display());
    println!("  template: {template}");
    println!("  next: cd {} && cargo check", dest.display());
    Ok(())
}

fn template_path(name: &str) -> anyhow::Result<PathBuf> {
    let root = std::env::current_exe()?
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let candidates = [
        root.join("templates").join(name),
        root.join("../../templates").join(name),
        PathBuf::from("templates").join(name),
    ];

    for c in &candidates {
        if c.exists() {
            return Ok(c.clone());
        }
    }
    anyhow::bail!("Template '{name}' not found")
}

fn copy_dir(
    src: &Path,
    dest: &Path,
    project_name: &str,
) -> std::io::Result<()> {
    for entry in walkdir(src)? {
        let relative = entry.strip_prefix(src).unwrap();
        let target = dest.join(relative);

        if entry.is_dir() {
            std::fs::create_dir_all(&target)?;
        } else {
            let content = std::fs::read_to_string(entry)?;
            let processed = content
                .replace("{{project_name}}", project_name)
                .replace("{{author}}", "Afsall");
            std::fs::write(&target, &processed)?;
        }
    }
    Ok(())
}

fn walkdir(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            entries.push(path.clone());
            entries.extend(walkdir(&path)?);
        } else {
            entries.push(path);
        }
    }
    Ok(entries)
}

fn cmd_templates() -> anyhow::Result<()> {
    println!("Available templates:");
    for t in ["default", "workspace", "prdoc"] {
        println!("  {t}");
    }
    Ok(())
}
