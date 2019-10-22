use std::cmp::min;
use std::mem::swap;
use crate::{Vector, Rank};

#[derive(Debug)]
pub struct VecVector<T: Ord> {
    elems: Vec<T>
}

impl<T: Ord> VecVector<T> {
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
}

impl<T: Ord> Vector<T> for VecVector<T> {
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

    fn search(&self, e: &T, mut lo: Rank, mut hi: Rank) -> Option<Rank> {
        if hi <= self.size() {
            while lo < hi {
                let mid = (lo + hi) >> 1;
                if *e < self.elems[mid] {
                    hi = mid;
                } else if self.elems[mid] < *e {
                    lo = mid + 1;
                } else {
                    return Some(mid);
                }
            }
        }
        None
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