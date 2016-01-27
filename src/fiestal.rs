	pub fn fiestal(item: u8, key: &[u8], index: usize) -> u8 {
		item ^ key[index % (key.len() - 1)]
	}