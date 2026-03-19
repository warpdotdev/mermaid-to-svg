use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            eprintln!("Error reading file {}: {}", args[1], e);
            std::process::exit(1);
        })
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {}", e);
            std::process::exit(1);
        });
        buffer
    };

    match mermaid_to_svg::render_mermaid_to_svg(&input, None) {
        Ok(svg) => println!("{}", svg),
        Err(e) => {
            eprintln!("Error rendering mermaid: {}", e);
            std::process::exit(1);
        }
    }
}
