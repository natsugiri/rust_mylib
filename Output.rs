fn main() {
    println!("{}", val.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}
