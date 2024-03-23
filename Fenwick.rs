mod fenwick {
    use std::ops::{ Sub, AddAssign };

    pub struct Fenwick<T> { d: Vec<T> }

    impl<T> Fenwick<T> where T: Default + Copy + AddAssign {
	pub fn new(n: usize) -> Self {
	    Self { d: vec![T::default(); n] }
	}

	pub fn add(&mut self, mut i: usize, val: T) {
	    while i < self.d.len() {
		self.d[i] += val;
		i |= i + 1;
	    }
	}

	pub fn prefix_sum(&self, mut r: usize) -> T {
	    let mut ret = T::default();
	    while 0 < r {
		ret += self.d[r-1];
		r &= r - 1;
	    }
	    ret
	}

	pub fn sum(&self, l: usize, r: usize) -> T where T : Sub<Output=T> {
	    self.prefix_sum(r) - self.prefix_sum(l)
	}
    }
}
