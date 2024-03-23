trait Bisection<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: std::cmp::PartialOrd> Bisection<T> for Vec<T> {
    fn lower_bound(&self, x: T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let middle = (left + right) / 2;
            if self[middle] < x {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        left
    }

    fn upper_bound(&self, x: T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let middle = (left + right) / 2;
            if x < self[middle] {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}

