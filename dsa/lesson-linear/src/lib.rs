use std::fmt;

pub type Rank = usize;

pub trait Vector<T>: fmt::Display
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
pub mod list;
pub mod node;

#[cfg(test)]
mod tests {
    mod vector_test {
        use crate::Vector;
        use std::fmt;

        fn test_print<E: Ord+fmt::Display, T: Vector<E>>(v: &T, s: &str) {
            assert_eq!(format!("{}", v), s)
        }

        fn test_size_empty<E: Ord+fmt::Display, T: Vector<E>>(v: &T) {
            assert_eq!(v.size(), 0);
            assert_eq!(v.empty(), true);
        }

        fn test_insert_i32<T: Vector<i32>>(v: &mut T) {
            // 初始时，size应为0
            assert_eq!(v.size(), 0);
            test_print(v, "[]");

            // 插入1个
            v.insert_one(0);
            assert_eq!(v.size(), 1);
            test_print(v, r#"[0]"#);

            // 再插入2个
            v.insert_one(1).insert_one(-1);
            assert_eq!(v.size(), 3);
            test_print(v, r#"[0,1,-1]"#);

            // 插入98个
            for i in 2..=99 {
                v.insert_one(i);
            }
            assert_eq!(v.size(), 101);
            test_print(v, r#"[0,1,-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99]"#);

            // 在第45个插入
            v.insert(44, 102);
            assert_eq!(v.size(), 102);
            test_print(v, r#"[0,1,-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,102,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99]"#);

            // 再在第45个插入两次
            v.insert(44, 103).insert(44, 104);
            assert_eq!(v.size(), 104);
            test_print(v, r#"[0,1,-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,104,103,102,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99]"#);

        }

        fn test_remove_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            test_insert_i32(v);
            
            // 删除1个
            v.remove_one(v.size() - 1);
            assert_eq!(v.size(), 103);
            test_print(v, r#"[0,1,-1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,104,103,102,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98]"#);

            // 删除一群
            v.remove(4, 10);
            assert_eq!(v.size(), 97);
            test_print(v, r#"[0,1,-1,2,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,104,103,102,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98]"#);

            // 删除全部
            v.remove(0, v.size());
            assert_eq!(v.size(), 0);
            test_print(v, "[]");
        }

        fn test_get_set_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            test_insert_i32(v);

            // 取第4个
            let e: Option<&i32> = v.get(3);
            assert_eq!(*(e.unwrap()), 2);

            // 取第size + 1个
            let e: Option<&i32> = v.get(v.size() + 1);
            assert_eq!(e, None);

            // 置第4个
            v.set(4, 10);
            assert_eq!(*(v.get(4).unwrap()), 10);

            // 连续置第4个
            v.set(4, 11).set(4, 12);
            assert_eq!(*(v.get(4).unwrap()), 12);

        }

        fn test_disordered_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            for i in 1..55 {
                v.insert_one(i);
            }

            // 应为有序
            assert_eq!(v.disordered(), 0);

            // 应为无序
            v.insert(0, 55).insert(0, 56);
            assert_eq!(v.disordered(), 2);
        }

        fn test_sort_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            for i in 1..=7 {
                v.insert_one(i);
            }
            v.insert(0, 7).insert(0, 9);
            
            // 部分排序
            v.sort(1, v.size());
            test_print(v, r#"[9,1,2,3,4,5,6,7,7]"#);

            // 全部排序
            v.sort_fully();
            test_print(v, r#"[1,2,3,4,5,6,7,7,9]"#);

        }

        fn test_find_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            test_insert_i32(v);
            
            // 部分查找
            assert_eq!(v.find(&(-1), 1, 4).unwrap(), 2);
            assert_eq!(v.find(&(-1), 4, 9), None);

            // 全部查找
            assert_eq!(v.find_fully(&(-1)).unwrap(), 2);
            assert_eq!(v.find_fully(&(10000)), None);

        }

        fn test_search_i32<T: Vector<i32>>(v: &mut T) {
            // 初始化
            test_insert_i32(v);
            v.sort_fully();
            
            // 部分查找
            assert_eq!(v.search(&(-1), 1, 4), None);
            assert_eq!(v.search(&(2), 0, 9).unwrap(), 3);

            // 全部查找
            assert_eq!(v.search_fully(&(100)).unwrap(), 100);
            assert_eq!(v.search_fully(&(10000)).unwrap(), 103);
            assert_eq!(v.search_fully(&(3)).unwrap(), 4);
            assert_eq!(v.search_fully(&(-2)), None);

        }

        fn test_insert_str<T: Vector<String>>(v: &mut T) {
            // 初始时，size应为0
            assert_eq!(v.size(), 0);
            test_print(v, r#"[]"#);

            // 插入1个
            v.insert_one(String::from("Vector test"));
            assert_eq!(v.size(), 1);
            test_print(v, r#"[Vector test]"#);

            // 再插入1个
            v.insert_one(String::from("Vector test 2"));
            assert_eq!(v.size(), 2);
            test_print(v, r#"[Vector test,Vector test 2]"#);

            // 再插入5个
            for c in "ABCDE".chars() {
                v.insert_one(c.to_string());
            }
            assert_eq!(v.size(), 7);
            test_print(v, r#"[Vector test,Vector test 2,A,B,C,D,E]"#);

            // 在第4个位置插入
            v.insert(3, String::from("F"));
            assert_eq!(v.size(), 8);
            test_print(v, r#"[Vector test,Vector test 2,A,F,B,C,D,E]"#);
        }

        fn test_remove_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            test_insert_str(v);
            
            // 删除1个
            v.remove_one(v.size() - 1);
            assert_eq!(v.size(), 7);
            test_print(v, r#"[Vector test,Vector test 2,A,F,B,C,D]"#);

            // 删除一群
            v.remove(1, 3);
            assert_eq!(v.size(), 5);
            test_print(v, r#"[Vector test,F,B,C,D]"#);

            // 删除全部
            v.remove(0, v.size());
            assert_eq!(v.size(), 0);
            test_print(v, "[]");
        }

        fn test_get_set_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            test_insert_str(v);

            // 取第4个
            let e: Option<&String> = v.get(3);
            assert_eq!(*(e.unwrap()), "F".to_string());

            // 取第size + 1个
            let e: Option<&String> = v.get(v.size() + 1);
            assert_eq!(e, None);

            // 置第4个
            v.set(4, "XFFF".to_string());
            assert_eq!(*(v.get(4).unwrap()), "XFFF".to_string());

            // 连续置第4个
            v.set(4, "ABBB".to_string()).set(4, "ASDFDF".to_string());
            assert_eq!(*(v.get(4).unwrap()), "ASDFDF");

        }
        fn test_disordered_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            for c in "ABCDEFGHIJKL".chars() {
                v.insert_one(c.to_string());
            }

            // 应为有序
            assert_eq!(v.disordered(), 0);

            // 应为无序
            v.insert(0, "M".to_string()).insert(0, "N".to_string());
            assert_eq!(v.disordered(), 2);
        }

        fn test_sort_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            for c in "ABCDEFG".chars() {
                v.insert_one(c.to_string());
            }
            v.insert(0, "G".to_string()).insert(0, "H".to_string());
            
            // 部分排序
            v.sort(1, v.size());
            test_print(v, r#"[H,A,B,C,D,E,F,G,G]"#);

            // 全部排序
            v.sort_fully();
            test_print(v, r#"[A,B,C,D,E,F,G,G,H]"#);

        }

        fn test_find_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            test_insert_str(v);
            
            // 部分查找
            assert_eq!(v.find(&String::from("Vector test 2"), 1, 4).unwrap(), 1);
            assert_eq!(v.find(&String::from("Vector test 2"), 4, 9), None);

            // 全部查找
            assert_eq!(v.find_fully(&String::from("Vector test 2")).unwrap(), 1);
            assert_eq!(v.find_fully(&String::from("Vector test 3")), None);

        }

        fn test_search_str<T: Vector<String>>(v: &mut T) {
            // 初始化
            test_insert_str(v);
            v.sort_fully();

            // 部分查找
            assert_eq!(v.search(&("Vector test".to_string()), 1, 4).unwrap(), 3);
            assert_eq!(v.search(&("B".to_string()), 0, 4).unwrap(), 1);
            assert_eq!(v.search(&("E".to_string()), 0, 4).unwrap(), 3);

            // 全部查找
            assert_eq!(v.search_fully(&("Vector test 2".to_string())).unwrap(), 7);
            assert_eq!(v.search_fully(&("G".to_string())).unwrap(), 5);
            assert_eq!(v.search_fully(&("Vector test 1".to_string())).unwrap(), 6);
            assert_eq!(v.search_fully(&("".to_string())), None);

        }

        mod vec_vector_test {
            use crate::tests::vector_test;
            use crate::vec_vector::VecVector;
            #[test]
            fn test_new() {
                // Copy
                let v: VecVector<i32> = VecVector::new(); // direct
                vector_test::test_print(&v, "[]");

                let mut v: VecVector<i32>;                // mutable
                v = VecVector::new();
                vector_test::test_print(&v, "[]");

                // Not Copy
                let v: VecVector<String> = VecVector::new();
                vector_test::test_print(&v, "[]");

                let mut v: VecVector<String>;
                v = VecVector::new();
                vector_test::test_print(&v, "[]");
            }

            #[test]
            fn test_size_empty() {
                let v: VecVector<i32> = VecVector::new();
                let w: VecVector<String> = VecVector::new();

                vector_test::test_size_empty(&v);
                vector_test::test_size_empty(&w);
            }

            #[test]
            fn test_insert() {
                let mut v: VecVector<i32> = VecVector::new();
                let mut w: VecVector<String> = VecVector::new();

                vector_test::test_insert_i32(&mut v);
                vector_test::test_insert_str(&mut w);
            }

            #[test]
            fn test_remove() {
                let mut v: VecVector<i32> = VecVector::new();
                let mut w: VecVector<String> = VecVector::new();

                vector_test::test_remove_i32(&mut v);
                vector_test::test_remove_str(&mut w);
            }

            #[test]
            fn test_get_set() {
                let mut v: VecVector<i32> = VecVector::new();
                let mut w: VecVector<String> = VecVector::new();

                vector_test::test_get_set_i32(&mut v);
                vector_test::test_get_set_str(&mut w);
            }

            #[test]
            fn test_disordered() {
                let mut v: VecVector<i32> = VecVector::new();
                vector_test::test_disordered_i32(&mut v);

                let mut w: VecVector<String> = VecVector::new();
                vector_test::test_disordered_str(&mut w);
            }

            #[test]
            fn test_sort() {
                let mut v: VecVector<i32> = VecVector::new();
                vector_test::test_sort_i32(&mut v);

                let mut w: VecVector<String> = VecVector::new();
                vector_test::test_sort_str(&mut w);
            }

            #[test]
            fn test_find() {
                let mut v: VecVector<i32> = VecVector::new();
                vector_test::test_find_i32(&mut v);

                let mut w: VecVector<String> = VecVector::new();
                vector_test::test_find_str(&mut w);
            }

            #[test]
            fn test_search() {
                let mut v: VecVector<i32> = VecVector::new();
                vector_test::test_search_i32(&mut v);

                let mut w: VecVector<String> = VecVector::new();
                vector_test::test_search_str(&mut w);
            }
        }
    }
}
