mod segtree {
    use std::ops::{ Fn, RangeBounds };
    use std::clone::Clone;

    #[derive(Debug)]
    pub struct SegTreeLazy<Seg, Lazy, Join, Raise, Compose> {
	seg: Vec<Seg>,
	lazy: Vec<Lazy>,
	default_seg: Seg,
	default_lazy: Lazy,
	join: Join,
	raise: Raise,
	compose: Compose,
    }

    impl <Seg, Lazy, Join, Raise, Compose>SegTreeLazy<Seg, Lazy, Join, Raise, Compose>
	where Seg: Clone,
	      Lazy: Clone,
	      Join: Fn(&Seg, &Seg) -> Seg,
	      Raise: Fn(&Seg, &Lazy) -> Seg,
	      Compose: Fn(&Lazy, &Lazy) -> Lazy,
    {
	pub fn new(n: usize, default_seg: Seg, default_lazy: Lazy, join: Join, raise: Raise, compose: Compose) -> Self {
	    let m = Self::ceil_power_2(n);
	    Self {
		seg: vec![default_seg.clone(); m * 2],
		lazy: vec![default_lazy.clone(); m * 2],
		default_seg,
		default_lazy,
		join,
		raise,
		compose }
	}

	pub fn from_vec(seg: &Vec<Seg>, default_seg: Seg, default_lazy: Lazy, join: Join, raise: Raise, compose: Compose) -> Self {
	    let m = Self::ceil_power_2(seg.len());
	    let mut a = vec![default_seg.clone(); m * 2];
	    for i in 0..seg.len() { a[m + i] = seg[i].clone(); }
	    for i in (1..m).rev() { a[i] = join(&a[i*2], &a[i*2+1]); }
	    Self {
		seg: a,
		lazy: vec![default_lazy.clone(); m * 2],
		default_seg,
		default_lazy,
		join,
		raise,
		compose }
	}

	fn ceil_power_2(n: usize) -> usize {
	    if n <= 1 {
		1
	    } else {
		1 << (usize::BITS - (n-1).leading_zeros())
	    }
	}

	fn force(&mut self, k: usize) {
	    if k * 2 < self.lazy.len() {
		self.lazy[k*2] = (self.compose)(&self.lazy[k*2], &self.lazy[k]);
		self.lazy[k*2+1] = (self.compose)(&self.lazy[k*2+1], &self.lazy[k]);
		self.seg[k] = (self.join)(&self.eval_node(k * 2), &self.eval_node(k * 2 + 1));
		self.lazy[k] = self.default_lazy.clone();
	    }
	}

	fn force_down(&mut self, i: usize) {
	    let t = usize::BITS - 1 - self.seg.len().leading_zeros();
	    let k = i + self.seg.len() / 2;
	    for s in (1..t).rev() { self.force(k >> s); }
	    self.seg[k] = (self.raise)(&self.seg[k], &self.lazy[k]);
	    self.lazy[k] = self.default_lazy.clone();
	}

	pub fn eval_node(&self, k: usize) -> Seg {
	    (self.raise)(&self.seg[k], &self.lazy[k])
	}

	pub fn get(&self, i: usize) -> Seg {
	    let mut k = i + self.seg.len() / 2;
	    let mut z = self.seg[k].clone();
	    while k > 0 {
		z = (self.raise)(&z, &self.lazy[k]);
		k /= 2;
	    }
	    z
	}

	pub fn modify(&mut self, i: usize, value: Seg) -> Seg {
	    self.force_down(i);
	    let mut k = i + self.seg.len() / 2;
	    let z = self.seg[k].clone();
	    self.seg[k] = value;
	    k /= 2;
	    while k > 0 {
		self.seg[k] = (self.join)(&self.eval_node(k * 2), &self.eval_node(k * 2 + 1));
		k /= 2;
	    }
	    z
	}

	pub fn sum<R: RangeBounds<usize>>(&mut self, range: R) -> Seg {
	    let (l, r) = self.bounds(range);
	    if l >= r { return self.default_seg.clone(); }
	    self.force_down(l);
	    if l + 1 < r { self.force_down(r - 1); }
	    let (mut l, mut r) = (l + self.seg.len() / 2, r + self.seg.len() / 2);
	    let (mut seg_l, mut seg_r) = (self.default_seg.clone(), self.default_seg.clone());
	    while l < r {
		if (l & 1) == 1 {
		    seg_l = (self.join)(&seg_l, &self.eval_node(l));
		    l += 1;
		}
		if (r & 1) == 1 {
		    r -= 1;
		    seg_r = (self.join)(&self.eval_node(r), &seg_r);
		}
		(l, r) = (l / 2, r / 2);
	    }
	    (self.join)(&seg_l, &seg_r)
	}

	pub fn add<R: RangeBounds<usize>>(&mut self, range: R, after: Lazy) {
	    let (l, r) = self.bounds(range);
	    if l >= r { return; }
	    self.force_down(l);
	    if l + 1 < r { self.force_down(r - 1); }
	    {
		let (mut l, mut r) = (l + self.seg.len() / 2, r + self.seg.len() / 2);
		while l < r {
		    if (l & 1) == 1 {
			self.lazy[l] = (self.compose)(&self.lazy[l], &after);
			l += 1;
		    }
		    if (r & 1) == 1 {
			r -= 1;
			self.lazy[r] = (self.compose)(&self.lazy[r], &after);
		    }
		    (l, r) = (l / 2, r / 2);
		}
	    }
	    {
		let mut l = (l + self.seg.len() / 2) / 2;
		while l > 0 {
		    self.seg[l] = (self.join)(&self.eval_node(l * 2), &self.eval_node(l * 2 + 1));
		    l /= 2;
		}
		let mut r = (r - 1 + self.seg.len() / 2) / 2;
		while r > 0 {
		    self.seg[r] = (self.join)(&self.eval_node(r * 2), &self.eval_node(r * 2 + 1));
		    r /= 2;
		}
	    }
	}

	fn bounds<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
	    use std::ops::Bound::{Included, Excluded, Unbounded};
	    (
		match range.start_bound() {
		    Included(&l) => l,
		    Excluded(&l) => l + 1,
		    Unbounded => 0,
		},
		match range.end_bound() {
		    Included(&r) => r + 1,
		    Excluded(&r) => r,
		    Unbounded => self.seg.len() / 2,
		})
	}
    }
}
