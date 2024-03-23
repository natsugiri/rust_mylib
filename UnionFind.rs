struct UnionFind {
    a: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind{ a: vec![-1; n] }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.a[x] < 0 {
            x
        } else {
            let r = self.root(self.a[x] as usize);
            self.a[x] = r as i32;
            r
        }
    }

    fn link(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            ()
        } else {
            if self.a[x] > self.a[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.a[x] += self.a[y];
            self.a[y] = x as i32;
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}
