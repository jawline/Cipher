extern crate rand;

mod stream;
mod keys;

use std::env;
use stream::*;

fn main() {
	let args: Vec<_> = env::args().collect();

	assert!(args.len() == 4);

	let command = &args[1];
	let key = &args[2];
	let text = &args[3];

	println!("{}", String::from_utf8(stream::decrypt(&stream::encrypt("hello".as_bytes(), &[234,062,323]), &[234,062,323])).unwrap());

	if command == "encrypt" {

	} else if command == "decrypt" {

	} else {
		println!("Unknown command");
	}
}
