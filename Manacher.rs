mod manacher {
    #[derive(Debug, Default)]
    pub struct Manacher {
	rad: Vec<usize>,
    }

    impl Manacher {
	pub fn new<T: Eq>(s: &Vec<T>) -> Self {
	    let n = s.len();
	    let mut rad = vec![0; 2*n];
	    let mut i = 0_usize;
	    let mut j = 0_usize;
	    while i < 2*n {
		while i >= j && i+j+1 < 2*n && s[(i-j)/2] == s[(i+j+1)/2] {
		    j += 1;
		}
		rad[i] = j;
		let mut k = 1;
		while i >= k && rad[i] >= k && rad[i-k] != rad[i] - k {
		    rad[i+k] = rad[i-k].min(rad[i]-k);
		    k += 1;
		}
		i += k;
		j = if j > k { j - k } else { 0 };
	    }
	    Self { rad }
	}

	pub fn ok(&self, w: std::ops::Range<usize>) -> bool {
	    let (l, r) = (w.start, w.end);
	    r-l <= self.rad[l+r-1]
	}
    }
} // mod manacher;
