use std::time::Instant;

const ITERATIONS: usize = 10;
const ARRAY_SIZE: usize = 8 << 20;
const OPS: f64 = (ITERATIONS * ARRAY_SIZE) as f64;
const NANOS_IN_SEC: u64 = 1_000_000_000;

fn main() {
    let mut array = vec![0u8; ARRAY_SIZE].into_boxed_slice();
    let mut step = 1;
    for s in 0..16 {
	let now = Instant::now();
	for t in 0..(ITERATIONS * step) {
	    let mut ix = 0;
	    while ix < ARRAY_SIZE {
		let c = array[ix] as usize * t;
		array[ix] = c as u8;
		ix += step;
	    }
	}
	step*=2;
	let elapsed = now.elapsed();
	let nanos = (elapsed.as_secs() * NANOS_IN_SEC + elapsed.subsec_nanos() as u64) as f64;
	let nsop = nanos/OPS;
	println!("it: {}, ns/op: {:.2}", s, nsop);
    }
}
