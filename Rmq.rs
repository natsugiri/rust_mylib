mod rmq {
    use std::fmt::Debug;
    use std::ops::{Add, Sub, Range};
    use std::convert::From;

    #[derive(Debug)]
    pub struct Impl<T> {
        min_value: T,
        d: Vec<T>,
    }

    impl<T: Copy + From<i32>> From<usize> for Impl<T> {
        fn from(n: usize) -> Self {
            Self { min_value: T::from(0), d: vec![T::from(0); n] }
        }
    }

    impl<T: Debug + Copy + Sub<Output=T> + Ord + From<i32>> From<&Vec<T>> for Impl<T> {
        fn from(a: &Vec<T>) -> Self {
            if a.is_empty() {
                return Self { min_value: T::from(0), d: vec![] };
            }
            let mut d = vec![T::from(0); a.len()];
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
            println!("{:?}", d);
            Self { min_value: d[0], d }
        }
    }

    impl<T: Copy + Add<Output=T> + Sub<Output=T> + Ord + From<i32>> Impl<T> {
        pub fn new() -> Self {
            todo!();
        }

        pub fn len(&self) -> usize {
            self.d.len()
        }

        pub fn at(&mut self, i: usize) -> T {
            self.min(i..i+1)
        }

        pub fn modify(&mut self, i: usize, x: T) {
            self.add(i..i+1, x);
        }

        pub fn add(&mut self, w: Range<usize>, x: T) {
            if w == (0..self.d.len()) {
                self.min_value = self.min_value + x;
                return;
            }
            let d = &mut self.d;
            d[0] = T::from(0);
            let mut ladd = T::from(0);
            let mut radd = T::from(0);
            let mut l = w.start + d.len();
            let mut r = w.end + d.len();
            while l < r || r % 2 == 0 {
                if l % 2 == 1 {
                    l += 1;
                    ladd = Self::add_helper(&mut d[l/2-1], ladd, x);
                } else {
                    ladd = Self::add_helper(&mut d[l/2-1], T::from(0), ladd);
                }
                if r % 2 == 1 {
                    radd = Self::add_helper(&mut d[r/2], x, radd);
                } else {
                    radd = Self::add_helper(&mut d[r/2], radd, T::from(0));
                }
                l /= 2;
                r /= 2;
            }
            if r > 1 {
                radd = Self::add_helper(&mut d[r/2], ladd, radd);
                r /= 2;
                while r > 1 {
                    if r % 2 == 1 {
                        radd = Self::add_helper(&mut d[r/2], T::from(0), radd);
                    } else {
                        radd = Self::add_helper(&mut d[r/2], radd, T::from(0));
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
            let diff = *dst;
            *dst = *dst + ladd - radd;
            if diff > 0.into() {
                if *dst > 0.into() { radd } else { ladd + diff }
            } else {
                if *dst > 0.into() { radd - diff } else { ladd }
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
                    lval = Self::eval(lval, Some(T::from(0)), d[l/2-1]);
                } else if lval.is_some() {
                    lval = Self::apply_minus(lval, d[l/2-1]);
                }
                if r & 1 == 1 {
                    rval = Self::eval(Some(T::from(0)), rval, d[r/2]);
                } else if rval.is_some() {
                    rval = Self::apply_plus(rval, d[r/2]);
                }
                l /= 2;
                r /= 2;
            }
            if r == 1 {
                rval = Self::eval(lval, rval, T::from(0));
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
                Some(x) if diff > T::from(0_i32) => Some(x + diff),
                _ => lval,
            }
        }

        fn apply_minus(rval: Option<T>, diff: T) -> Option<T> {
            match rval {
                Some(x) if diff < T::from(0_i32) => Some(x - diff),
                _ => rval,
            }
        }
    }
}

fn main() {
    let mut a: rmq::Impl<i32> = rmq::Impl::<i32>::from(&vec![0,1,2,3,4]);
    a.add(0..2, 3);
    println!("{:?}", a);
    for i in 0..a.len() {
        for j in (i+1)..=a.len() {
            print!("{} ", a.min(i..j));
        }
        println!("");
    }
}
