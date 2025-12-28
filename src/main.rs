// src/main.rs

mod generator;

use clap::{Parser, Subcommand};
use std::{env, fs, process::Command};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use anyhow::{Result, Context};
use crate::generator::{FrontendFramework, BackendFramework, Auth, Database, generate_env, generate_express_files, generate_fastapi_files, generate_docker_compose, generate_ci_cd};

#[derive(Parser)]
#[command(name = "forge", about = "Opinionated env & config generator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            run_init(name)?;
        }
    }

    Ok(())
}

fn run_init(name: Option<String>) -> Result<()> {
    let theme = ColorfulTheme::default();

    let project_name: String = match name {
        Some(n) => n,
        None => Input::with_theme(&theme)
            .with_prompt("Project name")
            .default("forge-project".into())
            .interact_text()?,
    };

    let frontend = match Select::with_theme(&theme)
        .with_prompt("Choose a Frontend")
        .items(&["None", "Next.js", "React (Vite)"])
        .default(1)
        .interact()?
    {
        0 => FrontendFramework::None,
        1 => FrontendFramework::NextJs,
        _ => FrontendFramework::React,
    };

    let backend = match Select::with_theme(&theme)
        .with_prompt("Choose a Backend")
        .items(&["None", "Express", "FastAPI"])
        .default(1)
        .interact()?
    {
        0 => BackendFramework::None,
        1 => BackendFramework::Express,
        _ => BackendFramework::FastAPI,
    };

    let db = match Select::with_theme(&theme)
        .with_prompt("Choose a database")
        .items(&["Postgres", "MySQL", "MongoDB", "SQLite"])
        .default(0)
        .interact()?
    {
        0 => Database::Postgres,
        1 => Database::MySQL,
        2 => Database::MongoDB,
        _ => Database::SQLite,
    };

    let auth = match Select::with_theme(&theme)
        .with_prompt("Choose auth type")
        .items(&["None", "JWT"])
        .default(0)
        .interact()?
    {
        0 => Auth::None,
        _ => Auth::Jwt,
    };

    println!("\n🚀 Scaffolding {}...", project_name);
    println!("Frontend: {:?}", frontend);
    println!("Backend: {:?}", backend);
    println!("Database: {:?}", db);
    println!("Auth: {:?}\n", auth);

    scaffold_project(&project_name, &frontend, &backend, &auth, &db)?;

    println!("\n Forge project created successfully!");
    println!(" cd {}", project_name);

    if frontend != FrontendFramework::None && backend != BackendFramework::None {
        println!("This is a Full Stack project.");
        println!(" cd frontend && npm install && npm run dev");
        println!("👉 cd backend && (setup backend)");
    } else {
        match frontend {
            FrontendFramework::NextJs | FrontendFramework::React => println!(" npm install\n npm run dev"),
            _ => {}
        }
        match backend {
            BackendFramework::Express => println!(" npm install\n npm run dev"),
            BackendFramework::FastAPI => println!(" pip install -r requirements.txt\n python main.py"),
            _ => {}
        }
    }

    Ok(())
}

fn scaffold_project(name: &str, frontend: &FrontendFramework, backend: &BackendFramework, auth: &Auth, db: &Database) -> Result<()> {
    let cwd = env::current_dir()?;
    let root_dir = cwd.join(name);

    // Determine directory structure
    // If both: root/frontend, root/backend
    // If one: root/
    let is_fullstack = *frontend != FrontendFramework::None && *backend != BackendFramework::None;

    // Create Root
    if !root_dir.exists() {
        fs::create_dir_all(&root_dir)?;
    }

    // --- FRONTEND ---
    if *frontend != FrontendFramework::None {
        // For CLI tools that CREATE a directory, we need to pass the target path.
        let target_arg = if is_fullstack {
            root_dir.join("frontend").to_str().unwrap().to_string()
        } else {
            root_dir.to_str().unwrap().to_string()
        };

        match frontend {
            FrontendFramework::NextJs => {
                // npx create-next-app@latest <target_arg>
                let status = Command::new("cmd")
                    .args(["/C", "npx", "create-next-app@latest", &target_arg])
                    .status()
                    .context("Failed to execute create-next-app")?;
                if !status.success() { eprintln!("Frontend generation failed or cancelled."); }
            }
            FrontendFramework::React => {
                // npm create vite@latest <target_arg> -- --template react
                let status = Command::new("cmd")
                    .args(["/C", "npm", "create", "vite@latest", &target_arg, "--", "--template", "react"])
                    .status()
                    .context("Failed to execute create-vite")?;
                if !status.success() { eprintln!("Frontend generation failed or cancelled."); }
            }
            _ => {}
        }
    }

    // --- BACKEND ---
    if *backend != BackendFramework::None {
        let be_target_dir = if is_fullstack { root_dir.join("backend") } else { root_dir.clone() };
        
        // Express/FastAPI allow us to just write files to be_target_dir
        if !be_target_dir.exists() {
            fs::create_dir_all(&be_target_dir)?;
        }

        match backend {
            BackendFramework::Express => {
                let files = generate_express_files(name, db, auth);
                for (path, content) in files {
                    let file_path = be_target_dir.join(path);
                    if let Some(parent) = file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(file_path, content)?;
                }
            }
            BackendFramework::FastAPI => {
                let files = generate_fastapi_files(name, db, auth);
                for (path, content) in files {
                    let file_path = be_target_dir.join(path);
                    if let Some(parent) = file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(file_path, content)?;
                }
            }
            _ => {}
        }
    }

    // --- COMMON FILES (Root) ---
    // .env.example
    fs::write(
        root_dir.join(".env.example"),
        generate_env(frontend, backend, auth, db),
    )?;

    // docker-compose.yml
    if let Some(docker_compose) = generate_docker_compose(db) {
        fs::write(root_dir.join("docker-compose.yml"), docker_compose)?;
    }

    // CI/CD
    let ci_cd_dir = root_dir.join(".github").join("workflows");
    fs::create_dir_all(&ci_cd_dir)?;
    fs::write(
        ci_cd_dir.join("ci.yml"),
        generate_ci_cd(name, is_fullstack),
    )?;

    // README.md
    let readme_content = format!(
        r#"
## Generated by Forge
- Frontend: {:?}
- Backend: {:?}
- Database: {:?}
- Auth: {:?}

### Structure
{}

### Setup
Check .env.example for environment variables.
"#,
        frontend, backend, db, auth,
        if is_fullstack { "- ./frontend\n- ./backend" } else { "- Root" }
    );

    let readme_path = root_dir.join("README.md");
    // Append if exists (e.g. key Next.js/Vite generated one), else create.
    if readme_path.exists() {
        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(readme_path)?;
        writeln!(file, "\n{}", readme_content)?;
    } else {
        fs::write(readme_path, format!("# {}\n{}", name, readme_content))?;
    }

    Ok(())
}
