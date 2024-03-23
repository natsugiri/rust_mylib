mod bitset {
    #[derive(Clone, Debug)]
    pub struct BitSet {
	b: Vec<u64>,
    }

    impl BitSet {
	fn new(n: usize) -> Self {
	    BitSet { b: vec![0; n] }
	}

	pub fn with_bits(bits: usize) -> Self {
	    Self::new((bits + 63) / 64)
	}

	pub fn test(&self, i: usize) -> bool {
	    (self.b[i/64] >> (i%64) & 1) == 1
	}

	pub fn set(&mut self, i: usize) {
	    self.b[i/64] |= 1 << (i%64);
	}

	pub fn reset(&mut self, i: usize) {
	    self.b[i/64] &= !(1 << (i%64));
	}
    }

    use std::ops::{BitOr, BitAnd, BitXor, BitOrAssign, BitAndAssign, BitXorAssign};

    impl BitOr for &BitSet {
	type Output = BitSet;
	fn bitor(self, other: &BitSet) -> BitSet {
	    BitSet { b: (0..self.b.len()).map(|i| self.b[i] | other.b[i]).collect() }
	}
    }

    impl BitAnd for &BitSet {
	type Output = BitSet;
	fn bitand(self, other: &BitSet) -> BitSet {
	    BitSet { b: (0..self.b.len()).map(|i| self.b[i] & other.b[i]).collect() }
	}
    }

    impl BitXor for &BitSet {
	type Output = BitSet;
	fn bitxor(self, other: &BitSet) -> BitSet {
	    BitSet { b: (0..self.b.len()).map(|i| self.b[i] ^ other.b[i]).collect() }
	}
    }

    impl BitOrAssign<&BitSet> for BitSet {
	fn bitor_assign(&mut self, other: &Self) {
	    for i in 0..self.b.len() {
		self.b[i] |= other.b[i];
	    }
	}
    }

    impl BitAndAssign<&BitSet> for BitSet {
	fn bitand_assign(&mut self, other: &Self) {
	    for i in 0..self.b.len() {
		self.b[i] &= other.b[i];
	    }
	}
    }

    impl BitXorAssign<&BitSet> for BitSet {
	fn bitxor_assign(&mut self, other: &Self) {
	    for i in 0..self.b.len() {
		self.b[i] ^= other.b[i];
	    }
	}
    }
}
