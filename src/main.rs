
use md5::{Md5};
use md4::{Md4};
use md2::{Md2};
use sha2::{Sha256, Digest};
use sha1::{Sha1};
use sha3::{Sha3_256, Sha3_384, Sha3_512};
use sm3::{Sm3};
use ntlm_hash::*; //https://docs.rs/ntlm-hash/latest/ntlm_hash/
 //using this guide: https://docs.rs/md-5/0.10.1/md5/
 // and this https://github.com/RustCrypto/hashes

 // for readline: https://riptutorial.com/rust/example/4275/read-a-file-line-by-line

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Error};
use std::cmp::Ordering;
use std::io::prelude::*;


//very similar to argparse 
use clap::Parser;


use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    //Hi! This is a simple password cracker that hashes values from a wordlist, then compares them
    // to the hashed value. If the match, it spits out the matching word! The faster your CPU, the
    // faster this will run! (Note, the '-a' is only a placeholder so this will show up on top, it has no use)
    //#[clap(short, long, value_parser)]
    //a: String,


    ///Wordlist to crack against
    #[clap(short, long, value_parser)]
    wordlist: String,

    ///Hash to crack
    #[clap(short, long, value_parser)]
    hash: String,


    ///Type of hash to use: md5/4/2, sha2, sha1, sha3_256/384/512, sm3, and ntlm 
    #[clap(short, long, value_parser)]
    t: String,

    //Additional args, a workaround becuase I don't know how to take a flag without an argument
    //#[clap(short, long, value_parser, required(false))]
    //extra: String,
    //extra: Option<String>
    


}

fn main() {
    //println!("\x1b[2J");
    let args = Args::parse();
    starting_message();
    logic(&args.hash);
    //final_message();
}

fn starting_message() {
    let args = Args::parse();
    //println!("\x1b[1;1H");
    println!("=======================================");
    println!("Running Crack on {} hash '{}' using '{}'", args.t, args.hash ,args.wordlist);
    println!("=======================================");
}

fn final_message() {
    println!("If hash was found, it will be listed above");
}

fn exit() {
    pub struct ExitCode();
}

// Instead of each function pulling the hash from the terminal, now it is all passed to it via main() and second()
// this should make scalability/loading from a text file easier in the future

fn logic(input: &str) {
    /*
    //Jank Ass timer becuase idk itjust works
    let handle = thread::spawn(|| {
        

        println!("0 Seconds elapsed...");
        thread::sleep(Duration::from_millis(5000));
    
        println!("5 Seconds elapsed...");
        thread::sleep(Duration::from_millis(5000));
    
        println!("10 Seconds elapsed... switching update to every 30 seconds");
        thread::sleep(Duration::from_millis(20000));
    
        println!("30 Seconds elapsed...");
        thread::sleep(Duration::from_millis(30000));
    
        println!("60 Seconds elapsed...");
        thread::sleep(Duration::from_millis(30000));
    
        println!("90 Seconds elapsed...");
        thread::sleep(Duration::from_millis(30000));
                
        println!("120 Seconds elapsed...");
        thread::sleep(Duration::from_millis(30000));
    
        println!("150 Seconds elapsed... How big is this wordlist??");
        thread::sleep(Duration::from_millis(30000));

        
    });
    */


    let args = Args::parse();

    if args.hash == "*.txt" {
        file();
    }

    // MD hashes
    else if args.t == "md5" {
        //file();
        md5(input);
    }
    else if args.t == "md4" {
        md4(input);
    }
    else if args.t == "md2" {
        md2(input);
    }
    //SHA hashes
    else if args.t == "sha2" {
        sha256(input);
    }
    else if args.t == "sha1" {
        sha1(input);
    }
    else if args.t == "sha3_256" {
        sha3_256(input);
    }
    else if args.t == "sha3_384" {
        sha3_384(input);
    }
    else if args.t == "sha3_512" {
        sha3_512(input);
    }
    //sm3, a chinese standard
    else if args.t == "sm3" {
        sm3(input);
        //file();
    }
    //NTLM
    else if args.t == "ntlm" {
        ntlm(input);
    }

    // =Multithread method=====================================
    // works great except it ALL needs to stop when it finds a value
    else if args.t == "unknown" {
        let handle = thread::spawn(|| loop {
            md4("placeholder");
            thread::sleep(Duration::from_millis(1));
            break;
        });
        let handle = thread::spawn(|| loop {
            sha256("placeholder");
            thread::sleep(Duration::from_millis(1));

        });    
        let handle = thread::spawn(|| loop {
            sha1("placeholder");
            thread::sleep(Duration::from_millis(1));
        });  
        let handle = thread::spawn(|| loop {
            ntlm("placeholder");
            thread::sleep(Duration::from_millis(1));
        });        

        
        handle.join().unwrap();
    }

    else if args.wordlist == "bruteforce" {
        bruteforce();
    }


    else {
        println!("Bad Hash Type '{}'", args.t);
        //println!("^^ This may go off even on a good hash type, please ignore ^^") //seems to be solved by else if statements
    }

    //handle.join().unwrap(); //having thread join (for commented out timer)
}

fn file() {

    let args = Args::parse();

    let file = File::open("hashes.txt").unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap();

        println!("HIIIIIIIIIIIII");
        println!("{:?}", input);
        //println!("TEST CAN YOU SEE ME");
        logic(&input);
    }

}



//Note, md5 is the guinea pig so that is why its kinda messy
fn md5(hash_input: &str) {
    //loop {
        //println!("{}", test);
    //}

    let args = Args::parse();

    let uncracked = hash_input; //args.hash;
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
        
        //if args.extra == "verbose" {
            //println!("{alt_input}");
        //}


        if uncracked == encoded_hash {
            println!("
=======================================
Decoded MD5 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}


fn md4(hash_input: &str) {
    let args = Args::parse();

    let uncracked = hash_input;
    let wordlist = args.wordlist;

    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Md4::new();
        hasher.update(input);

        let hash = hasher.finalize();
        

        let encoded_hash = hex::encode(hash);


        if uncracked != encoded_hash {
            //println!("\x1b[1J");
            //println!("\x1b[5;1H");
            println!("MD4 {}", alt_input);
        }


        else if uncracked == encoded_hash {
            println!("
=======================================
Decoded MD4 Hash is: {alt_input}
=======================================");
            exit();
            break;
        }
    }
}

fn md2(hash_input: &str) {
    let args = Args::parse();

    let uncracked = hash_input;
    let wordlist = args.wordlist;

    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.

        let alt_input = input.clone();

        let mut hasher = Md2::new();

        hasher.update(input);

        let hash = hasher.finalize();
        
        let encoded_hash = hex::encode(hash);

        if uncracked == encoded_hash {
            println!("
=======================================
Decoded MD2 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}

fn sha256(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
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
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SHA-2 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}

fn sha1(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
    let wordlist = args.wordlist;
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Sha1::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SHA-1 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}

fn sha3_256(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
    let wordlist = args.wordlist;
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Sha3_256::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SHA3-256 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}

fn sha3_384(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
    let wordlist = args.wordlist;
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Sha3_384::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SHA3-384 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}

fn sha3_512(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
    let wordlist = args.wordlist;
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Sha3_512::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SHA3-512 Hash is: {alt_input}
=======================================");
            exit()        
        }
    }
}

fn sm3(hash_input: &str) {
    let args = Args::parse();
    let uncracked = hash_input;
    let wordlist = args.wordlist;
    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let input = line.unwrap(); // Ignore errors.
        let alt_input = input.clone();
        let mut hasher = Sm3::new();
        hasher.update(input);
        let hash = hasher.finalize();
        let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded SM3 Hash is: {alt_input}
=======================================");
            exit();
        }
    }
}



fn ntlm(hash_input: &str) {
    let args = Args::parse();

    let uncracked = hash_input;
    let wordlist = args.wordlist;

    let file = File::open(wordlist).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        //println!("\x1B[2J\x1B[1;1H");
        let input = line.unwrap(); // Ignore errors.

        //println!("{}",input.len());

        //handler for paswords over 31 characters
        if input.len() > 31 {
            continue
       };

        let alt_input = input.clone();


        let encoded_hash = ntlm_hash(&input);
        
        //println!("{}", hash);
        //let encoded_hash = hex::encode(hash);
        if uncracked == encoded_hash {
            println!("
=======================================
Decoded NTLM Hash is: {alt_input}
=======================================");
            exit()
        }
    }
}

fn bruteforce() -> Result<(), Error> {
    let args = Args::parse();

    let uncracked = args.hash;
    let wordlist = args.wordlist;

    //let file = File::open(wordlist).unwrap();
    //let reader = BufReader::new(file);

    for i in 0i64..999_999_999_999 {
        let input_init = i; // Ignore errors.

        let input = input_init.to_string();

        //println!("{}", input);

        //let alt_input = input.clone();

        // have this either write to file or pipe into a hash decryptor, also find how to get whole alphabet in 
 
        let mut output = File::create("tmp")?;
        let data = "test data";
        write!(output, "test")?;

    }
    Ok(())
}

