trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
    fn prev_permutation(&mut self) -> bool;
}

impl<T: PartialOrd> NextPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        let n = self.len();
        if n <= 1 { return false; }

        for i in (0..n-1).rev() {
            if self[i] < self[i+1] {
                for j in (0..n).rev() {
                    if self[i] < self[j] {
                        self.swap(i, j);
                        self[i+1..].reverse();
                        return true;
                    }
                }
            }
        }
        self.reverse();
        false
    }

    fn prev_permutation(&mut self) -> bool {
        let n = self.len();
        if n <= 1 { return false; }

        for i in (0..n-1).rev() {
            if self[i] > self[i+1] {
                for j in (0..n).rev() {
                    if self[i] > self[j] {
                        self.swap(i, j);
                        self[i+1..].reverse();
                        return true;
                    }
                }
            }
        }
        self.reverse();
        false
    }
}


fn main() {
    let mut v = vec![1, 2, 2, 3];

    while {
        println!("{:?}", v);

        v.next_permutation()
    } {}

    println!("> {:?}", v);
}
