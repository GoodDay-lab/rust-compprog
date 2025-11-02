// Iterative realization of Segment Tree

#[allow(dead_code)]
struct SegTree<T> {
    data : Vec<T>,
    size : usize,
    op: fn(T, T) -> T,
    default: T
}

#[allow(dead_code, non_snake_case, unused_parens)]
impl<T: Copy> SegTree<T> {
    fn new(N: usize, n0: T, op: fn(T, T) -> T) -> SegTree<T> {
        let new_N = N.next_power_of_two();
        return SegTree::<T> { data: vec![n0; 2 * new_N], size: new_N, op: op, default: n0 };
    }

    fn set(&mut self, idx: usize, val: T) {
        let mut i = idx + self.size;
        self.data[i] = val;
        while (i > 1) {
            self.data[i>>1] = (self.op)(self.data[i], self.data[i ^ 1]);
            i = i>>1;
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> T {
        let mut ret = self.default;
        l += self.size; r += self.size;

        while (l < r) {
            if (l % 2 == 1) { ret = (self.op)(ret, self.data[l]); l += 1; };
            if (r % 2 == 1) { ret = (self.op)(ret, self.data[r-1]); r -= 1; };
            l /= 2; r /= 2;
        }

        return ret;
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::max;
    use super::*;

    #[test]
    fn test_segtree_min() {
        let array = vec![1, 2, 3, 4, 5];

        let mut segtree: SegTree<i64> = SegTree::new(array.len(), 0, max);
        for i in 0..array.len() { segtree.set(i, array[i]); }

        let requests = [0, 4, 3, 1];
        let values   = [4, 0, 10, 0];
        for i in 0..requests.len() {
            let j = requests[i];
            segtree.set(j, values[i]);
            println!("{:?}", segtree.data);
            assert_eq!(segtree.query(j, j+1), values[i]);
        }

        let requests_2 = [(0, 3), (0, 1), (2, 5)];
        let correct    = [4, 4, 10];
        for i in 0..requests_2.len() {
            let (l, b) = requests_2[i];
            assert_eq!(segtree.query(l, b), correct[i]);
        }
    }
}
