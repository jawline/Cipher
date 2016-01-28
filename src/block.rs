/**
 * Block will be able to store arbitrary blocks of bits
 */

struct Block {
	bytes: Vec<u8>,
	size: usize
}

impl Block {
	pub fn new(size: usize) -> Block {
		Block{
			Vec::new(),
			size
		}
	}
	
	fn set(&mut self, bytes: &[u8]) {
	}
}

impl BitXor for Block {
	fn bitxor(self, rhs: Block) -> Block {
		self.bytes
	}
}