use clap::Parser;

use std::fs;
use std::path::Path;

use commslang::cli::{
    create_project, print_check_success, print_simulation_results, resolve_source_file, Cli,
    Commands,
};

use commslang::compiler::compile;
use commslang::runtime::simulate_module;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name } => create_project(&name),

        Commands::Run { file } => run_command(file.as_deref()),

        Commands::Check { file } => check_command(file.as_deref()),

        Commands::Build { file } => build_command(file.as_deref()),

        Commands::Version => {
            println!("CommsLang {}", env!("CARGO_PKG_VERSION"));

            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("error: {}", error);
        std::process::exit(1);
    }
}

fn run_command(file: Option<&Path>) -> Result<(), String> {
    let source_file = resolve_source_file(file)?;

    let source = read_source(&source_file)?;

    let compilation = compile(&source)?;

    println!("Semantic analysis successful.");

    println!("IR generation successful.");

    println!();

    println!("Starting simulation...");

    println!();

    let results = simulate_module(&compilation.ir)?;

    print_simulation_results(&results);

    Ok(())
}

fn check_command(file: Option<&Path>) -> Result<(), String> {
    let source_file = resolve_source_file(file)?;

    let source = read_source(&source_file)?;

    compile(&source)?;

    print_check_success(&source_file);

    Ok(())
}

fn build_command(file: Option<&Path>) -> Result<(), String> {
    let source_file = resolve_source_file(file)?;

    let source = read_source(&source_file)?;

    let compilation = compile(&source)?;

    println!("Semantic analysis successful.");

    println!("IR generation successful.");

    println!();

    println!("{:#?}", compilation.ir);

    Ok(())
}

fn read_source(file: &Path) -> Result<String, String> {
    fs::read_to_string(file)
        .map_err(|error| format!("Could not read '{}': {}", file.display(), error))
}
