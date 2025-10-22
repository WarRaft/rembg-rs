use clap::Parser;
use std::process;

mod cli;
mod image_processor;
mod model;
mod background_removal;
mod error;

use cli::Args;
use model::ModelManager;
use background_removal::BackgroundRemover;

fn main() {
    let args = Args::parse();
    
    println!("🎨 rembg-rs - Background Removal Tool");
    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Model: {}", args.model);
    println!();

    // Initialize the model manager
    let model_manager = match ModelManager::new(&args.model) {
        Ok(mgr) => mgr,
        Err(e) => {
            eprintln!("❌ Failed to initialize model manager: {}", e);
            process::exit(1);
        }
    };
    
    // Initialize the background remover
    let mut background_remover = match BackgroundRemover::new(model_manager) {
        Ok(remover) => remover,
        Err(e) => {
            eprintln!("❌ Failed to initialize background remover: {}", e);
            process::exit(1);
        }
    };
    
    // Process the image
    match background_remover.remove_background(&args.input, &args.output, args.quality) {
        Ok(_) => {
            println!("✅ Background removed successfully!");
            println!("Output saved to: {:?}", args.output);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            process::exit(1);
        }
    }
}