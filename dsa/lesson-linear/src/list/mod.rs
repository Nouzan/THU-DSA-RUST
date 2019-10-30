pub mod node;

pub trait List<T: Eq> {
    type ListNodePosi;

    fn size(&self) -> usize;
    fn empty(&self) -> bool {
        self.size() == 0
    }

    fn find(&self, e: &T) -> Option<Self::ListNodePosi>;
    fn relative_find(&self, e: &T, n: usize, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi>;

    fn first(&self) -> Option<Self::ListNodePosi>;
    fn last(&self) -> Option<Self::ListNodePosi>;
    fn valid(&self, p: &Self::ListNodePosi) -> bool;
    fn pred(&self, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi>;
    fn suss(&self, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi>;

    fn insert_as_first(&mut self, e: T) -> Self::ListNodePosi;
    fn insert_as_last(&mut self, e: T) -> Self::ListNodePosi;
    fn insert_before(&mut self, q: &Self::ListNodePosi, e: T) -> Self::ListNodePosi;
    fn insert_after(&mut self, p: &Self::ListNodePosi, e: T) -> Self::ListNodePosi;
    
    fn remove(&mut self, p: &Self::ListNodePosi) -> Option<T>;

}

use std::fmt;
use std::rc::Rc;

use node::ListNode;
use super::node::Node;
use std::default::Default;

#[derive(Debug)]
pub struct LessonList<T> {
    header: ListNode<T>,
    tailer: ListNode<T>,
    size: usize
}

impl<T: fmt::Debug> Default for LessonList<T> {
    fn default() -> Self {
        let h = ListNode::default();
        let t = ListNode::default();
        h.link_after(&t);
        LessonList {
            header: h,
            tailer: t,
            size: 0
        }
    }
}

impl<T: fmt::Debug> LessonList<T> {
    pub fn new() -> Self {
        LessonList::default()
    }
}

impl<T: Eq + fmt::Debug> List<T> for LessonList<T> {
    type ListNodePosi = ListNode<T>;  

    fn size(&self) -> usize {
        self.size
    }

    fn find(&self, e: &T) -> Option<Self::ListNodePosi> {
        self.relative_find(e, self.size, &self.tailer)
    }

    fn relative_find(&self, e: &T, n: usize, posi: &Self::ListNodePosi) -> Option<Self::ListNodePosi> {
        let mut mp = posi.pred();
        for i in 0..n {
            if let Some(p) = mp {
                if p == self.header {
                    println!("Not found");
                    return None;
                }
                if let Some(data) = p.get() {
                    if *data == *e {
                        return Some(p);
                    }
                }
                mp = p.pred();

            } else {
                return None
            }
        }
        None
    }

    fn first(&self) -> Option<Self::ListNodePosi> {
        if let Some(q) = self.header.suss() {
            if q != self.tailer {
                Some(q)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn last(&self) -> Option<Self::ListNodePosi> {
        if let Some(p) = self.tailer.pred() {
            if p != self.header {
                Some(p)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn valid(&self, p: &Self::ListNodePosi) -> bool {
        *p != self.header && *p != self.tailer
    }

    fn pred(&self, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi> {
        if let Some(p) = p.pred() {
            if self.valid(&p) {
                return Some(p);
            }
        }
        None
    }

    fn suss(&self, p: &Self::ListNodePosi) -> Option<Self::ListNodePosi> {
        if let Some(p) = p.suss() {
            if self.valid(&p) {
                return Some(p);
            }
        }
        None
    }

    fn insert_as_first(&mut self, e: T) -> Self::ListNodePosi {
        self.size += 1;
        self.header.insert_as_suss(e)
    }

    fn insert_as_last(&mut self, e: T) -> Self::ListNodePosi {
        self.size += 1;
        self.tailer.insert_as_pred(e)
    }

    fn insert_before(&mut self, q: &Self::ListNodePosi, e: T) -> Self::ListNodePosi {
        self.size += 1;
        q.insert_as_pred(e)
    }

    fn insert_after(&mut self, p: &Self::ListNodePosi, e: T) -> Self::ListNodePosi {
        self.size += 1;
        p.insert_as_suss(e)
    }

    fn remove(&mut self, p: &Self::ListNodePosi) -> Option<T> {
        if self.valid(p) {
            self.size -= 1;
            let pred = p.pred().unwrap();
            let suss = p.suss().unwrap();
            p.clean_after().clean_before();
            pred.combine(&suss);
            if let Some(data) = p.remove_data() {
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }

}

use super::stack::Stack;
impl<T: fmt::Debug + Eq> Stack<T> for LessonList<T> {
    fn push(&mut self, e: T) -> &mut Self {
        self.insert_as_first(e);
        self
    }
    fn pop(&mut self) -> Option<T> {
        if let Some(p) = self.first() {
            self.remove(&p)
        } else {
            None
        }
    }
    fn top(&self) -> Option<&mut T> {
        if let Some(node) = self.first() {
            let ptr = node.as_ptr();
            unsafe {
                ptr.as_ref().unwrap().get_mut()
            }
        } else {
            None
        }
    }
    fn empty(&self) -> bool {
        self.size == 0
    }
}