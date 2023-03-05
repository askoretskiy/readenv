use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("[renv] Please provide a command to execute");
        exit(1);
    }

    let iter_result = dotenv::from_filename_iter(".env");

    match iter_result {
        Err(_) => println!("[renv] Failed to load .env file"),
        Ok(iter) => {
            for item_result in iter {
                let (key, value) = item_result.unwrap();
                env::set_var(&key, value);
            }
        }
    }

    // Try to execute and replace current process -- should quit here
    let err = exec::execvp(&args[1], &args[1..]);

    // If failed -- it would continue here; Show an error
    println!("[renv] Failed to run: {}", &args[1..].join(" "));
    println!("\t{}", err);
    exit(1);
}
