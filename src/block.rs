/**
 * Block will be able to store arbitrary blocks of bits
 */
 use std::ops::BitXor;

pub struct Block {
	bytes: Vec<u8>,
	size: usize
}

const B1: u8 = 1;
const B2: u8 = 2;
const B3: u8 = 4;
const B4: u8 = 8;
const B5: u8 = 16;
const B6: u8 = 32;
const B7: u8 = 64;
const B8: u8 = 128;

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

pub fn blocks_from_bytes(bytes: &[u8], block_size: usize) -> Vec<Block> {
	let mut current_byte = 0;
	let mut bytes_from_last_block = 0;
	let mut blocks = Vec::new();

	blocks
}

pub fn blocks_into_bytes(blocks: Vec<Block>) -> Vec<u8> {
	Vec::new()
}