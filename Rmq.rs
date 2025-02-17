mod rmq {
    use std::fmt::Debug;
    use std::ops::{Add, Sub, Neg, Range};
    use std::convert::From;

    pub trait NumTrait: Debug + Copy + Default + Ord + Add<Output=Self> + Sub<Output=Self> { }
    impl<T: Debug + Copy + Default + Ord + Add<Output=Self> + Sub<Output=Self>> NumTrait for T {}

    #[derive(Debug)]
    // d[i]: {左の最小値}-{右の最小値}
    pub struct Rmq<T: NumTrait> {
	min_value: T,
	d: Vec<T>,
    }

    impl<T: NumTrait> From<usize> for Rmq<T> {
	fn from(n: usize) -> Self {
	    Self { min_value: T::default(), d: vec![T::default(); n] }
	}
    }

    impl<T: NumTrait> From<&Vec<T>> for Rmq<T> {
	fn from(a: &Vec<T>) -> Self {
	    if a.is_empty() {
		return Self { min_value: T::default(), d: vec![] };
	    }
	    let mut d = vec![T::default(); a.len()];
	    let mut m = 1;
	    while m < a.len() { m += m; }
	    let mut j = m - 1;
	    for _ in 0..a.len() {
		if j < a.len() { j += a.len(); }
		let mut k = j;
		let mut val = a[j - a.len()];
		while k % 2 == 0 {
		    k /= 2;
		    let tmp = val.min(d[k]);
		    d[k] = val - d[k];
		    val = tmp;
		}
		d[k / 2] = val;
		j -= 1;
	    }
	    Self { min_value: d[0], d }
	}
    }

    impl<T: NumTrait> Rmq<T> {
	pub fn new() -> Self { Self::from(0) }

	pub fn get(&mut self, i: usize) -> T {
	    self.min(i..i+1)
	}

	pub fn modify(&mut self, i: usize, x: T) {
	    let tmp = self.get(i);
	    self.add(i..i+1, x - tmp);
	}

	pub fn add(&mut self, w: Range<usize>, x: T) {
	    if w == (0..self.d.len()) {
		self.min_value = self.min_value + x;
		return;
	    }
	    let d = &mut self.d;
	    let mut ladd = T::default();
	    let mut radd = T::default();
	    let mut l = w.start + d.len();
	    let mut r = w.end + d.len();
	    while l < r || r % 2 == 0 {
		if l % 2 == 1 {
		    l += 1;
		    ladd = Self::add_helper(&mut d[l/2-1], ladd, x);
		} else {
		    ladd = Self::add_helper(&mut d[l/2-1], T::default(), ladd);
		}
		if r % 2 == 1 {
		    radd = Self::add_helper(&mut d[r/2], x, radd);
		} else if r < d.len() * 2 {
		    radd = Self::add_helper(&mut d[r/2], radd, T::default());
		}
		l /= 2;
		r /= 2;
	    }
	    if r == 1 {
		radd = ladd.min(radd);
	    } else {
		radd = Self::add_helper(&mut d[r/2], ladd, radd);
		r /= 2;
		while r > 1 {
		    if r % 2 == 1 {
			radd = Self::add_helper(&mut d[r/2], T::default(), radd);
		    } else {
			radd = Self::add_helper(&mut d[r/2], radd, T::default());
		    }
		    r /= 2;
		}
	    }
	    assert!(r == 1);
	    self.min_value = self.min_value + radd;
	}

	// ladd: 左の子の増加量
	// radd: 右の子の増加量
	// dst: 現在の{左の最小値}-{右の最小値}を新しい値に更新
	// return: 最小値の増加量
	fn add_helper(dst: &mut T, ladd: T, radd: T) -> T {
	    let diff: T = *dst;
	    *dst = *dst + ladd - radd;
	    if diff > T::default() {
		if *dst > T::default() { radd } else { ladd + diff }
	    } else {
		if *dst > T::default() { radd - diff } else { ladd }
	    }
	}

	pub fn min(&mut self, w: Range<usize>) -> T {
	    if w == (0..self.d.len()) {
		return self.min_value;
	    }
	    let d = &self.d;
	    let mut lval: Option<T> = None;
	    let mut rval: Option<T> = None;
	    let mut l = w.start + d.len();
	    let mut r = w.end + d.len();
	    while l < r || r % 2 == 0 {
		if l & 1 == 1 {
		    l += 1;
		    lval = Self::eval(lval, Some(T::default()), d[l/2-1]);
		} else if lval.is_some() {
		    lval = Self::apply_minus(lval, d[l/2-1]);
		}
		if r & 1 == 1 {
		    rval = Self::eval(Some(T::default()), rval, d[r/2]);
		} else if rval.is_some() {
		    rval = Self::apply_plus(rval, d[r/2]);
		}
		l /= 2;
		r /= 2;
	    }
	    if r == 1 {
		rval = Self::eval(lval, rval, T::default());
	    } else {
		rval = Self::eval(lval, rval, d[r/2]);
		r /= 2;
		while r > 1 {
		    if r & 1 == 1 {
			rval = Self::apply_minus(rval, d[r/2]);
		    } else {
			rval = Self::apply_plus(rval, d[r/2]);
		    }
		    r /= 2;
		}
	    }
	    match rval {
		Some(val) => val + self.min_value,
		None => self.min_value,
	    }
	}

	fn eval(mut lch: Option<T>, mut rch: Option<T>, diff: T) -> Option<T> {
	    lch = Self::apply_plus(lch, diff);
	    rch = Self::apply_minus(rch, diff);
	    match (lch, rch) {
		(Some(x), Some(y)) => Some(x.min(y)),
		_ => lch.or(rch),
	    }
	}

	fn apply_plus(lval: Option<T>, diff: T) -> Option<T> {
	    match lval {
		Some(x) if diff > T::default() => Some(x + diff),
		_ => lval,
	    }
	}

	fn apply_minus(rval: Option<T>, diff: T) -> Option<T> {
	    match rval {
		Some(x) if diff < T::default() => Some(x - diff),
		_ => rval,
	    }
	}
    }

    #[derive(Debug)]
    pub struct Rxq<T: NumTrait + Neg<Output=T>>(Rmq<T>);

    impl<T: NumTrait + Neg<Output=T>> From<usize> for Rxq<T> {
	fn from(n: usize) -> Self {
	    Self(Rmq::<T>::from(n))
	}
    }

    impl<T: NumTrait + Neg<Output=T>> From<&Vec<T>> for Rxq<T> {
	fn from(a: &Vec<T>) -> Self {
	    Self(Rmq::<T>::from(a))
	}
    }

    impl<T: NumTrait + Neg<Output=T>> Rxq<T> {
	pub fn add(&mut self, w: Range<usize>, x: T) {
	    self.0.add(w, -x);
	}

	pub fn max(&mut self, w: Range<usize>) -> T {
	    -self.0.min(w)
	}

	pub fn modify(&mut self, i: usize, x: T) {
	    self.0.modify(i, -x);
	}
    }
}
