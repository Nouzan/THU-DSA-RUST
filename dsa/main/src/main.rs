use lesson_linear::Vector;
use lesson_linear::vec_vector::VecVector;
// use lesson_linear::list::node::ListNode;
use lesson_linear::list::RcList;
use lesson_linear::list::List;
use lesson_linear::list::node::flat_node::{ListNode, RawNode};
use lesson_linear::node::Node;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // let a = ListNode::new(1);
    // a.insert_as_suss(2);
    // a.insert_as_suss(3);
    // a.insert_as_pred(4);
    // a.suss().unwrap().insert_as_pred(5);
    // let b = a.suss().unwrap();
    // a.suss().unwrap().remove_ownership();
    // a.clear_after();
    // // a.remove_ownership();

    // println!("{:?}", a);
    // println!("{:?}", b);

    // a.insert_ownership(&b);

    // println!("{:?}", a);
    // println!("{:?}", b);
    let a = ListNode::new(1);
    let b = ListNode::new(2);
    // a.insert_ownership(&b);
    // b.insert_ownership(&a);
    println!("{:?} {:?}", a, b);
    a.link_after(&b);
    println!("{:?} {:?}", a, b);
    a.insert_as_pred(3);
    let n = a.remove_node();
    println!("{:?} {:?}", a, n);
    a.set_node(n.unwrap());
    println!("{:?} {:?}", a, a.get_node());
}