use std::fs;
use std::path::PathBuf;
use std::process;

use sas_rs::validation::{sample_corpus_root, sweep_sample_corpus};

fn main() {
    let mut root = sample_corpus_root();
    let mut batch_size_rows = 1_024_usize;
    let mut limit = None;
    let mut output_path = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sample-root" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| usage("missing value for --sample-root"));
                root = PathBuf::from(value);
            }
            "--batch-size-rows" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| usage("missing value for --batch-size-rows"));
                batch_size_rows = value
                    .parse()
                    .unwrap_or_else(|_| usage("--batch-size-rows must be a positive integer"));
            }
            "--limit" => {
                let value = args
                    .next()
                    .unwrap_or_else(|| usage("missing value for --limit"));
                limit = Some(
                    value
                        .parse()
                        .unwrap_or_else(|_| usage("--limit must be a positive integer")),
                );
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

    let report = sweep_sample_corpus(&root, batch_size_rows, limit).unwrap_or_else(|error| {
        eprintln!("sample corpus sweep failed: {error}");
        process::exit(2);
    });
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
    if report.compatibility_failure_count() > 0 {
        process::exit(1);
    }
}

fn usage(message: &str) -> ! {
    eprintln!("{message}");
    eprintln!(
        "usage: cargo run --bin sample_corpus_sweep -- [--sample-root PATH] [--batch-size-rows N] [--limit N] [--output PATH]"
    );
    process::exit(2);
}
