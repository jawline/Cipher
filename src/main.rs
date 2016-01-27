extern crate rand;
extern crate rustc_serialize;

mod stream;
mod keys;
mod fiestal;

use rustc_serialize::base64::{STANDARD, FromBase64, ToBase64};
use std::env;
use stream::*;

fn main() {
	let args: Vec<_> = env::args().collect();

	assert!(args.len() >= 2);

	let command = &args[1];

	match command as &str {
		"encrypt" => {
			let key_text = &args[2].from_base64().unwrap();
			let text = &args[3];
			println!("{}", &stream::encrypt("hello".as_bytes(), &key_text, &fiestal::fiestal).to_base64(STANDARD));
		},
		"decrypt" => {
			let key_text = &args[2].from_base64().unwrap();
			let text = &args[3].from_base64().unwrap();
			println!("{}", String::from_utf8(stream::decrypt(&text, &key_text, &fiestal::fiestal)).unwrap());
		},
		"make_key" => {
			println!("{}", &keys::create(16).to_base64(STANDARD))
		},
		_ => { println!("Unknown"); }
	}
}
