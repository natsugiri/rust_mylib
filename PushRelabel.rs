mod push_relabel {
    struct Edge {
	src: usize,
	dst: usize,
	cap: i64,
    }

    pub struct PushRelabel {
	n: usize,
	graph: Vec<Vec<usize>>,
	edges: Vec<Edge>,
	x: Vec<i64>,
	d: Vec<i32>,
	stk: Vec<Vec<usize>>,
	stk_size: usize,
    }

    impl PushRelabel {
	pub fn new(n: usize) -> Self {
	    PushRelabel { 
		n,
		graph: vec![Vec::new(); n],
		edges: Vec::new(),
		x: Vec::new(),
		d: Vec::new(),
		stk: Vec::new(),
		stk_size: 0,
	    }
	}

	pub fn add_edge(&mut self, src: usize, dst: usize, cap: i64) {
	    self.add_biedge(src, dst, cap, 0);
	}

	pub fn add_biedge(&mut self, src: usize, dst: usize, cap: i64, cap_rev: i64) {
	    let edges = &mut self.edges;
	    self.graph[src].push(edges.len());
	    edges.push(Edge { src, dst, cap });
	    self.graph[dst].push(edges.len());
	    edges.push(Edge { src: dst, dst: src, cap: cap_rev });
	}

	pub fn solve(&mut self, source: usize, sink: usize) -> i64 {
	    self.x = vec![0; self.n];
	    self.x[source] = i64::MAX;
	    self.stk = vec![Vec::new(); self.n * 2];
	    self.d = vec![0; self.n];
	    self.d[source] = self.n as i32;
	    self.global_relabeling(source, sink);

	    for i in self.graph[source].clone() {
		self.push(i);
	    }

	    let mut cnt = 0;
	    while self.stk_size > 0 {
		if let Some(v) = self.stk[self.stk_size - 1].pop() {
		    if v != source && v != sink && self.x[v] > 0 {
			for i in 0..self.graph[v].len() {
			    let i = self.graph[v][i];
			    let dst = self.edges[i].dst;
			    if self.edges[i].cap > 0 && self.d[v] == self.d[dst] + 1 {
				self.push(i);
			    }
			}
			if self.x[v] > 0 {
			    cnt += 1;
			    if cnt >= 2 * self.n {
				self.global_relabeling(source, sink);
				self.stk_push(v);
				cnt = 0;
			    } else {
				self.relabel(v);
			    }
			}
		    }
		} else {
		    self.stk_size -= 1;
		}
	    }
	    self.x[sink]
	}

	fn push(&mut self, index: usize) {
	    let arc = &mut self.edges[index];
	    let delta = self.x[arc.src].min(arc.cap);
	    if delta == 0 { return; }
	    self.x[arc.src] -= delta;
	    self.x[arc.dst] += delta;
	    arc.cap -= delta;
	    self.edges[index ^ 1].cap += delta;
	    self.stk_push(self.edges[index].dst);
	}

	fn relabel(&mut self, v: usize) {
	    let mut mi = i32::MAX;
	    for &i in &self.graph[v] {
		if self.edges[i].cap > 0 {
		    mi = mi.min(self.d[self.edges[i].dst]);
		}
	    }
	    self.d[v] = mi + 1;
	    self.stk_push(v);
	}

	fn stk_push(&mut self, v: usize) {
	    self.stk[self.d[v] as usize].push(v);
	    self.stk_size = self.stk_size.max(self.d[v] as usize + 1);
	}

	fn global_relabeling(&mut self, source: usize, sink: usize) {
	    for i in 0..self.n {
		self.d[i] = self.d[i].max(self.n as i32);
	    }
	    self.d[sink] = 0;
	    let mut que = vec![sink];
	    let mut t = 0;
	    while t < que.len() {
		let v = que[t];
		let k = self.d[v] + 1;
		if k as usize >= self.n { break; }
		t += 1;
		for &i in &self.graph[v] {
		    let arc = &self.edges[i ^ 1];
		    if arc.cap > 0 && arc.src != source && self.d[arc.src] > k {
			self.d[arc.src] = k;
			que.push(arc.src);
		    }
		}
	    }
	}
    }
}
