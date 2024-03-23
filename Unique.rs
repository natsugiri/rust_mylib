fn unique<T: Ord + Clone>(v: &mut Vec<T>) {
    v.sort();
    let mut len = 0;
    for i in 0..v.len() {
	if len == 0 || v[len-1] != v[i] {
	    let tmp = v[i].clone();
	    v[len] = tmp;
	    len += 1;
	}
    }
    while len < v.len() { v.pop(); }
}
