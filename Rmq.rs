mod rmq {
    use std::fmt::Debug;
    use std::ops::{Add, Sub, Neg, Range};
    use std::convert::From;

    pub trait NumTrait: Debug + Copy + Default + Ord + Add<Output=Self> + Sub<Output=Self> { }
    impl<T: Debug + Copy + Default + Ord + Add<Output=Self> + Sub<Output=Self>> NumTrait for T {}

    #[derive(Debug)]
    // d[i]: {左の最小値}-{右の最小値}
    pub struct Rmq<T: NumTrait> {
	d: Vec<T>,
    }

    impl<T: NumTrait> From<usize> for Rmq<T> {
	fn from(n: usize) -> Self {
	    Self { d: vec![T::default(); n] }
	}
    }

    impl<T: NumTrait> From<&Vec<T>> for Rmq<T> {
	fn from(a: &Vec<T>) -> Self {
	    if a.is_empty() {
		return Self { d: vec![] };
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
	    Self { d }
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
		self.d[0] = self.d[0] + x;
		return;
	    }
	    let d = &mut self.d;
	    let mut ladd = T::default();
	    let mut radd = T::default();
	    let mut l = w.start + d.len();
	    let mut r = w.end + d.len();
	    while l > 2 || r > 1 {
		if l % 2 == 1 {
		    ladd = Self::add_helper(&mut d[l/2], ladd, if l < r { x } else { T::default() });
		} else if l > 2 {
		    ladd = Self::add_helper(&mut d[l/2-1], T::default(), ladd);
		}
		if r % 2 == 1 {
		    radd = Self::add_helper(&mut d[r/2], if l < r { x } else { T::default() }, radd);
		} else if r < d.len() * 2 {
		    radd = Self::add_helper(&mut d[r/2], radd, T::default());
		}
		l = (l + 1) / 2;
		r /= 2;
	    }
	    self.d[0] = self.d[0] + ladd + radd;
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
		return self.d[0];
	    }
	    let d = &self.d;
	    let mut lval: Option<T> = None;
	    let mut rval: Option<T> = None;
	    let mut l = w.start + d.len();
	    let mut r = w.end + d.len();
	    while l > 2 || r > 1 {
		if l & 1 == 1 {
		    lval = Self::eval(d[l/2], lval, if l<r {Some(T::default())} else {None});
		} else if l > 2 {
		    lval = Self::eval(d[l/2-1], None, lval);
		}
		if r & 1 == 1 {
		    rval = Self::eval(d[r/2], if l<r {Some(T::default())} else {None}, rval);
		} else if rval.is_some() {
		    rval = Self::eval(d[r/2], rval, None);
		}
		l = (l+1)/2;
		r /= 2;
	    }
	    self.d[0] + match (lval, rval) {
		(None, None) => T::default(),
		(Some(x), None) => x,
		(None, Some(y)) => y,
		(Some(x), Some(y)) => x.min(y),
	    }
	}

	fn eval(diff: T, mut lval: Option<T>, mut rval: Option<T>) -> Option<T> {
	    if diff > T::default() && lval.is_some() { lval = Some(lval.unwrap() + diff); }
	    if diff < T::default() && rval.is_some() { rval = Some(rval.unwrap() - diff); }
	    match (lval, rval) {
		(Some(x), Some(y)) => Some(x.min(y)),
		_ => lval.or(rval),
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
	    let b = a.iter().map(|&x| -x).collect();
	    Self(Rmq::<T>::from(&b))
	}
    }

    impl<T: NumTrait + Neg<Output=T>> Rxq<T> {
	pub fn new() -> Self { Self::from(0) }

	pub fn get(&mut self, i: usize) -> T {
	    -self.0.get(i)
	}

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
