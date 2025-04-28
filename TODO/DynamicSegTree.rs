mod dynamic_segtree {
    use std::clone::Clone;
    use std::ops::Range;

    #[derive(Debug)]
    struct Node<Seg> {
        left_most: u64,
        right_most: u64,
        seg: Seg,
        left: Option<Box<Self>>,
        right: Option<Box<Self>>,
    }

    impl<Seg: Clone> Node<Seg> {
        pub fn new_leaf(key: u64, seg: Seg) -> Self {
            Self {
                left_most: key,
                right_most: key,
                seg: seg.clone(),
                left: None,
                right: None,
            }
        }

        pub fn refresh<Join: Fn(&Seg, &Seg) -> Seg>(&mut self, join: &Join) {
            if let Some(left) = &self.left {
                if let Some(right) = &self.right {
                    self.seg = join(&left.seg, &right.seg);
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct DynamicSegTree<Seg, Join> {
        root: Option<Box<Node<Seg>>>,
	default_seg: Seg,
	join: Join,
    }

    impl <Seg, Join>DynamicSegTree<Seg, Join> where Seg: Clone,
    Join: Fn(&Seg, &Seg) -> Seg, {
        pub fn new(default_seg: Seg, join: Join) -> Self {
            Self {
                root: None,
                default_seg,
                join,
            }
        }

        pub fn get(&self, key: u64) -> Option<&Seg> {
            if let Some(ref node) = &self.root {
                if key < node.left_most {
                    None
                } else if node.right_most < key {
                    None
                } else {
                    let mut node: &Box<Node<Seg>> = node;
                    loop {
                        if node.left_most == key && node.right_most == key {
                            return Some(&node.seg);
                        }
                        if (node.left_most ^ key).leading_zeros() > (node.right_most ^ key).leading_zeros() {
                            node = node.left.as_ref().unwrap();
                        } else {
                            node = node.right.as_ref().unwrap();
                        }
                    }
                }
            } else {
                None
            }
        }

        pub fn push(&mut self, key: u64, seg: Seg) {
            let mut node_opt = self.root.take();
            self.rec_push(&mut node_opt, key, seg);
            self.root = node_opt;
        }

        fn rec_push(&self, node_opt: &mut Option<Box<Node<Seg>>>, key: u64, seg: Seg) {
            match node_opt {
                None => {
                    *node_opt = Some(Box::new(Node::new_leaf(key, seg)));
                },
                Some(node) => {
                    let clz_key_left = (key ^ node.left_most).leading_zeros();
                    let clz_key_right = (key ^ node.right_most).leading_zeros();
                    let clz_left_right = (node.left_most ^ node.right_most).leading_zeros();
                    if key < node.left_most && clz_key_left < clz_left_right {
                        let mut new_node = Box::new(Node {
                            seg: seg.clone(),
                            left_most: key,
                            right_most: node.right_most,
                            left: None,
                            right: node_opt.take(),
                        });
                        self.rec_push(&mut new_node.left, key, seg);
                        *node_opt = Some(new_node);
                    } else if node.right_most < key && clz_key_right < clz_left_right {
                        let mut new_node = Box::new(Node {
                            seg: seg.clone(),
                            left_most: node.left_most,
                            right_most: key,
                            left: node_opt.take(),
                            right: None,
                        });
                        self.rec_push(&mut new_node.right, key, seg);
                        *node_opt = Some(new_node);
                    } else if node.left_most == node.right_most {
                        node.seg = seg.clone();
                    } else if clz_key_left < clz_key_right {
                        self.rec_push(&mut node.right, key, seg);
                    } else {
                        self.rec_push(&mut node.left, key, seg);
                    }
                },
            }
            node_opt.as_mut().unwrap().refresh(&self.join);
        }

        pub fn range(&self, range: Range<u64>) -> Seg {
            self.rec_range(&self.root, &range)
        }

        fn rec_range(&self, node_opt: &Option<Box<Node<Seg>>>, range: &Range<u64>) -> Seg {
            match node_opt {
                None => self.default_seg.clone(),
                Some(node) => {
                    if range.end <= node.left_most || node.right_most < range.start {
                       self.default_seg.clone()
                    } else if range.start <= node.left_most && node.right_most < range.end {
                        node.seg.clone()
                    } else {
                        (self.join)(&self.rec_range(&node.left, &range), &self.rec_range(&node.right, &range))
                    }
                },
            }
        }
    }

    impl <Seg, Join>DynamicSegTree<Seg, Join> where Seg: std::fmt::Debug {
        pub fn show(&self) {
            Self::rec_show(&self.root);
            eprint!("\n");
        }

        fn rec_show(node_opt: &Option<Box<Node<Seg>>>) {
            match node_opt {
                None => { eprint!("_"); },
                Some(node) => {
                    eprint!("({:?}:", node.seg);
                    Self::rec_show(&node.left);
                    Self::rec_show(&node.right);
                    eprint!(")");
                },
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Seg {
    value: i64,
    len: u64,
}
impl Seg {
    pub fn new(value: i64) -> Self {
        Seg { value, len: 1 }
    }
}


fn main() {
    let mut tree = dynamic_segtree::DynamicSegTree::new(Seg{value:0, len:0}, |x, y| Seg {
        value: if x.len % 2 == 0 {
            x.value + y.value
        } else {
            x.value - y.value
        },
        len: x.len + y.len,
    });
    tree.push(1, Seg::new(1));
    tree.push(2, Seg::new(2));
    tree.push(5, Seg::new(5));
    tree.push(11, Seg::new(11));
    tree.push(9, Seg::new(9));
    tree.push(3, Seg::new(3));
    tree.modify(3, Seg::new(10000));
    println!("{:?}", tree.get(1));
    println!("{:?}", tree.get(2));
    println!("{:?}", tree.range(1..1000));
    tree.show();
}
