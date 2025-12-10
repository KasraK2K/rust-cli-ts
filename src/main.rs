use clap::Parser;
use std::fs;
use std::path::Path;
use techsuite_cli::{Flag, get_template_files};

#[derive(Parser, Debug)]
#[command(name = "techsuite-cli")]
#[command(about = "TechSuite file generator")]
struct Args {
    kind: String,
    name: Option<String>,
    path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let flag = Flag::get_flag(&args.kind, args.name);

    let files = get_template_files(&flag, args.path);

    for f in files {
        // println!("Generating: {}", f.file_name);
        // println!("{}", f.content);
        // println!("{}", f.path);
        
        let full_path = Path::new(&f.path).join(&f.file_name);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("failed to create directories");
        }

        fs::write(full_path, f.content).expect("write failed");
    }
}
