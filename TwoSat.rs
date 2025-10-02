mod two_sat {
    // graph[i]: i が偶数のとき変数x, i+1がnot x;
    #[derive(Clone, Debug)]
    pub struct TwoSat {
	graph: Vec<Vec<usize>>,
    }

    impl TwoSat {
	pub fn new() -> Self {
	    Self { graph: Vec::new() }
	}

	pub fn add_nodes(&mut self, n: usize) -> Vec<Node> {
	    let cur = self.graph.len();
	    self.graph.resize(cur + n * 2, Vec::new());
	    (0..n).map(|i| Node(cur + i * 2)).collect()
	}

	pub fn imply(&mut self, x: Node, y: Node) {
	    self.graph[x.0].push(y.0);
	    self.graph[y.not().0].push(x.not().0);
	}

	pub fn solve(&self) -> Option<Truth> {
	    use crate::scc;
	    let scc = scc::scc(&self.graph);
	    let n = self.graph.len() / 2;
	    let mut ret = vec![false; n];
	    for i in 0..n {
		if scc[i*2] == scc[i*2+1] {
		    return None
		}
		ret[i] = scc[i*2] > scc[i*2+1];
	    }
	    Some(Truth(ret))
	}
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Node(usize);

    impl Node {
	pub fn not(&self) -> Node {
	    Node(self.0 ^ 1)
	}
    }

    #[derive(Clone, Debug)]
    pub struct Truth(Vec<bool>);
    impl Truth {
	pub fn get(&self, x: Node) -> bool {
	    if x.0 % 2 == 0 {
		self.0[x.0 / 2]
	    } else {
		!self.0[x.0 / 2]
	    }
	}
    }
}

