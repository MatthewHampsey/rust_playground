extern crate parallel_word_frequency;
use parallel_word_frequency::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
                     eprintln!("Problem parsing arguments: {}", err);
                     process::exit(1);
    });  


    let result = match parallel_word_frequency::run(config) {
        Ok(r) => r,
        Err(e) => { println!("Application error: {}", e);
                    process::exit(1)
                  }
    };
    println!("{:?}", result);

}
