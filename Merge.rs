mod merge {
    use std::cmp::Ordering;

    pub trait Merge<T, Output> {
	fn merge(self, other: Self) -> Output where T: Ord;
	fn merge_by<F: Fn(&T, &T) -> Ordering>(self, other: Self, comp: F) -> Output;
    }

    impl<T: Clone> Merge<T, Vec<T>> for &Vec<T> {
	fn merge(self, other: Self) -> Vec<T> where T: Ord {
	    self.merge_by(other, |x, y| x.cmp(y))
	}

	fn merge_by<F: Fn(&T, &T) -> Ordering>(self, other: Self, comp: F) -> Vec<T> {
	    let mut ret = Vec::new();
	    let mut i = 0;
	    let mut j = 0;
	    while i < self.len() || j < other.len() {
		if i == self.len() || (j < other.len() && comp(&self[i], &other[j]) == Ordering::Greater) {
		    ret.push(other[j].clone());
		    j += 1;
		} else {
		    ret.push(self[i].clone());
		    i += 1;
		}
	    }
	    ret
	}
    }

    impl<T: Clone> Merge<T, Vec<T>> for Vec<T> {
	fn merge(self, other: Self) -> Vec<T> where T: Ord {
	    (&self).merge(&other)
	}

	fn merge_by<F: Fn(&T, &T) -> Ordering>(self, other: Self, comp: F) -> Vec<T> {
	    (&self).merge_by(&other, comp)
	}
    }
} // mod merge;
