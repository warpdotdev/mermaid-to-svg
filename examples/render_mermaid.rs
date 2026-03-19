use mermaid_to_svg::render_mermaid_to_svg;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <mermaid_file>", args[0]);
        std::process::exit(1);
    }
    let content = fs::read_to_string(&args[1]).expect("Failed to read file");
    match render_mermaid_to_svg(&content, None) {
        Ok(svg) => print!("{}", svg),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
