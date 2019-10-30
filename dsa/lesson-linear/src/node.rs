use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};

pub struct Node<T> (Rc<RefCell<T>>);

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node(Rc::new(RefCell::new(data)))
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    pub fn borrow(&self) -> Ref<T> {
        (*self.0).borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        (*self.0).borrow_mut()
    }

    pub fn try_into_inner(self) -> Result<T, Self> {
        match Rc::try_unwrap(self.0) {
            Err(rc) => Err(Node(rc)),
            Ok(cell) => Ok(cell.into_inner())
        }

    }
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node(Rc::clone(&self.0))
    }
}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({:?})", self.borrow())
    }
}