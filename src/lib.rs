use rand;
mod avltree;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::avltree::AvlTree;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn avltree_basic_test() {
        let mut t: AvlTree<i32, i32> = avltree::AvlTree::new();
        t.insert(1, 1);
        t.insert(2, 2);
        t.insert(3, 3);
        t.insert(4, 4);
        t.insert(-10, -10);
        t.insert(-3, -3);

        assert!(t.search(1).is_some() && t.search(1) == Some(&1));
        assert!(t.search(2).is_some() && t.search(2) == Some(&2));
        assert!(t.search(3).is_some() && t.search(3) == Some(&3));
        assert!(t.search(4).is_some() && t.search(4) == Some(&4));
        assert!(t.search(-10).is_some() && t.search(-10) == Some(&-10));
        assert!(t.search(-3).is_some() && t.search(-3) == Some(&-3));
    }

    #[test]
    fn avltree_random_test() {
        let mut t: AvlTree<i32, i32> = avltree::AvlTree::new();
        let mut v: Vec<i32> = Vec::new();
        let maxn = 1000000;
        for _ in 0..maxn {
            let r = rand::random::<i32>();
            t.insert(r, r);
            v.push(r);
        }

        for i in 0..maxn {
            assert!(t.search(v[i]).is_some() && t.search(v[i]) == Some(&v[i]));
        }
    }
}
