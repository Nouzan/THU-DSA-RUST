use std::cmp::min;
use std::mem::swap;
use std::fmt;
use crate::{Vector, Rank};

#[derive(Debug)]
pub struct VecVector<T: Ord> {
    elems: Vec<T>
}

impl<T: Ord+fmt::Display> fmt::Display for VecVector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = write!(f, "[");
        for (i, e) in self.elems.iter().enumerate() {
            res = write!(f, "{}", e);
            if i < self.size() - 1 {
                res = write!(f, ",");
            }
        }
        res = write!(f, "]");
        res
    }
}

impl<T: Ord+fmt::Display> VecVector<T> {
    pub fn new() -> Self {
        VecVector {
            elems: Vec::new()
        }
    }

    fn bubble(&mut self, lo: Rank, hi: Rank) -> bool {
        let mut sorted = true;
        for i in (lo + 1)..hi {
            if self.elems[i - 1] > self.elems[i] {
                self.elems.swap(i - 1, i);
                sorted = false;
            }
        }
        sorted
    }

    fn search_b(&self, e: &T, mut lo: Rank, mut hi: Rank) -> Option<Rank> {
        while 1 < hi - lo {
            let mid = (lo + hi) >> 1;
            if *e < self.elems[mid] {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        // [lo, lo) or [lo, lo + 1)
        // 1) [lo, lo): [lo, lo + 1) -> [lo, lo) 是不可能的；因此必然是 [lo, lo) -> [lo, lo)
        // 2) [lo, lo + 1): [lo, hi) -> [lo, lo + 1) 即：hi -> mid，意味着 *e < *(lo + 1)，则 *e <= *lo
        //                  [lo, hi) -> [hi - 1, hi) 即：lo -> mid，意味着 *lo <= *e <= *(lo + 1)
        if *e == self.elems[lo] {
            Some(lo)
        } else {
            None
        }
    }

    fn search_c(&self, e: &T, mut lo: Rank, mut hi: Rank) -> Option<Rank> {
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if *e < self.elems[mid] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        // [lo, lo + 1) -> [lo, lo): *e < *lo
        // [hi - 1, hi) -> [hi, hi): *(hi - 1) <= *e < *hi
        if lo == 0 {
            None
        } else {
            Some(lo - 1)
        }
    }
}

impl<T: Ord+fmt::Display> Vector<T> for VecVector<T> {
    fn size(&self) -> usize {
        self.elems.len()
    }

    fn disordered(&self) -> usize {
        if self.size() < 2 {
            0
        } else {
            let mut sum: usize = 0;
            for i in 0..self.size() - 1 {
                if self.elems[i] > self.elems[i + 1] {
                    sum += 1;
                }
            }
            sum
        }
    }

    fn find(&self, e: &T, lo: Rank, hi: Rank) -> Option<Rank> {
        if lo < hi && hi <= self.size() {
            for i in lo..hi {
                if self.elems[i] == *e {
                    return Some(i);
                }
            }
        }
        None
    }

    fn search(&self, e: &T, lo: Rank, hi: Rank) -> Option<Rank> {
        if lo < hi && hi <= self.size() && self.elems[lo] <= *e {
            self.search_c(e, lo, hi)
        } else {
            None
        }
    }

    fn get(&self, r: Rank) -> Option<&T> {
        if r >= self.size() {
            None
        } else {
            Some(&self.elems[r])
        }
    }

    fn set(&mut self, r: Rank, e: T) -> &mut Self {
        if r < self.size() {
            self.elems[r] = e;
        }
        self
    }

    fn remove_one(&mut self, r: Rank) -> Option<T> {
        if r >= self.size() {
            None
        } else {
            Some(self.elems.remove(r))
        }
    }

    fn remove(&mut self, lo: Rank, hi: Rank) -> usize {
        let hi = min(self.size(), hi);
        for i in (lo..hi).rev() {
            self.remove_one(i);
        }
        hi - lo
    }

    fn insert(&mut self, r: Rank, e: T) -> &mut Self {
        self.elems.insert(r, e);
        self
    }

    fn sort(&mut self, lo: Rank, hi: Rank) -> &mut Self {
        let hi = min(self.size(), hi);
        for i in (lo..hi).rev() {
            if self.bubble(lo, i + 1) {
                break
            }
        }
        self
    }
}