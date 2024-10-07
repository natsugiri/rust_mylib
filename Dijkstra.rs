#[derive(Debug, Clone)]
struct Dijkstra (Vec<Vec<(usize, i64)>>);

impl Dijkstra {
    pub fn new(n: usize) -> Self {
        Self(vec![Vec::new(); n])
    }

    pub fn add_edge(&mut self, x: usize, y: usize, c: i64) {
        self.0[x].push((y, c));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn solve(&self, source: usize) -> Vec<i64> {
        use std::cmp::Reverse;

        let mut dist = vec![i64::MAX; self.len()];
        dist[source] = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Reverse((0, source)));
        while let Some(Reverse((d, v))) = heap.pop() {
            if dist[v] == d {
                for &(to, c) in self.0[v].iter() {
                    if dist[to] > d + c {
                        dist[to] = d + c;
                        heap.push(Reverse((dist[to], to)));
                    }
                }
            }
        }
        dist
    }
}

fn main() {
    let mut g = Dijkstra::new(6);
    g.add_edge(0, 2, 10);
    g.add_edge(2, 5, 20);
    g.add_edge(5, 3, 12);
    println!("{:?}", g.solve(0));
}
