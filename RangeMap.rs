mod rangemap {
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub struct RangeMap<Key, Value> {
        map: std::collections::BTreeMap<Key, Value>,
        initial_value: Value,
    }

    impl<Key: Debug + Clone + Ord, Value: Debug + Clone + PartialEq + Default> RangeMap<Key, Value> {
        pub fn new() -> Self {
            Self::new_with_initial_value(Value::default())
        }
    }

    impl<Key: Debug + Clone + Ord, Value: Debug + Clone + PartialEq> RangeMap<Key, Value> {
        pub fn new_with_initial_value(initial_value: Value) -> Self {
            RangeMap {
                map: std::collections::BTreeMap::new(),
                initial_value,
            }
        }

        fn chop(&mut self, key: Key) {
            let mut right = self.map.range(key.clone()..);
            if let Some((right_key, right_value)) = right.next() {
                if key < *right_key {
                    self.map.insert(key, right_value.clone());
                }
            } else {
                self.map.insert(key, self.initial_value.clone());
            }
        }

        fn join_if_same(&mut self, key: Key) {
            let mut lb = self.map.range(key.clone()..);
            let Some((left_key, left_value)) = lb.next() else { panic!("range not found"); };
            assert!(*left_key == key);
            if let Some((_right_key, right_value)) = lb.next() {
                if *left_value == *right_value {
                    self.map.remove(&key);
                }
            } else if *left_value == self.initial_value {
                self.map.remove(&key);
            }
        }

        pub fn overwrite(&mut self, left: Key, right: Key, new_value: Value) {
            self.chop(left.clone());
            self.chop(right.clone());
            let mut pending_remove = Vec::new();
            let mut it = self.map.range_mut(left.clone() ..= right.clone());
            it.next();
            while let Some((key, value)) = it.next() {
                *value = new_value.clone();
                if *key < right {
                    pending_remove.push(key.clone());
                }
            }
            for key in pending_remove {
                self.map.remove(&key);
            }
            self.join_if_same(left.clone());
            self.join_if_same(right.clone());
        }


        // ex. diff = map.fold_and_overwrite(left, right, 0,
        //         |acc, left, right, value| { acc + (right - left) * (new_value - value) },
        //         new_value);
        pub fn fold_and_overwrite<Acc, F: Fn(Acc, Key, Key, Value) -> Acc>(&mut self, left: Key, right: Key, init: Acc, f: F, new_value: Value) -> Acc {
            self.chop(left.clone());
            self.chop(right.clone());
            let mut pending_remove = Vec::new();
            let mut it = self.map.range_mut(left.clone() ..= right.clone());
            let mut acc = init;
            let mut prv = left.clone();
            it.next();
            while let Some((key, value)) = it.next() {
                acc = f(acc, prv, key.clone(), value.clone());
                *value = new_value.clone();
                prv = key.clone();
                if *key < right {
                    pending_remove.push(key.clone());
                }
            }
            for key in pending_remove {
                self.map.remove(&key);
            }
            self.join_if_same(left.clone());
            self.join_if_same(right.clone());
            acc
        }

        pub fn fold<Acc, F: Fn(Acc, Key, Key, Value) -> Acc>(&mut self, left: Key, right: Key, init: Acc, f: F) -> Acc {
            self.chop(left.clone());
            self.chop(right.clone());
            let mut it = self.map.range_mut(left.clone() ..= right.clone());
            let mut acc = init;
            let mut prv = left.clone();
            it.next();
            while let Some((key, value)) = it.next() {
                acc = f(acc, prv, key.clone(), value.clone());
                prv = key.clone();
            }
            self.join_if_same(left.clone());
            self.join_if_same(right.clone());
            acc
        }
    }
} // mod rangemap;

fn main() {
    let mut map = rangemap::RangeMap::<i32, i32>::new_with_initial_value(-1);
    map.overwrite(10, 20, 100);
    map.overwrite(15, 18, 150);
    let mut v = Vec::new();
    {
        map.fold_and_overwrite(10, 30, &mut v, |acc, left, right, value| {
            println!("!! {} {} {}", left, right, value);
            acc.push(value);
            acc
        }, -1);
    }
    println!("{:?}", v);
    map.fold_and_overwrite(10, 30, 0, |acc, left, right, value| {
        println!("!! {} {} {}", left, right, value);
        value
    }, -1);

    let mut map = rangemap::RangeMap::<String, String>::new_with_initial_value("".to_string());
}
