mod scc {
    pub fn scc(graph: &Vec<Vec<usize>>) -> Vec<usize> {
	let mut vis: Vec<u8> = vec![0; graph.len()];
	let mut ord = Vec::new();
	for root in 0..graph.len() {
	    if vis[root] == 1 { continue; }
	    let mut stk = Vec::new();
	    stk.push((root, 0));
	    while let Some(&(v, i)) = stk.last() {
		if i == 0 {
		    vis[v] = 1;
		}
		if i == graph[v].len() {
		    ord.push(v);
		    stk.pop();
		} else {
		    let w = graph[v][i];
		    *stk.last_mut().unwrap() = (v, i + 1);
		    if vis[w] == 0 {
			stk.push((w, 0));
		    }
		}
	    }
	}
	let mut scc: Vec<usize> = vec![0; graph.len()];
	let mut count = 0;
	for &root in &ord {
	    if vis[root] == 2 { continue; }
	    count += 1;
	    let mut stk = Vec::new();
	    stk.push((root, 0));
	    while let Some(&(v, i)) = stk.last() {
		if i == 0 {
		    vis[v] = 2;
		    scc[v] = count;
		}
		if i == graph[v].len() {
		    stk.pop();
		} else {
		    let w = graph[v][i];
		    *stk.last_mut().unwrap() = (v, i + 1);
		    if vis[w] == 1 {
			stk.push((w, 0));
		    }
		}
	    }
	}
	scc.iter_mut().for_each(|x| *x = count - *x);
	scc
    }
}

