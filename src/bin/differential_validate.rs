use std::fs;
use std::path::PathBuf;
use std::process;

use sas_rs::validation::run_differential_validation;

fn main() {
    let mut work_dir = PathBuf::from(".tmp/differential-validation");
    let mut output_path = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--work-dir" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| usage("missing value for --work-dir"));
                work_dir = PathBuf::from(value);
            }
            "--output" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| usage("missing value for --output"));
                output_path = Some(PathBuf::from(value));
            }
            _ => usage(&format!("unknown argument: {arg}")),
        }
    }

    let report = run_differential_validation(&work_dir);
    let rendered = report.render_text();

    if let Some(path) = output_path {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|error| {
                eprintln!(
                    "failed to create output directory {}: {error}",
                    parent.display()
                );
                process::exit(2);
            });
        }
        fs::write(&path, &rendered).unwrap_or_else(|error| {
            eprintln!("failed to write {}: {error}", path.display());
            process::exit(2);
        });
    }

    println!("{rendered}");
    if report.failure_count() > 0 {
        process::exit(1);
    }
    if report.skipped_count() > 0 {
        process::exit(2);
    }
}

fn usage(message: &str) -> ! {
    eprintln!("{message}");
    eprintln!("usage: cargo run --bin differential_validate -- [--work-dir PATH] [--output PATH]");
    process::exit(2);
}
