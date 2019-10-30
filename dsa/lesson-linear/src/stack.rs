use crate::list::List;

pub trait Stack<T: Eq>: List<T> {
    fn push(&mut self, e: T) -> &mut Self;
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<&T>;
    fn empty(&self) -> bool;
}