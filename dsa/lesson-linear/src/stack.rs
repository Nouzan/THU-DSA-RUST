
pub trait Stack<T> {
    fn push(&mut self, e: T) -> &mut Self;
    fn pop(&mut self) -> Option<T>;
    fn top(&self) -> Option<&mut T>;
    fn empty(&self) -> bool;
}