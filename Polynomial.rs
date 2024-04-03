mod polynomial {
    use crate::mod_int::{ModInt998244353, ModIntTrait};
    type Mint = ModInt998244353;
    use std::ops::*;

    #[derive(Debug)]
    pub struct Polynomial(Vec<Mint>);

    impl Polynomial {
	pub fn new() -> Self {
	    Self(Vec::new())
	}

	pub fn from(mut v: Vec<Mint>) -> Self {
	    while Some(&Mint::ZERO) == v.last() { v.pop(); }
	    Self(v)
	}

	pub fn is_zero(&self) -> bool {
	    for &m in &self.0 {
		if m != Mint::ZERO {
		    return false;
		}
	    }
	    true
	}

	pub fn get(&self, i: usize) -> Mint {
	    if i < self.0.len() {
		self.0[i]
	    } else {
		Mint::ZERO
	    }
	}

	pub fn set(&mut self, i: usize, m: Mint) {
	    while self.0.len() <= i {
		self.0.push(Mint::ZERO);
	    }
	    self.0[i] = m;
	}

	pub fn add(&mut self, i: usize, m: Mint) {
	    while self.0.len() <= i {
		self.0.push(Mint::ZERO);
	    }
	    self.0[i] += m;
	}

	pub fn normal(&mut self) {
	    while Some(&Mint::ZERO) == self.0.last() {
		self.0.pop();
	    }
	}

	pub fn inv(&self, k: usize) -> Polynomial {
	    if self.0.is_empty() || self.0[0] == Mint::ZERO {
		return Polynomial::new();
	    }
	    let mut z = Polynomial::new();
	    z.set(0, Mint::ONE / self.0[0]);
	    let mut w = 1;
	    while w < k {
		let tmp = (&z * &self.mod_xk(w * 2)).substr(w, w * 2);
		z -= &(&z * &tmp).mod_xk(w * 2).mul_xk(w);
		w += w;
	    }
	    z.normal();
	    z.mod_xk(k)
	}

	fn mul_xk(&self, k: usize) -> Polynomial {
	    if self.0.is_empty() {
		self.clone()
	    } else {
		let mut v = vec![Mint::ZERO; k];
		v.append(&mut self.0.clone());
		Polynomial(v)
	    }
	}

	pub fn mod_xk(&self, k: usize) -> Polynomial {
	    if self.0.len() <= k {
		self.clone()
	    } else {
		Polynomial(self.0[..k].to_vec())
	    }
	}

	fn substr(&self, l: usize, mut r: usize) -> Polynomial {
	    r = r.min(self.0.len());
	    if r <= l {
		Polynomial::new()
	    } else {
		Polynomial(self.0[l..r].to_vec())
	    }
	}

	pub fn log(&self, k: usize) -> Polynomial {
	    assert!(self.get(0) == Mint::ONE);
	    if k <= 1 {
		Polynomial::new()
	    } else {
		(&self.deriv() * &self.inv(k)).mod_xk(k - 1).integr()
	    }
	}

	pub fn deriv(&self) -> Polynomial {
	    if self.0.len() <= 1 { return Polynomial::new(); }
	    Polynomial((1..self.0.len()).map(|i| self.0[i] * Mint::raw(i as u32)).collect())
	}

	pub fn integr(&self) -> Polynomial {
	    if self.0.is_empty() { return Polynomial::new(); }
	    let mut v = vec![Mint::ZERO; self.0.len() + 1];
	    for i in 0..self.0.len() { v[i + 1] = self.0[i] / Mint::raw((i + 1) as u32); }
	    Polynomial(v)
	}

	pub fn exp(&self, k: usize) -> Polynomial {
	    assert!(self.get(0) == Mint::ZERO);
	    if k == 0 {
		Polynomial::new()
	    } else {
		let one = Polynomial::from(vec![Mint::ONE]);
		if self.0.is_empty() {
		    one
		} else {
		    let mut z = one.clone();
		    let mut w = 1;
		    while w < k {
			w += w;
			z = (z.clone() * (self.mod_xk(w) - z.log(w) + one.clone())).mod_xk(w);
		    }
		    z.mod_xk(k)
		}
	    }
	}

	pub fn pow(&self, y: u64, k: usize) -> Polynomial {
	    let mut t = self.0.len();
	    for i in 0..self.0.len() {
		if self.0[i] != Mint::ZERO {
		    t = i;
		    break;
		}
	    }
	    if t == self.0.len() { return Polynomial::new(); }
	    let alpha = self.0[t];
	    let alpha_inv = alpha.inv();
	    if t != 0 && y >= (k as u64) / (t as u64) { return Polynomial::new(); }
	    let yt = (y * (t as u64)) as usize;
	    let w = k - yt;
	    (((self.substr(t, self.0.len()).mod_xk(w) * alpha_inv).log(w) * Mint::new(y)).exp(w) * alpha.pow(y)).mul_xk(yt)


	}
    }

    impl Clone for Polynomial {
	fn clone(&self) -> Polynomial {
	    Self(self.0.to_vec())
	}
    }

    impl Add for Polynomial {
	type Output = Polynomial;
	fn add(self, other: Self) -> Polynomial {
	    &self + &other
	}
    }

    impl Add for &Polynomial {
	type Output = Polynomial;
	fn add(self, other: Self) -> Polynomial {
	    let mut v = vec![Mint::ZERO; self.0.len().max(other.0.len())];
	    for i in 0..self.0.len() { v[i] = self.0[i]; }
	    for i in 0..other.0.len() { v[i] += other.0[i]; }
	    Polynomial::from(v)
	}
    }

    impl AddAssign<&Polynomial> for Polynomial {
	fn add_assign(&mut self, other: &Self) {
	    while self.0.len() < other.0.len() { self.0.push(Mint::ZERO); }
	    for i in 0..other.0.len() { self.0[i] += other.0[i]; }
	}
    }

    impl Sub for Polynomial {
	type Output = Polynomial;
	fn sub(self, other: Self) -> Polynomial {
	    &self - &other
	}
    }

    impl Sub for &Polynomial {
	type Output = Polynomial;
	fn sub(self, other: Self) -> Polynomial {
	    let mut v = vec![Mint::ZERO; self.0.len().max(other.0.len())];
	    for i in 0..self.0.len() { v[i] = self.0[i]; }
	    for i in 0..other.0.len() { v[i] -= other.0[i]; }
	    Polynomial::from(v)
	}
    }

    impl SubAssign<&Polynomial> for Polynomial {
	fn sub_assign(&mut self, other: &Self) {
	    while self.0.len() < other.0.len() { self.0.push(Mint::ZERO); }
	    for i in 0..other.0.len() { self.0[i] -= other.0[i]; }
	}
    }

    impl Mul for Polynomial {
	type Output = Polynomial;
	fn mul(self, other: Self) -> Polynomial {
	    convolution(&self, &other)
	}
    }

    impl Mul for &Polynomial {
	type Output = Polynomial;
	fn mul(self, other: Self) -> Polynomial {
	    convolution(self, other)
	}
    }

    impl MulAssign<&Polynomial> for Polynomial {
	fn mul_assign(&mut self, other: &Self) {
	    *self = convolution(self, other);
	}
    }

    impl Mul<Mint> for Polynomial {
	type Output = Polynomial;
	fn mul(self, other: Mint) -> Self {
	    &self * other
	}
    }

    impl Mul<Mint> for &Polynomial {
	type Output = Polynomial;
	fn mul(self, other: Mint) -> Polynomial {
	    if other == Mint::ZERO {
		Polynomial::new()
	    } else {
		Polynomial::from(self.0.iter().map(|&x| x * other).collect())
	    }
	}
    }

    impl MulAssign<Mint> for Polynomial {
	fn mul_assign(&mut self, other: Mint) {
	    *self = &*self * other;
	}
    }

    impl Neg for Polynomial {
	type Output = Self;
	fn neg(self) -> Self {
	    Polynomial::from(self.0.iter().map(|&x| -x).collect())
	}
    }

    pub fn ntt_impl(x: &mut Vec<Mint>, e: &Vec<Mint>) {
	assert!((Mint::MOD - 1) % x.len() as u64 == 0);
	let n = x.len();
	let y = &mut vec![Mint::ZERO; n];
	let mut nn = n;
	let mut s = 1;
	while nn > 1 {
	    let m = nn / 2;
	    for p in 0..m {
		let wp = e[p*(n/nn)];
		for q in 0..s {
		    let a = x[q+s*p];
		    let b = x[q+s*(p+m)];
		    y[q+s*(p+p)] = a + b;
		    y[q+s*(p+p+1)] = (a - b) * wp;
		}
	    }
	    nn = m;
	    s += s;
	    std::mem::swap(x, y);
	}
    }

    fn convolution_impl(mut x: Vec<Mint>, mut y: Vec<Mint>) -> Vec<Mint> {
	let len = {
	    let mut a = 1;
	    while a < x.len() + y.len() { a += a; }
	    a
	};
	assert!((Mint::MOD - 1) % len as u64 == 0);
	while x.len() < len { x.push(Mint::ZERO); }
	while y.len() < len { y.push(Mint::ZERO); }

	let rate = Mint::raw(3).pow((Mint::MOD - 1) / len as u64);
	let rate_inv = rate.inv();
	let mut e = vec![Mint::ONE; len];
	let mut er = vec![Mint::ONE; len];
	for i in 0..x.len()-1 {
	    e[i+1] = e[i] * rate;
	    er[i+1] = er[i] * rate_inv;
	}
	ntt_impl(&mut x, &e);
	ntt_impl(&mut y, &e);
	for i in 0..len { x[i] *= y[i]; }
	ntt_impl(&mut x, &er);
	let d = Mint::new(len as u64).inv();
	x.iter_mut().for_each(|a| *a *= d);
	while !x.is_empty() && x.last().unwrap().geti() == 0 { x.pop(); }
	x
    }

    fn convolution(x: &Polynomial, y: &Polynomial) -> Polynomial {
	if x.is_zero() || y.is_zero() {
	    Polynomial::new()
	} else {
	    Polynomial::from(convolution_impl(x.0.to_vec(), y.0.to_vec()))
	}
    }

    pub mod product {
	use super::Polynomial;
	use std::cmp::Ordering;

	#[derive(Debug)]
	struct Entry(Polynomial);

	impl Entry {
	    fn len(&self) -> usize { self.0.0.len() }
	}

	impl PartialOrd for Entry {
	    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
	}

	impl PartialEq for Entry {
	    fn eq(&self, other: &Self) -> bool { false }
	}

	impl Eq for Entry {}

	impl Ord for Entry {
	    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// Reversed;
		other.len().cmp(&self.len()) 
	    }
	}

	#[derive(Debug)]
	pub struct Product {
	    heap: std::collections::BinaryHeap::<Entry>,
	}

	impl Product {
	    pub fn new() -> Self {
		Self { heap: std::collections::BinaryHeap::new() }
	    }

	    pub fn push(&mut self, p: Polynomial) {
		self.heap.push(Entry(p));
	    }

	    pub fn product_mod_xk(&mut self, k: usize) -> Polynomial {
		loop {
		    if let Some(p) = self.heap.pop() {
			if let Some(q) = self.heap.pop() {
			    self.heap.push(Entry((p.0 * q.0).mod_xk(k)));
			} else {
			    return p.0;
			}
		    } else {
			let mut p = Polynomial::new();
			p.set(0, super::Mint::ONE);
			p.mod_xk(k);
			return p;
		    }
		}
	    }
	}
    }
}
