use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("[renv] Please provide a command to execute");
        exit(1);
    }

    let iter = dotenv::from_path_iter(".env");

    if iter.is_err() {
        println!("[renv] Failed to load .env file");
    } else {
        for item in iter.unwrap() {
            let (key, value) = item.unwrap();
            // Set env variable if not set or key is "PATH"
            if env::var(&key).is_err() || &key == "PATH" {
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
