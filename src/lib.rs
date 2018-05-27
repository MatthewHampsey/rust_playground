use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::fs::File;
use std::error::{Error};
use std::io::{Read};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    let h = word_freq("wow foo bar bar bar foo foo.
    rrr dfw vam vam
    wwefdd");
    println!("{:?}", h);
    assert!(1==2);
    }
}

pub struct Config {
    filepath: String,
    num_threads: usize,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, Box<Error>> {
        args.next();
        let filepath = match args.next() {
            Some(path) => path,
            None   => return Err(From::from("Expected file path")),
        };

        let num_threads: usize = match args.next() {
            Some(nthreads) => nthreads.parse()?,
            //Some(nthreads) => match nthreads.parse() {
            //                    Ok(n) => n,
            //                    Err(_) => return Err("Require int input"),
            //                  },
            None   => return Err(From::from("Expected number of threads")),
        };

        Ok(Config {filepath, num_threads })
    }
}

pub fn run(config: Config) -> Result<HashMap<String, u32>, Box<Error>> {
    let mut f = File::open(config.filepath)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(word_freq(&contents, config.num_threads)) 
}

fn word_freq(s: &str, num_threads:usize) -> HashMap<String, u32>{
    let (tx, rx) = mpsc::channel();
    let words: Vec<&str> = s.split_whitespace().collect();
    let section_length = words.len()/num_threads;
    
    let mut handles = vec![];

    for i in 0..num_threads {
        let tx_t = mpsc::Sender::clone(&tx);      
        
        let start_index = i*section_length;
        let end_index = if i == num_threads-1 {words.len()} else {(i+1)*section_length};
        //copy vector of &str into owned Strings to pass to the thread
        let s_t: Vec<String> = words[start_index..end_index].iter().map(|x| x.to_string()).collect();//;
        let handle = thread::spawn(move || {
            let mut h: HashMap<String, u32>  = HashMap::new();
            for word in s_t {
                let count = h.entry(word.to_string()).or_insert(0);
                *count += 1;
            }   
            tx_t.send(h).unwrap();
        });
        handles.push(handle);
    }

    let mut h: HashMap<String, u32> = HashMap::new();
    for _ in 0..num_threads {
        let h_t = rx.recv().unwrap();
        for (k, v) in h_t.iter() {
            let entry = h.entry(k.to_string()).or_insert(0);
            *entry += *v;
        }
    }
    for handle in handles{
        handle.join().unwrap();
    } 

    h
}




