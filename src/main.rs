use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("[renv] Please provide a command to execute");
        exit(1);
    }

    // Extend ENV with content of `.env` file (do not overwrite existing)
    let dotenv_result = dotenv::from_path(".env");

    if dotenv_result.is_err() {
        println!("[renv] Failed to load .env file");
    }

    // Try to execute and replace current process -- should quit here
    let err = exec::execvp(&args[1], &args[1..]);

    // If failed -- it would continue here; Show an error
    println!("[renv] Failed to run: {}", &args[1..].join(" "));
    println!("\t{}", err);
    exit(1);
}
