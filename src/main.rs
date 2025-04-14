use std::env;           // Provides functionality for working with the environment (args, variables, etc.)
use std::process;       // Used for exiting the process with specific codes, among other features.

use minigrep::Config;   // Brings the Config struct from the lib file into scope.

fn main() 
{
    // Collect the command-line arguments into a vector of strings.
    let args: Vec<String> = env::args().collect(); 
    
    // Try to build a Config struct from the arguments. If there's an error, handle it gracefully.
    let config = Config::build(&args)
        .unwrap_or_else(|_err| 
            {
                eprintln!("Problem parsing arguments: {}", _err);
                process::exit(1);
            });

    // Run the main logic of the program with the config.
    // If an error occurs, handle it by printing and exiting.
    if let Err(_e) = minigrep::run(config)
    {
        eprintln!("Application error: {}", _e);
        process::exit(1);
    }
}
