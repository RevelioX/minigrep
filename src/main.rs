use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::Config;
use minigrep::run;


fn main() {
    let args : Vec<String> = env::args().collect();
    
    let config : Config = minigrep::Config::new(&args).unwrap_or_else( |err| {
        println!("Problem with arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {:?}",config.query);
    println!("In file: {:?}",config.filename);

    if let Err(e) = run(config){
        println!("App error: {}", e);
        process::exit(1);
    };
}

