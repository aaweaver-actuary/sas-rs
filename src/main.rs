use std::process::ExitCode;

fn main() -> ExitCode {
    match sas_rs::cli::run(std::env::args_os()) {
        Ok(outcome) => {
            println!("{outcome}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            error.exit_code()
        }
    }
}
