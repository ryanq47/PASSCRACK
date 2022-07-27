
use md5::{Md5};
use sha2::{Sha256, Digest};

 //using this guide: https://docs.rs/md-5/0.10.1/md5/
 // and this https://github.com/RustCrypto/hashes

 // for readline: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;


use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    ///Wordlist to crack against
    #[clap(short, long, value_parser)]
    wordlist: String,

    ///Hash to crack
    #[clap(short, long, value_parser)]
    hash: String,

    ///type of hash to use (md5, sha256,)
    #[clap(short, long, value_parser)]
    t: String,

}


fn main() {
    let args = Args::parse();

    if args.t == "md5" {
        md5();
    }

    if args.t == "sha256" {
        sha256();
    }

    else {
        println!("Bad Hash Type '{}'", args.t);
    }
    //md5();
}


//Note, md5 is the guinea pig so that is why its kinda messy
fn md5() {
    let args = Args::parse();

    let uncracked = args.hash;
    //let uncracked = "5f4dcc3b5aa765d61d8327deb882cf99";

    //FOR DEBUGGING
    //println!("This is the hash from argument: {uncracked}");

    //let filename = "textfile.txt";
    let wordlist = args.wordlist;

    //println!("This is the filename from argument: {filename}"); FOR DEBUGGING

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.

        //grabbing a copy of input to print if a match
        let alt_input = input.clone();

        let mut hasher = Md5::new();

        //adding string to be hashed I beieve
        ////println!("Input is {:?}", input);
        
        hasher.update(input);

        //converting to hash
        let hash = hasher.finalize();

        //printing hash
        
        let encoded_hash = hex::encode(hash);
        ////println!("MD5 Hash is: {:?}", encoded_hash);

        //yes there is a better way to do this I know
        match uncracked.cmp(&encoded_hash) {
            Ordering::Less => println!("Trying {alt_input}..."),
            Ordering::Greater => println!("Trying {alt_input}..."),
            Ordering::Equal => {
                println!("
=======================================
Decoded MD5 Hash is: {alt_input}
=======================================");
                break;
            }
        }
    }
}


fn sha256() {
    let args = Args::parse();

    let uncracked = args.hash;
    let wordlist = args.wordlist;

    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.

        let alt_input = input.clone();

        let mut hasher = Sha256::new();

        hasher.update(input);

        let hash = hasher.finalize();
        
        let encoded_hash = hex::encode(hash);

        match uncracked.cmp(&encoded_hash) {
            Ordering::Less => println!("Trying {alt_input}..."),
            Ordering::Greater => println!("Trying {alt_input}..."),
            Ordering::Equal => {
                println!("
=======================================
Decoded SHA256 Hash is: {alt_input}
=======================================");
                    
                break;
            }
        }
    }
}