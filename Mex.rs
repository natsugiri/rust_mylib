fn mex(a: &Vec<usize>) -> usize {
    let mut c = vec![false; a.len()];
    for &x in a {
	if x < a.len() {
	    c[x] = true;
	}
    }
    for i in 0..a.len() {
	if !c[i] {
	    return i;
	}
    }
    a.len()
}
