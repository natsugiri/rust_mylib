fn read<T> () -> T where T: std::str::FromStr {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.trim().parse().ok().unwrap()
}

fn read_vec<T> () -> Vec<T> where T: std::str::FromStr {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.split_whitespace().map(|t| t.parse().ok().unwrap()).collect()
}

fn main() {
    let n = read::<usize>();

    println!("{}", n);
}
