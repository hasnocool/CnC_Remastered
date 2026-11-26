// Filename: crates/launcher/src/main.rs
//! CnC_Remastered application entry point.

use std::io::{self, Write};

use launcher::{
    application::ApplicationId,
    available_applications,
    launch,
};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    if let Err(error) = run() {
        eprintln!("Launcher error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut selected = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--app" | "-a" => {
                let value = args.next().ok_or("--app requires td, ra, or editor")?;
                selected = Some(value.parse::<ApplicationId>()?);
            }
            "--list" => {
                for app in available_applications() {
                    println!("{:<12} {} — {}", app.id, app.name, app.description);
                }
                return Ok(());
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            other => return Err(format!("unknown argument '{other}'").into()),
        }
    }

    let application = match selected {
        Some(application) => application,
        None => prompt_application()?,
    };
    launch(application)?;
    Ok(())
}

fn prompt_application() -> Result<ApplicationId, Box<dyn std::error::Error>> {
    println!("CnC Remastered — select an application");
    for (index, app) in available_applications().iter().enumerate() {
        println!("  {}. {} — {}", index + 1, app.name, app.description);
    }
    print!("Selection [1-3]: ");
    io::stdout().flush()?;
    let mut selection = String::new();
    io::stdin().read_line(&mut selection)?;
    match selection.trim() {
        "1" | "td" => Ok(ApplicationId::TiberianDawn),
        "2" | "ra" => Ok(ApplicationId::RedAlert),
        "3" | "editor" => Ok(ApplicationId::MapEditor),
        value => Err(format!("invalid application selection '{value}'").into()),
    }
}

fn print_help() {
    println!("CnC Remastered launcher");
    println!("  cnc-remastered [--app td|ra|editor]");
    println!("  cnc-remastered --list");
    println!("  cnc-remastered --help");
}
