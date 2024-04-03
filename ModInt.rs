mod mod_int {
    use std::ops::*;

    #[derive(Clone, Copy, Default, Debug, PartialEq)]
    pub struct ModInt<const M: u64>(u32);

    pub type ModInt998244353 = ModInt::<998244353>;
    pub type ModInt1000000007 = ModInt::<1000000007>;

    pub trait ModIntTrait: Copy + Default + Mul<Output=Self> + From<i64> {
	const ZERO: Self;
	const ONE: Self;

	fn inv(&self) -> Self;
    }

    impl<const M: u64> ModIntTrait for ModInt<M> {
	const ZERO : Self = ModInt::<M>::ZERO;
	const ONE : Self = ModInt::<M>::ONE;

        fn inv(&self) -> Self {
            self.pow(M - 2)
        }
    }

    impl<const M: u64> ModInt<M> {
	pub const ZERO: Self = Self(0);
	pub const ONE: Self = Self(1);
	pub const MOD : u64 = M;

        pub fn new(a: u64) -> Self {
            Self((a % M) as u32)
        }

	pub fn raw(a: u32) -> Self {
	    Self(a)
	}

        pub fn getu(&self) -> u32 {
            self.0
        }

        pub fn geti(&self) -> i32 {
            self.0 as i32
        }

        pub fn pow(&self, mut y: u64) -> Self {
            let mut r = Self::new(1);
            let mut x = *self;
            while y > 0 {
                if y & 1 == 1 { r *= x; }
                x *= x;
                y /= 2;
            }
            r
        }
    }

    impl<const M: u64> Add for ModInt<M> {
        type Output = Self;
        fn add(mut self, other: Self) -> Self {
            self += other;
            self
        }
    }

    impl<const M: u64> Sub for ModInt<M> {
        type Output = Self;
        fn sub(mut self, other: Self) -> Self {
            self -= other;
            self
        }
    }

    impl<const M: u64> Mul for ModInt<M> {
        type Output = Self;
        fn mul(mut self, other: Self) -> Self {
            self *= other;
            self
        }
    }

    impl<const M: u64> Div for ModInt<M> {
        type Output = Self;
        fn div(mut self, other: Self) -> Self {
            self /= other;
            self
        }
    }

    impl<const M: u64> AddAssign for ModInt<M> {
        fn add_assign(&mut self, other: Self) {
            let m = M as u32;
            self.0 += other.0;
            if self.0 >= m { self.0 -= m; }
        }
    }

    impl<const M: u64> SubAssign for ModInt<M> {
        fn sub_assign(&mut self, other: Self) {
            let m = M as u32;
            self.0 += m - other.0;
            if self.0 >= m { self.0 -= m; }
        }
    }

    impl<const M: u64> MulAssign for ModInt<M> {
        fn mul_assign(&mut self, other: Self) {
            self.0 = (self.0 as u64 * other.0 as u64 % M) as u32;
        }
    }

    impl<const M: u64> DivAssign for ModInt<M> {
        fn div_assign(&mut self, other: Self) {
            *self *= other.inv();
        }
    }

    impl<const M: u64> Neg for ModInt<M> {
	type Output = Self;
	fn neg(self) -> Self {
	    if self.0 == 0 { ModInt::ZERO } else { Self(M as u32 - self.0) }
	}
    }

    impl<const M: u64> From<i64> for ModInt<M> {
        fn from(mut a: i64) -> Self {
            let m = M as i64;
            a %= m;
            if a < 0 { a += m; }
            Self(a as u32)
        }
    }

    impl<const M: u64> From<usize> for ModInt<M> {
        fn from(a: usize) -> Self {
            Self((a as u64 % M) as u32)
        }
    }

    pub struct Fact<Mint> {
	fact: Vec<Mint>,
	fact_inv: Vec<Mint>,
    }

    // let fact = mod_int::Fact::<Mint>::new(n + 1);
    impl<Mint: ModIntTrait> Fact<Mint> {
	pub fn new(size: usize) -> Self {
	    let mut fact = vec![Mint::ZERO; size];
	    let mut fact_inv = vec![Mint::ZERO; size];
	    fact[0] = Mint::ONE;
	    for i in 1..size {
		fact[i] = fact[i-1] * Mint::from(i as i64);
	    }
	    fact_inv[size-1] = fact[size-1].inv();
	    for i in (1..size).rev() {
		fact_inv[i-1] = fact_inv[i] * Mint::from(i as i64);
	    }
	    Self {fact, fact_inv}
	}

	pub fn nck(&self, n: usize, k: usize) -> Mint {
	    self.fact[n] * self.fact_inv[k] * self.fact_inv[n-k]
	}

	pub fn fact(&self, n: usize) -> Mint {
	    self.fact[n]
	}

	pub fn fact_inv(&self, n: usize) -> Mint {
	    self.fact_inv[n]
	}
    }
}
type Mint = mod_int::ModInt998244353;
