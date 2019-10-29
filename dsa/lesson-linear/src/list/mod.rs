pub mod node;

pub trait List<T: Eq> {
    type ListNodePosi;

    fn size(&self) -> usize;
    fn empty(&self) -> bool {
        self.size() == 0
    }

    fn find(&self, e: &T) -> Option<Self::ListNodePosi>;
    fn relative_find(&self, e: &T, n: usize, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi>;

    // fn first(&self) -> Option<Self::ListNodePosi>;
    // fn last(&self) -> Option<Self::ListNodePosi>;
    // fn valid(p: &Self::ListNodePosi) -> bool;

    fn insert_as_first(&mut self, e: T) -> Self::ListNodePosi;
    fn insert_as_last(&mut self, e: T) -> Self::ListNodePosi;
    fn insert_before(&mut self, q: &Self::ListNodePosi, e: T) -> Self::ListNodePosi;
    fn insert_after(&mut self, p: &Self::ListNodePosi, e: T) -> Self::ListNodePosi;
    
    // fn remove(p: &Self::ListNodePosi) -> T;

}

use crate::list::node::ListNodePosi;
use crate::list::node::ListNode;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct RcList<T> {
    header: ListNodePosi<T>,
    tailer: ListNodePosi<T>,
    size: usize
}

impl<T: Default + fmt::Debug> RcList<T> {
    pub fn new() -> Self {
        let h = ListNode::new_as_posi(T::default());
        let t = h.borrow_mut().insert_as_suss(T::default());
        RcList {
            header: h,
            tailer: t,
            size: 0
        }
    }
}

impl<T: Eq + fmt::Debug> List<T> for RcList<T> {
    type ListNodePosi = ListNodePosi<T>;

    fn size(&self) -> usize {
        self.size
    }

    fn find(&self, e: &T) -> Option<Self::ListNodePosi> {
        self.relative_find(e, self.size, &self.tailer)
    }

    fn relative_find(&self, e: &T, mut n: usize, mut p: &Self::ListNodePosi) -> Option<Self::ListNodePosi> {
        while n > 0 {
            n = n - 1;
            if let Some(p) = p.borrow().pred() {
                if Rc::ptr_eq(&p, &self.header) {
                    return None;
                }
                if **p.borrow() == *e {
                    return Some(p);
                }
            } else {
                return None
            }
        }
        None
    }

    fn insert_as_first(&mut self, e: T) -> Self::ListNodePosi {
        let qq = self.header.borrow().suss().unwrap();
        let q = self.header.borrow_mut().insert_as_suss(e);
        q.borrow_mut().combine(&qq);
        self.size += 1;
        q
    }

    fn insert_as_last(&mut self, e: T) -> Self::ListNodePosi {
        let pp = self.tailer.borrow().pred().unwrap();
        let p = self.tailer.borrow_mut().insert_as_pred(e);
        pp.borrow_mut().combine(&p);
        self.size += 1;
        p
    }

    fn insert_before(&mut self, q: &Self::ListNodePosi, e: T) -> Self::ListNodePosi {
        let pp = q.borrow().pred().unwrap();
        let p = q.borrow_mut().insert_as_pred(e);
        pp.borrow_mut().combine(&p);
        self.size += 1;
        p
    }

    fn insert_after(&mut self, p: &Self::ListNodePosi, e: T) -> Self::ListNodePosi {
        let qq = p.borrow().suss().unwrap();
        let q = p.borrow_mut().insert_as_suss(e);
        q.borrow_mut().combine(&qq);
        self.size += 1;
        q
    }
}