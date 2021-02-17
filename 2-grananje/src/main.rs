const ITERATIONS: u32 = 50000;
const ARRAY_SIZE: usize = 8192;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let mut args = env::args();
    args.next().expect("progname");
    if let Some(param) = args.next() {
	match sum_array(&param) {
	    Ok(sum) => println!("sum: {}", sum),
	    Err(e) => {
		eprintln!("arraysum: error: {}", e);
		process::exit(1);
	    }
	}
    } else {
	eprintln!("Usage: arraysum <filename>");
	process::exit(1);
    }
}

fn sum_array(filename: &str) -> Result<u32, Box<Error>> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut vals = Vec::with_capacity(ARRAY_SIZE);
    for line in buf_reader.lines() {
	vals.push(u8::from_str_radix(&line?, 16)?);
    }
    if vals.len() != 8192 {
	return Err("file not 8192 lines long")?;
    }
    let mut sum: u32 = 0;
    for _ in 0..ITERATIONS {
	for &val in vals.iter() {
	    if val >= 128 {
		sum = sum.wrapping_add(val as u32);
	    }
	}
    }
    Ok(sum)
}
