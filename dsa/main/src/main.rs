use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
// use lesson_linear::list::node::ListNode;
use lesson_linear::list::RcList;
use lesson_linear::list::List;
use lesson_linear::list::node::flat_node::ListNode;

use std::rc::Rc;

fn main() {
    // let mut a: RcList<i32> = RcList::new();
    // let b = a.insert_as_first(11);
    // let c = a.insert_as_first(12);
    // println!("{:?}", b.borrow().pred().unwrap());
    let a = ListNode::new(15);
    let b = ListNode::new(16);
    a.link(&b);
    let c = b.pred();

    println!("{:?}", c)
}