use std::cmp::max;

trait DSUElem {
    fn index(&self) -> usize;
}

impl DSUElem for usize {
    fn index(&self) -> usize {
        *self
    }
}

struct DSURoot {
    index: usize,
    size: usize,
}
impl DSURoot {
    fn of(index: usize, size: usize) -> DSURoot {
        DSURoot { index, size }
    }
}

struct DSU {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    count: usize,
}

impl DSU {
    #[allow(dead_code)]
    pub fn new() -> DSU {
        DSU {
            parents: vec![],
            sizes: vec![],
            count: 0,
        }
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> DSU {
        DSU {
            parents: (0..capacity).collect(),
            sizes: vec![1; capacity],
            count: capacity,
        }
    }

    #[allow(dead_code)]
    fn extend_to_index(&mut self, index: usize) {
        let pl = self.parents.len();
        let sl = self.parents.len();
        self.parents.extend(pl..=index);
        self.sizes.extend((sl..=index).map(|_| 1));
        self.count += (index + 1).checked_sub(pl).unwrap_or_default();
    }

    fn set_root_and_size_for_index(&mut self, index: usize, root_index: usize, size: usize) {
        if let Some(pa_i_ref) = self.parents.get_mut(index) {
            *pa_i_ref = root_index;
        }
        if let Some(size_ref) = self.sizes.get_mut(index) {
            *size_ref = size;
        }
    }

    #[allow(dead_code)]
    pub fn union<T: DSUElem>(&mut self, a: &T, b: &T) {
        let back = max(a.index(), b.index());
        self.extend_to_index(back);

        let a_root = self.find_root_of_elem(a);
        let b_root = self.find_root_of_elem(b);
        let (pa, ch) = if a_root.size > b_root.size {
            (a_root, b_root)
        } else {
            (b_root, a_root)
        };
        if pa.index != ch.index {
            self.set_root_and_size_for_index(ch.index, pa.index, 0);
            self.set_root_and_size_for_index(pa.index, pa.index, ch.size + pa.size);
        }
    }

    #[allow(dead_code)]
    pub fn find_root_of_elem<T: DSUElem>(&mut self, element: &T) -> DSURoot {
        self.find_root_of_index(element.index())
    }

    #[allow(dead_code)]
    fn find_root_of_index(&mut self, index: usize) -> DSURoot {
        self.parents.extend(self.parents.len()..=index);
        let pa_index = self.parents[index];
        if pa_index == index {
            DSURoot::of(index, self.sizes[index])
        } else {
            let root = self.find_root_of_index(pa_index);
            self.set_root_and_size_for_index(index, root.index, 0);
            root
        }
    }

    #[allow(dead_code)]
    pub fn list_all_roots(&self) -> Vec<DSURoot> {
        self.parents
            .iter()
            .zip(self.sizes.iter())
            .filter_map(|(pa, size)| {
                if *size == 0 {
                    None
                } else {
                    Some(DSURoot::of(*pa, *size))
                }
            })
            .collect::<Vec<DSURoot>>()
    }
}

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut dsu = DSU::with_capacity(n as usize);
        edges.iter().for_each(|v| {
            if let &[x, y, ..] = v.as_slice() {
                dsu.union(&(x as usize), &(y as usize));
            }
        });
        dsu.list_all_roots()
            .iter()
            .map(|root| root.size * (n as usize - root.size))
            .sum::<usize>() as i64
            / 2
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example1() {
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];
        assert_eq!(Solution::count_pairs(7, edges), 14)
    }
}
