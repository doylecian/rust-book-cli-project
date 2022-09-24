use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    let mut f = File::open(config.filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could not read file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let filename = args[2].clone();
        Self { query, filename }
    }
}


