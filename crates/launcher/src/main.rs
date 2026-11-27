// Filename: crates/launcher/src/main.rs
//! CnC_Remastered application entry point.

use launcher::{application::ApplicationId, available_applications, launch, launch_menu};
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
    let mut selected=None;
    let mut args=std::env::args().skip(1);
    while let Some(argument)=args.next() {
        match argument.as_str() {
            "--app"|"-a"=>{
                let value=args.next().ok_or("--app requires td, ra, or editor")?;
                selected=Some(value.parse::<ApplicationId>()?);
            }
            "--list"=>{
                for app in available_applications(){println!("{:<12} {} — {}",app.id,app.name,app.description);}
                return Ok(());
            }
            "--help"|"-h"=>{print_help();return Ok(());}
            other=>return Err(format!("unknown argument '{other}'").into()),
        }
    }
    if let Some(application)=selected { launch(application)?; }
    else { launch_menu()?; }
    Ok(())
}

fn print_help() {
    println!("CnC Remastered launcher");
    println!("  cnc-remastered              Open graphical launcher");
    println!("  cnc-remastered --app td|ra|editor");
    println!("  cnc-remastered --list");
    println!("  cnc-remastered --help");
}
