#[allow(non_snake_case)]
#[allow(unused_parens)]
use std::io::{self, Read};
use std::cmp::max;

// Iterative realization of Segment Tree

struct SegTree<T> {
    data : Vec<T>,
    size : usize,
    op: fn(T, T) -> T,
    default: T
}
impl<T: Copy> SegTree<T> {
    fn new(N: usize, n0: T, op: fn(T, T) -> T) -> SegTree<T> {
        let new_N = N.next_power_of_two();
        return SegTree::<T> { data: vec![n0; 2 * new_N], size: new_N, op: op, default: n0 };
    }

    fn set(&mut self, idx: usize, val: T) {
        let mut i = idx + self.size;
        self.data[i] = val;
        while (i > 1) {
            self.data[i / 2] = (self.op)(self.data[i], self.data[i ^ 1]);
            i = i>>1;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> T {
        let mut ret = self.default;
        l += self.size; r += self.size;

        while (l < r) {
            if (l % 2 == 1) { ret = (self.op)(ret, self.data[l + 1]); l += 1; };
            if (r % 2 == 1) { ret = (self.op)(ret, self.data[r - 1]); r -= 1; };
            l /= 2; r /= 2;
        }

        return ret;
    }
}


fn main() {
    let mut line = String::new();
    std::io::stdin().lock().read_to_string(&mut line).unwrap();
    let mut input = line.trim().split_whitespace();

    let N: usize = input.next().unwrap().parse().ok().unwrap();
    let Q: usize = input.next().unwrap().parse().ok().unwrap();
    let mut segtree: SegTree<i64> = SegTree::new(N, 0, max);
    for i in 0..N { segtree.set(i, input.next().unwrap().parse().ok().unwrap()); }

    for _ in 0..Q {    
        let t: usize = input.next().unwrap().parse().ok().unwrap();
        let l: usize = input.next().unwrap().parse().ok().unwrap();
        let r: i64 = input.next().unwrap().parse().ok().unwrap();
        
        if (t == 0) {
            segtree.set(l, r);
        } else {
            println!("{}", segtree.query(l, r as usize));
        }
    }

}
