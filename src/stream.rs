fn next_idx(last_idx: usize, key: &[u8]) -> usize {
	let new_idx = last_idx + 1;
	if new_idx >= key.len() { 0 } else { new_idx }
}

pub fn encrypt(message: &[u8], key: &[u8], cipher: &Fn(u8, &[u8], usize) -> u8) -> Vec<u8> {

	let mut previous_block: u8 = 0;
	let mut last_idx = 0;

	message.iter().map(|&x| {
		let mut current_block = x ^ previous_block;

		current_block = cipher(current_block, key, last_idx);
		last_idx = next_idx(last_idx, key);

		previous_block = current_block;
		current_block
	}).collect()
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], cipher: &Fn(u8, &[u8], usize) -> u8) -> Vec<u8> {

	let mut previous_block: u8 = 0;
	let mut last_idx = 0;

	ciphertext.iter().map(|&x| {

		let mut current_block = x;
		current_block = cipher(current_block, key, last_idx);
		last_idx = next_idx(last_idx, key);

		current_block = current_block ^ previous_block;
		previous_block = x;
		current_block
	}).collect()
}