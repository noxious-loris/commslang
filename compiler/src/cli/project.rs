use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_SOURCE: &str = "main.cl";

const DEFAULT_MAIN: &str = r#"system Main {

signal tx {
modulation = QPSK
power = 1 W
}

channel space{
model = AWGN
snr = 10 dB
}

receiver rx {
demodulation = QPSK
}

connect tx -> space -> rx

simulate {
bits = 100000
seed = 42
}


}
"#;

const DEFAULT_CONFIG: &str = r#"[package]
name = "PROJECT_NAME"
version = "0.1.0"

[build]
source = "main.cl"
"#;

const DEFAULT_README: &str = r#"# PROJECT_NAME

A CommsLang communication-system project.

## Run

```bash
commslang check
commslang run
commslang build
```

## Source

The main program is located in `main.cl`.
"#;

pub fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;

    loop {
        if current.join("commslang.toml").is_file() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

pub fn resolve_source_file(file: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(file) = file {
        let path = if file.is_absolute() {
            file.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|error| format!("Could not determine current directory: {}", error))?
                .join(file)
        };

        if !path.is_file() {
            return Err(format!("Source file '{}' does not exist.", path.display()));
        }

        return Ok(path);
    }

    let project_root = find_project_root().ok_or_else(|| {
        "Could not find a CommsLang project. Run 'commslang new <name>' or provide a source file."
            .to_string()
    })?;

    let source = project_root.join(DEFAULT_SOURCE);

    if !source.is_file() {
        return Err(format!(
            "Project source file '{}' does not exist.",
            source.display()
        ));
    }

    Ok(source)
}

pub fn create_project(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Project name cannot be empty.".to_string());
    }

    let project_path = PathBuf::from(name);

    if project_path.exists() {
        return Err(format!(
            "Directory '{}' already exists.",
            project_path.display()
        ));
    }

    fs::create_dir_all(project_path.join("src"))
        .map_err(|error| format!("Could not create project source directory: {}", error))?;

    fs::create_dir_all(project_path.join("examples"))
        .map_err(|error| format!("Could not create project examples directory: {}", error))?;

    let main_source = project_path.join("main.cl");
    let config = project_path.join("commslang.toml");
    let readme = project_path.join("README.md");

    let config_contents = DEFAULT_CONFIG.replace("PROJECT_NAME", name);
    let readme_contents = DEFAULT_README.replace("PROJECT_NAME", name);

    fs::write(&main_source, DEFAULT_MAIN)
        .map_err(|error| format!("Could not create '{}': {}", main_source.display(), error))?;

    fs::write(&config, config_contents)
        .map_err(|error| format!("Could not create '{}': {}", config.display(), error))?;

    fs::write(&readme, readme_contents)
        .map_err(|error| format!("Could not create '{}': {}", readme.display(), error))?;

    println!("Created CommsLang project '{}'.", name);
    println!();
    println!("  {}", project_path.join("main.cl").display());
    println!("  {}", project_path.join("commslang.toml").display());
    println!("  {}", project_path.join("src").display());
    println!("  {}", project_path.join("examples").display());
    println!("  {}", project_path.join("README.md").display());
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  commslang check");
    println!("  commslang run");

    Ok(())
}
