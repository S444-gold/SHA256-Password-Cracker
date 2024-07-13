use std::env;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::process;

struct Args{
    wanted_hash: String,
    password_file: String
}

impl Args {
    fn build(build_args: &[String]) -> Result<Args, &'static str>{
        if build_args.len() < 3{
            return Err("Not Enough Arguments")
        }

        let wanted_hash = build_args[1].clone();
        let password_file = build_args[2].clone();
        Ok(Args { wanted_hash, password_file})

    }
}

fn crack(crack_args: Args) -> Result<(), Box<dyn Error>>{
    let password_list = File::open(crack_args.password_file)?;
    let reader = BufReader::new(password_list);

    let mut attempts = 1;

    for line in reader.lines(){
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));
        //converts password into sha256 hash
        println!("|{}| {:?} {}",attempts, std::str::from_utf8(&password).unwrap(), password_hash);

        if crack_args.wanted_hash ==  password_hash{
            println!("|{}| Password Matched\nPassword: {:?}\nHash: {}",attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            process::exit(0);
        }
        attempts += 1;
    }
    Ok(println!("Password Not Found"))
}
fn main() {
    let config_args: Vec<String> = env::args().collect();
    let args = Args::build(&config_args).unwrap_or_else(|err|{
        eprintln!("Error Running Application: {}", err);
        process::exit(0);
    });

    if let Err(e) = crack(args){
        eprintln!("Error Cracking Password: {}", e);
        process::exit(0); 
    }
}




