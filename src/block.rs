/**
 * Block will be able to store arbitrary blocks of bits
 */
 use std::ops::BitXor;

struct Block {
	bytes: Vec<u8>,
	size: usize
}

impl Block {
	pub fn new(size: usize) -> Block {
		Block {
			bytes: Vec::new(),
			size: size
		}
	}
	
	fn set(&mut self, bytes: &[u8]) {
		self.bytes = bytes.iter().map(|&x| x).collect();
	}
}

impl BitXor for Block {
	type Output = Block;

	fn bitxor(self, rhs: Block) -> Block {
		if rhs.size != self.size {
			self
		} else {
			let mut newBytes = Vec::new();
			for i in 0..self.bytes.len() {
				newBytes.push(self.bytes[i] ^ rhs.bytes[i]);
			}

			let mut result = Block::new(self.size);
			result.set(&newBytes);
			result
		}
	}
}