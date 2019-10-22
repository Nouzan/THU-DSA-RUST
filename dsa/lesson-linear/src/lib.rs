pub type Rank = usize;

pub trait Vector<T>
    where T: Ord {
    fn size(&self) -> usize;
    fn empty(&self) -> bool {
        self.size() == 0
    }
    fn disordered(&self) -> usize;
    fn find_fully(&self, e: &T) -> Option<Rank> {
        self.find(e, 0, self.size())
    }
    fn find(&self, e: &T, lo: Rank, hi: Rank) -> Option<Rank>;
    fn search_fully(&self, e: &T) -> Option<Rank> {
        self.search(e, 0, self.size())
    }
    fn search(&self, e: &T, lo: Rank, hi: Rank) -> Option<Rank>;
    fn get(&self, r: Rank) -> Option<&T>;
    fn set(&mut self, r: Rank, e: T) -> &mut Self;
    fn remove_one(&mut self, r: Rank) -> Option<T>;
    fn remove(&mut self, lo: Rank, hi: Rank) -> usize;
    fn insert(&mut self, r: Rank, e: T) -> &mut Self;
    fn insert_one(&mut self, e: T) -> &mut Self {
        self.insert(self.size(), e)
    }
    fn sort_fully(&mut self) -> &mut Self {
        self.sort(0, self.size())
    }
    fn sort(&mut self, lo: Rank, hi: Rank) -> &mut Self;
    
}

pub mod vec_vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
