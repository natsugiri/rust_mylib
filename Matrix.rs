mod matrix {
    use std::ops::*;
    use std::fmt::Debug;

    #[derive(Debug, Clone, Copy)]
    pub struct Array<T, const N: usize>([T; N]);

    #[derive(Debug, Clone)]
    pub struct Matrix<T, const N: usize>([[T; N]; N]);

    pub trait ElemTrait: Debug + Default + Copy + Clone + Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Mul<Output=Self> + MulAssign {}
    impl<T: Debug + Default + Copy + Clone + Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Mul<Output=Self> + MulAssign> ElemTrait for T {}

    macro_rules! gen_bin_op {
        ($( (($($types:tt)*), $type:ty, $op_trait:ident, $op:ident, $op_assign:ident) ),*) => {
            $(
                // T + T -> T;
                impl <$($types)*> $op_trait for $type {
                    type Output = $type;
                    fn $op(mut self, other: Self) -> Self::Output {
                        self.$op_assign(other);
                        self
                    }
                }
                // &T + &T -> T;
                impl <'a, $($types)*> $op_trait for &'a $type {
                    type Output = $type;
                    fn $op(self, other: Self) -> Self::Output {
                        self.clone().$op(other.clone())
                    }
                }
                // &T + T -> T;
                impl <'a, $($types)*> $op_trait<$type> for &'a $type {
                    type Output = $type;
                    fn $op(self, other: $type) -> Self::Output {
                        self.clone().$op(other)
                    }
                }
                // T + &T -> T;
                impl <'a, $($types)*> $op_trait<&'a $type> for $type {
                    type Output = $type;
                    fn $op(mut self, other: &'a $type) -> Self::Output {
                        self.$op_assign(other.clone());
                        self
                    }
                }
                )*
        }
    }

    //////////////////////////////////////////////////
    // Array
    //////////////////////////////////////////////////
    mod array {
        use std::ops::*;
        use matrix::Array;
        use matrix::ElemTrait;

        impl<T: ElemTrait, const N: usize> Default for Array<T, N> {
            fn default() -> Self {
                Self([T::default(); N])
            }
        }

        impl<T, const N: usize> Index<usize> for Array<T, N> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T, const N: usize> IndexMut<usize> for Array<T, N> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<T: ElemTrait, const N: usize> AddAssign<&Array<T, N>> for Array<T, N> {
            fn add_assign(&mut self, other: &Self) {
                for i in 0..N {
                    self.0[i] += other.0[i].clone();
                }
            }
        }

        impl<T: ElemTrait, const N: usize> AddAssign for Array<T, N> {
            fn add_assign(&mut self, other: Self) {
                *self += &other;
            }
        }

        impl<T: ElemTrait, const N: usize> SubAssign<&Array<T, N>> for Array<T, N> {
            fn sub_assign(&mut self, other: &Self) {
                for i in 0..N {
                    self.0[i] -= other.0[i].clone();
                }
            }
        }

        impl<T: ElemTrait, const N: usize> SubAssign for Array<T, N> {
            fn sub_assign(&mut self, other: Self) {
                *self -= &other;
            }
        }

        gen_bin_op!{
            ((T: ElemTrait, const N: usize), Array<T, N>, Add, add, add_assign),
            ((T: ElemTrait, const N: usize), Array<T, N>, Sub, sub, sub_assign)}
    }

    //////////////////////////////////////////////////
    // Matrix
    //////////////////////////////////////////////////
    mod matrix {
        use std::ops::*;
        use matrix::Matrix;
        use matrix::ElemTrait;

        impl<T: Default + Copy, const N: usize> Default for Matrix<T, N> {
            fn default() -> Self {
                Self([[T::default(); N]; N])
            }
        }

        impl<T, const N: usize> Index<usize> for Matrix<T, N> {
            type Output = [T];
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T, const N: usize> IndexMut<usize> for Matrix<T, N> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<T: ElemTrait, const N: usize> AddAssign<&Matrix<T, N>> for Matrix<T, N> {
            fn add_assign(&mut self, other: &Matrix<T, N>) {
                for i in 0..N {
                    for j in 0..N {
                        self[i][j] += other[i][j].clone();
                    }
                }
            }
        }

        impl<T: ElemTrait, const N: usize> AddAssign for Matrix<T, N> {
            fn add_assign(&mut self, other: Self) {
                *self += &other;
            }
        }

        impl<T: ElemTrait, const N: usize> SubAssign<&Matrix<T, N>> for Matrix<T, N> {
            fn sub_assign(&mut self, other: &Matrix<T, N>) {
                for i in 0..N {
                    for j in 0..N {
                        self[i][j] -= other[i][j].clone();
                    }
                }
            }
        }

        impl<T: ElemTrait, const N: usize> SubAssign for Matrix<T, N> {
            fn sub_assign(&mut self, other: Self) {
                *self -= &other;
            }
        }

        impl<T: ElemTrait, const N: usize> MulAssign<&Matrix<T, N>> for Matrix<T, N> {
            fn mul_assign(&mut self, other: &Self) {
                let mut d = [[T::default(); N]; N];
                for i in 0..N {
                    for j in 0..N {
                        for k in 0..N {
                            d[i][k] += self.0[i][j] * other.0[j][k];
                        }
                    }
                }
                *self = Matrix(d)
            }
        }

        impl<T: ElemTrait, const N: usize> MulAssign for Matrix<T, N> {
            fn mul_assign(&mut self, other: Self) {
                *self *= &other.clone();
            }
        }

        gen_bin_op!{
            ((T: ElemTrait, const N: usize), Matrix<T, N>, Add, add, add_assign),
            ((T: ElemTrait, const N: usize), Matrix<T, N>, Sub, sub, sub_assign),
            ((T: ElemTrait, const N: usize), Matrix<T, N>, Mul, mul, mul_assign)}
    }

    impl<T: ElemTrait, const N: usize> Mul<Array<T, N>> for Matrix<T, N> {
        type Output = Array<T, N>;
        fn mul(self, other: Array<T, N>) -> Self::Output {
            let mut d = [T::default(); N];
            for i in 0..N {
                for j in 0..N {
                    d[i] += self[i][j] * other[j];
                }
            }
            Array(d)
        }
    }

    pub mod gaussian_elimination {
        use std::ops::Div;
        use matrix::{ElemTrait, Array, Matrix};

        #[derive(Debug, Clone)]
        pub enum Operation<T> {
            SWAP(usize, usize), // swap(m[i], m[j]);
            ADD(usize, usize, T), // m[i] += m[j] * rate;
        }

        #[derive(Debug, Clone)]
        pub struct GaussianElimination<T, const N: usize> {
            a: Matrix<T, N>,
            v: Vec<Operation<T>>,
        }

        impl<T: ElemTrait + PartialEq + Div<Output=T>, const N: usize>  GaussianElimination<T, N> {
            pub fn new(mut mat: Matrix<T, N>) -> Self {
                let mut r = 0;
                let mut c = 0;
                let mut v = Vec::new();
                while r < N && c < N {
                    let mut pivot = usize::MAX;
                    for i in r..N {
                        if mat[i][c] != T::default() {
                            pivot = i;
                            break;
                        }
                    }
                    if pivot == usize::MAX {
                        c += 1;
                        continue;
                    }
                    if r < pivot {
                        v.push(Operation::SWAP(r, pivot));
                        for j in c..N {
                            let tmp = mat[r][j];
                            mat[r][j] = mat[pivot][j];
                            mat[pivot][j] = tmp;
                        }
                    }
                    for i in 0..N {
                        if i != r && mat[i][c] != T::default() {
                            let rate = T::default() - mat[i][c] / mat[r][c];
                            v.push(Operation::ADD(i, r, rate));
                            for j in c..N {
                                let tmp = mat[r][j] * rate;
                                mat[i][j] += tmp;
                            }
                        }
                    }
                    r += 1;
                    c += 1;
                }
                GaussianElimination { a: mat, v }
            }
        } // impl;
    } // mod gaussian_elimination;
} // mod matrix;
use matrix::*;

fn main() {
    let mut a = Array::<i32, 4>::default();
    a[0] = 1;
    let mut b = a + a;
    b += a;
    println!("{:?}", b);
    println!("{:?}", b[0]);
    println!("{:?}", a);
    let mut a = Array::<i32, 4>::default();
    a[0] = 1;
    let mut b = a + a;
    b += a;

    let mut a = Matrix::<i32, 3>::default();
    a += a.clone();
    a[0][0] = 1;
    a[1][1] = 2;
    a[2][2] = 3;
    a[0][1] = 4;
    a[1][2] = 5;
    a = &a + &a;
    a = a.clone() + a.clone();
    a = &a + (&a + &a) + &a;
    println!("{:?}", a);

    let ge = matrix::gaussian_elimination::GaussianElimination::new(a.clone());
    println!("{:?}", ge);
}
