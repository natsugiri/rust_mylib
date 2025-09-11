mod shortest_path {
    #[derive(Debug, Default)]
    pub struct ShortestPath {
	n: usize,
	graph: Vec<Vec<(usize, i64)>>,
    }

    impl ShortestPath {
	pub fn new(n: usize) -> Self {
	    Self { n, graph: vec![Vec::new(); n] }
	}

	pub fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
	    self.graph[from].push((to, cost));
	}

	pub fn solve(&self, start: usize) -> Option<Vec<i64>> {
	    let mut dist = vec![i64::MAX; self.n];
	    let mut dq = std::collections::VecDeque::new();
	    let mut ins = vec![false; self.n];
	    let mut counter = vec![0; self.n];
	    dist[start] = 0;
	    dq.push_back(start);
	    ins[start] = true;
	    while let Some(v) = dq.pop_front() {
		ins[v] = false;
		for &(w, c) in &self.graph[v] {
		    if dist[w] > dist[v] + c {
			dist[w] = dist[v] + c;
			if !ins[w] {
			    counter[w] += 1;
			    if counter[w] >= self.n {
				return None;
			    }
			    dq.push_back(w);
			    ins[w] = true;
			}
		    }
		}
	    }
	    Some(dist)
	}
    }
}
