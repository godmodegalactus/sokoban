#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench_tests {
    use rand::seq::SliceRandom;
    use rand::{self, Rng};
    use sokoban::node_allocator::NodeAllocatorMap;
    use sokoban::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use test::Bencher;

    const MAX_SIZE: usize = 20001;
    const NUM_BUCKETS: usize = MAX_SIZE >> 2;
    const NUM_NODES: usize = (MAX_SIZE << 1) + 1;

    type RBTree = RedBlackTree<u128, u128, MAX_SIZE>;

    const NUM_BUCKETS_1K: usize = 1000;
    const NUM_NODES_1K: usize = (1001 << 1) + 1;

    type RBTree1K = RedBlackTree<u128, u128, 1001>;

    #[bench]
    fn bench_std_btree_map_insert_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = BTreeMap::new();
        b.iter(|| {
            for v in 0..1000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_std_hash_map_insert_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = HashMap::new();
        b.iter(|| {
            for v in 0..1000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_sokoban_red_black_tree_insert_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = RBTree1K::new();
        b.iter(|| {
            for v in 0..1000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_sokoban_red_black_tree_insert_1000_u128_stack(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = RBTree1K::new();
        b.iter(|| {
            for v in 0..1000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_std_btree_map_insert_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = BTreeMap::new();
        b.iter(|| {
            for v in 0..20000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_std_hash_map_insert_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = HashMap::new();
        b.iter(|| {
            for v in 0..20000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_sokoban_red_black_tree_insert_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = RBTree::new();
        b.iter(|| {
            for v in 0..20000 {
                m.insert(v as u128, rng.gen::<u128>());
            }
        })
    }

    #[bench]
    fn bench_std_btree_map_remove_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = BTreeMap::new();
        let mut slice: Vec<u128> = (0..1000).collect();
        slice.shuffle(&mut rng);
        for v in 0..1000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for k in slice.iter() {
                m.remove(k);
            }
        })
    }

    #[bench]
    fn bench_std_hash_map_remove_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = HashMap::new();
        let mut slice: Vec<u128> = (0..1000).collect();
        slice.shuffle(&mut rng);
        for v in 0..1000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for k in slice.iter() {
                m.remove(k);
            }
        })
    }

    #[bench]
    fn bench_sokoban_red_black_tree_remove_1000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = RBTree::new();
        let mut slice: Vec<u128> = (0..1000).collect();
        slice.shuffle(&mut rng);
        for v in 0..1000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for k in slice.iter() {
                m.remove(k);
            }
        })
    }

    #[bench]
    fn bench_std_btree_map_lookup_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = BTreeMap::new();
        for v in 0..20000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for v in 0..20000 {
                m.get(&v);
            }
        })
    }

    #[bench]
    fn bench_std_hash_map_lookup_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = HashMap::new();
        for v in 0..20000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for v in 0..20000 {
                m.get(&v);
            }
        })
    }

    #[bench]
    fn bench_sokoban_red_black_tree_lookup_20000_u128(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut m = RBTree::new();
        for v in 0..20000 {
            m.insert(v as u128, rng.gen::<u128>());
        }
        b.iter(|| {
            for v in 0..20000 {
                m.get(&v);
            }
        })
    }
}
