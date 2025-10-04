mod grid {
    use std::ops::{Index, IndexMut};

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Grid<T> {
	height: usize,
	width: usize,
	a: Vec<T>,
    }

    #[allow(dead_code)]
    impl<T> Grid<T> {
	pub fn new(height: usize, width: usize, value: T) -> Self where T: Clone {
	    Self { height, width, a: vec![value; height * width] }
	}
	pub fn height(&self) -> usize {
	    self.height
	}
	pub fn width(&self) -> usize {
	    self.width
	}
	pub fn resize(&mut self, new_height: usize, new_width: usize, value: T) where T: Clone {
	    self.a.resize(new_height * new_width, value);
	}
	pub fn fill(&mut self, value: T) where T: Clone {
	    self.a.fill(value);
	}
    }

    impl<T> Index<usize> for Grid<T> {
        type Output = [T];
        fn index(&self, i: usize) -> &Self::Output {
	    assert!(i < self.height, "ERROR: Grid index out of bounds height={}, index={}", self.height, i);
            &self.a[i * self.width .. (i + 1) * self.width]
        }
    }

    impl<T> IndexMut<usize> for Grid<T> {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output {
	    assert!(i < self.height, "ERROR: Grid index out of bounds height={}, index={}", self.height, i);
	    &mut self.a[i * self.width .. (i + 1) * self.width]
	}
    }
} // mod grid;
