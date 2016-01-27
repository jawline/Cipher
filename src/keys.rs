pub use rand::{Rng, OsRng};

pub fn create(size: usize) -> Vec<u8> {
	OsRng::new().unwrap().gen_iter::<u8>().take(size).collect()
}