mod union_find {
    pub struct UnionFind {
        cc: usize,
        a: Vec<isize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind{ cc: n, a: vec![-1; n] }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.a[x] < 0 {
                x
            } else {
                let r = self.root(self.a[x] as usize);
                self.a[x] = r as isize;
                r
            }
        }

        pub fn link(&mut self, x: usize, y: usize) {
            let mut x = self.root(x);
            let mut y = self.root(y);
            if x != y {
                if self.a[x] > self.a[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.a[x] += self.a[y];
                self.a[y] = x as isize;
                self.cc -= 1;
            }
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn count_node(&mut self, x: usize) -> usize {
            let x = self.root(x);
            (-self.a[x]) as usize
        }

        pub fn count_tree(&self) -> usize {
            self.cc
        }
    }
} // mod union_find;
