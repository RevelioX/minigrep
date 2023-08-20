use std::env;
use std::fs;

struct Config{
    query: String,
    filename: String
}

impl Config {
    fn new(args : &Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
    
}
fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}",args[1]);
    
    let config : Config = Config::new(&args);

    println!("Searching for: {:?}",config.query);
    println!("In file: {:?}",config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong");

    println!("Contents: {}",contents)
}

