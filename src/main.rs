use std::env;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let args: Vec<String> = env::args().collect();
}
