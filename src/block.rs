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

	fn from(size: usize, bytes: &[u8]) -> Block {
		Block {
			size: size,
			bytes: bytes.iter().map(|&x| x).collect()
		}
	}
}

impl BitXor for Block {
	type Output = Block;

	fn bitxor(self, rhs: Block) -> Block {
		if rhs.size != self.size {
			self
		} else {
			Block::from(self.size, &self.bytes.iter().zip(rhs.bytes.iter()).map(|(lhs, rhs)| lhs ^ rhs).collect::<Vec<u8>>())
		}
	}
}