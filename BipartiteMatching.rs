mod bipartite_matching {
    // Hopcroft-Karp algorithm;
    // from_left(x) = y <=> from_right(y) = x
    // O(E sqrt(V));

    #[derive(Debug)]
    pub struct BipartiteMatching {
	left_size: usize,
	right_size: usize,
	graph: Vec<Vec<usize>>,
    }

    #[derive(Debug)]
    pub struct Matches {
	from_left: Vec<usize>,
	from_right: Vec<usize>,
	count: usize,
    }

    impl BipartiteMatching {
	pub const INVALID : usize = usize::MAX;

	pub fn new(left_size: usize, right_size: usize) -> Self {
	    BipartiteMatching { left_size, right_size, graph: vec![Vec::new(); left_size] }
	}

	pub fn add_edge(&mut self, left: usize, right: usize) {
	    self.graph[left].push(right);
	}

	pub fn solve(&self) -> Matches {
	    let mut res = Matches {
		from_left: vec![Self::INVALID; self.left_size],
		from_right: vec![Self::INVALID; self.right_size],
		count: 0,
	    };
	    loop {
		let (mut dist, stop) = Self::bfs(&self.graph, &res);
		if stop { break; }
		for i in 0..self.left_size {
		    if res.from_left[i] == Self::INVALID && self.dfs(i, &mut dist, &mut res) {
			res.count += 1;
		    }
		}
	    }
	    res
	}

	fn bfs(graph: &Vec<Vec<usize>>, m: &Matches) -> (Vec<usize>, bool) {
	    let mut dist = vec![Self::INVALID; graph.len()];
	    let mut que = Vec::new();
	    for i in 0..graph.len() {
		if m.from_left[i] == Self::INVALID {
		    que.push(i);
		    dist[i] = 0;
		}
	    }
	    let mut stop = Self::INVALID;
	    let mut i = 0;
	    while i < que.len() {
		let v = que[i];
		if dist[v] < stop {
		    for &x in graph[v].iter() {
			let y = m.from_right[x];
			if y == Self::INVALID {
			    stop = dist[v] + 1;
			} else if dist[y] == Self::INVALID {
			    dist[y] = dist[v] + 1;
			    que.push(y);
			}
		    }
		}
		i += 1;
	    }
	    (dist, stop == Self::INVALID)
	}

	fn dfs(&self, v: usize, dist: &mut Vec<usize>, m: &mut Matches) -> bool {
	    for &x in self.graph[v].iter() {
		let y = m.from_right[x];
		if y == Self::INVALID || (dist[v] + 1 == dist[y] && self.dfs(y, dist, m)) {
		    m.from_left[v] = x;
		    m.from_right[x] = v;
		    return true;
		}
	    }
	    dist[v] = Self::INVALID;
	    false
	}
    }

    impl Matches {
	pub fn from_left(&self, i: usize) -> Option<usize> {
	    if self.from_left[i] == BipartiteMatching::INVALID {
		None
	    } else {
		Some(self.from_left[i])
	    }
	}

	pub fn from_right(&self, i: usize) -> Option<usize> {
	    if self.from_right[i] == BipartiteMatching::INVALID {
		None
	    } else {
		Some(self.from_right[i])
	    }
	}

	pub fn count(&self) -> usize {
	    self.count
	}
    }
}
