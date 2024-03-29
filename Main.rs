mod input {
    use std::str::FromStr;

    pub fn read_line() -> String {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).ok();
	line.trim().into()
    }

    pub fn read<T: FromStr>() -> T {
	read_line().parse().ok().unwrap()
    }

    pub fn read_vec<T: FromStr>() -> Vec<T> {
	read_line().split_whitespace().map(|t| t.parse().ok().unwrap()).collect()
    }

    pub fn read2<T: FromStr, U: FromStr>() -> (T, U) {
	let line = read_line();
	let mut a = line.split_ascii_whitespace();
	(a.next().unwrap().parse().ok().unwrap(),
	a.next().unwrap().parse().ok().unwrap())
    }

    pub fn read3<T: FromStr, U: FromStr, V: FromStr>() -> (T, U, V) {
	let line = read_line();
	let mut a = line.split_ascii_whitespace();
	(a.next().unwrap().parse().ok().unwrap(),
	a.next().unwrap().parse().ok().unwrap(),
	a.next().unwrap().parse().ok().unwrap())
    }
}
use input::*;

fn main() {
    let n = read::<usize>();

    println!("{}", n);
}
