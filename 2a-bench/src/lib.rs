#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    const ARRAY_SIZE: usize = 8192;

    use std::error::Error;
    use std::io::{BufRead, BufReader};

    use test::{black_box, Bencher};

    #[bench]
    fn unsorted(b: &mut Bencher) {
	fn sum_array() -> Result<u32, Box<Error>> {
	    let file = include_bytes!("../2.input");
	    let buf_reader = BufReader::new(&file[..]);
	    let mut vals = Vec::with_capacity(ARRAY_SIZE);
	    for line in buf_reader.lines() {
		vals.push(u8::from_str_radix(&line?, 16)?);
	    }
	    if vals.len() != 8192 {
		return Err("file not 8192 lines long")?;
	    }
	    let mut sum: u32 = 0;
	    for &val in vals.iter() {
		if val >= 128 {
		    sum = sum.wrapping_add(val as u32);
		}
	    }
	    Ok(sum)
	}

	b.iter(|| {
	    black_box(sum_array().ok());
	});
	}
	#[bench]
	fn sorted(b: &mut Bencher) {
		fn sum_array() -> Result<u32, Box<Error>> {
			let file = include_bytes!("../2.sorted");
			let buf_reader = BufReader::new(&file[..]);
			let mut vals = Vec::with_capacity(ARRAY_SIZE);
			for line in buf_reader.lines() {
			vals.push(u8::from_str_radix(&line?, 16)?);
			}
			if vals.len() != 8192 {
			return Err("file not 8192 lines long")?;
			}
			let mut sum: u32 = 0;
			for &val in vals.iter() {
			if val >= 128 {
				sum = sum.wrapping_add(val as u32);
			}
			}
			Ok(sum)
		}
	
		b.iter(|| {
			black_box(sum_array().ok());
		});
		}
}
