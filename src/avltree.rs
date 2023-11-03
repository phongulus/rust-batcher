use std::mem::{replace, swap};

pub struct AvlTree<K: Ord, T> {
    root: Option<Box<Node<K, T>>>,
}

struct Node<K: Ord, T> {
    key: K,
    value: T,
    height: usize,
    left: Option<Box<Node<K, T>>>,
    right: Option<Box<Node<K, T>>>,
}

impl <K: Ord, T> Node<K, T> {
    fn new(key: K, value: T) -> Self {
        Node {
            key: key,
            value: value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height_opt(node: &Option<Box<Node<K, T>>>) -> usize {
        match *node {
            Some(ref node) => node.height,
            None => 0,
        }
    }

    fn left_height(&self) -> usize {
        match self.left {
            Some(ref left) => left.height,
            None => 0,
        }
    }

    fn right_height(&self) -> usize {
        match self.right {
            Some(ref right) => right.height,
            None => 0,
        }
    }

    fn balance_factor(&self) -> isize {
        self.left_height() as isize - self.right_height() as isize
    }

    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(self.left_height(), self.right_height());
    }

    fn search_aux(&self, key: K) -> Option<&T> {
        if key == self.key {
            return Some(&self.value);
        } else if key < self.key {
            match self.left {
                Some(ref left) => left.search_aux(key),
                None => None,
            }
        } else {
            match self.right {
                Some(ref right) => right.search_aux(key),
                None => None,
            }
        }
    }

    fn rotate_right(&mut self) {
        if self.left.is_none() {
            panic!("Cannot rotate right with no left child");
        }
        let left_right = self.left.as_mut().unwrap().right.take();
        let mut left = self.left.take();
        swap(self, left.as_mut().unwrap());
        self.right = left;
        self.right.as_mut().unwrap().left = left_right;

        // Update height
        if let Some(node) = self.right.as_mut() {
            node.update_height();
        }
        self.update_height();
    }

    fn rotate_left(&mut self) {
        if self.right.is_none() {
            panic!("Cannot rotate left with no right child");
        }
        let right_left = self.right.as_mut().unwrap().left.take();
        let mut right = self.right.take();
        swap(self, right.as_mut().unwrap());
        self.left = right;
        self.left.as_mut().unwrap().right = right_left;

        // Update height
        if let Some(node) = self.left.as_mut() {
            node.update_height();
        }
        self.update_height();
    }

    fn insert_aux(self: &mut Node<K, T>, new_node: Node<K, T>) {
        if new_node.key == self.key {
            return ();
        } else if new_node.key < self.key {
            match self.left {
                Some(ref mut left) => left.insert_aux(new_node),
                None => {
                    self.left = Some(Box::new(new_node));
                },
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert_aux(new_node),
                None => {
                    self.right = Some(Box::new(new_node));
                },
            }
        }
        self.height = 1 + std::cmp::max(self.left_height(), self.right_height());
        let balance = self.balance_factor();
        if balance > 1 {
            if self.left.as_mut().unwrap().left_height() > self.left.as_mut().unwrap().right_height() {
                self.rotate_right();
            } else {
                self.left.as_mut().unwrap().rotate_left();
                self.rotate_right();
            }
        } else if balance < -1 {
            if self.right.as_mut().unwrap().right_height() > self.right.as_mut().unwrap().left_height() {
                self.rotate_left();
            } else {
                self.right.as_mut().unwrap().rotate_right();
                self.rotate_left();
            }
        }
    }

    fn join_right(&mut self, lt: &mut Option<Box<Node<K, T>>>, rt: &mut Option<Box<Node<K, T>>>) {
        let l = lt.as_mut().unwrap().left.take();
        let mut c = lt.as_mut().unwrap().right.take();
        let k = lt;
        if Self::height_opt(&c) <= Self::height_opt(rt) + 1 {
            // After the swap, self = k' and k = k
            swap(self, k.as_mut().unwrap());
            k.as_mut().unwrap().left = c;
            swap(&mut k.as_mut().unwrap().right, rt);
            if Self::height_opt(k) < Self::height_opt(&l) + 1 {
                self.left = l;
                swap(self, k.as_mut().unwrap());
            } else {
                k.as_mut().unwrap().rotate_right();
                self.left = l;
                swap(self, k.as_mut().unwrap());
                self.rotate_left();
            }
        } else {
            swap(self, k.as_mut().unwrap());
            k.as_mut().unwrap().join_right(&mut c, rt);
            let height_check = Self::height_opt(k) <= Self::height_opt(&l) + 1;
            self.left = l;
            swap(self, k.as_mut().unwrap());
            if !height_check {
                k.as_mut().unwrap().rotate_left();
            }
        }
    }

    fn join_left(&mut self, lt: &mut Option<Box<Node<K, T>>>, rt: &mut Option<Box<Node<K, T>>>) {
        let r = rt.as_mut().unwrap().right.take();
        let mut c = rt.as_mut().unwrap().left.take();
        let k = rt;
        if Self::height_opt(&c) <= Self::height_opt(lt) + 1 {
            swap(self, k.as_mut().unwrap());
            k.as_mut().unwrap().right = c;
            swap(&mut k.as_mut().unwrap().left, lt);
            if Self::height_opt(k) < Self::height_opt(&r) + 1 {
                self.right = r;
                swap(self, k.as_mut().unwrap());
            } else {
                k.as_mut().unwrap().rotate_left();
                self.right = r;
                swap(self, k.as_mut().unwrap());
                self.rotate_right();
            }
        } else {
            swap(self, k.as_mut().unwrap());
            k.as_mut().unwrap().join_left(lt, &mut c);
            let height_check = Self::height_opt(k) <= Self::height_opt(&r) + 1;
            self.right = r;
            swap(self, k.as_mut().unwrap());
            if !height_check {
                k.as_mut().unwrap().rotate_right();
            }
        }
    }

    fn join(&mut self, lt: &mut Option<Box<Node<K, T>>>, rt: &mut Option<Box<Node<K, T>>>) {
        if Self::height_opt(&lt) > Self::height_opt(&rt) + 1 {
            self.join_right(lt, rt)
        } else if Self::height_opt(&rt) > Self::height_opt(&lt) + 1 {
            self.join_left(lt, rt)
        } else {
            swap(&mut self.left, lt);
            swap(&mut self.right, rt);
            self.update_height();
        }
    }

    // fn set_left_child(&mut self, child: &mut Option<Box<Node<K, T>>>) {
    //     match child {
    //         Some(ref mut child) => {
    //             self.left = Some(*child);
    //             // child.parent = Some(Box::new(*self))
    //         },
    //         None => (),
    //     }
    // }

    // fn set_right_child(&mut self, child: &mut Option<Box<Node<K, T>>>) {
    //     match *child {
    //         Some(mut child) => {
    //             self.right = Some(child);
    //             // child.parent = Some(Box::new(*self))
    //         },
    //         None => (),
    //     }
    // }

    // fn insert_aux(&mut self, new_node: Node<K, T>) {
    //     if new_node.key < self.key {
    //         match self.left {
    //             Some(ref mut left) => left.insert_aux(new_node),
    //             None => {
    //                 self.left = Some(Box::new(new_node));
    //                 // self.set_height(2);
    //             },
    //         }
    //     } else {
    //         match self.right {
    //             Some(ref mut right) => right.insert_aux(new_node),
    //             None => {
    //                 self.right = Some(Box::new(new_node));
    //                 // self.set_height(2);
    //             },
    //         }
    //     }
    //     self.height = 1 + std::cmp::max(self.left_height(), self.right_height());
    //     let balance = self.balance_factor();
    //     if balance > 1 && new_node.key < self.left.unwrap().key {
    //         self.right_rotate();
    //     }
    // }
}

impl <K: Ord, T> AvlTree<K, T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
        }
    }

    // let rotate_right x t =
    //     let y = left x in
    //     set_child x Left (right y);
    //     if right y != Leaf then set_parent (right y) x;
    //     set_parent y (parent x);
    //     if parent x = Leaf then t.root <- y
    //     else if x == right @@ parent x then set_child (parent x) Right y
    //     else set_child (parent x) Left y;
    //     set_child y Right x;
    //     set_height x @@ 1 + max (height @@ left x) (height @@ right x);
    //     set_height y @@ 1 + max (height @@ left y) (height @@ right y)

    // fn rotate_right(&mut self, current_node: *mut Node<K, T>, x: *mut Box<Node<K, T>>) {
    //     unsafe {
    //         let mut y = (*current_node).left.as_mut().unwrap();
    //         (*current_node).left = y.right.take();
    //         // if (*current_node).left.is_some() {
    //         //     (*current_node).left.as_mut().unwrap().parent = Some(Box::new(*current_node));
    //         // }
    //         // y.parent = (*current_node).parent;
    //         if (*current_node).parent.is_none() {
    //             self.root = Some(y);
    //         } else if (*current_node).parent.as_ref().unwrap().right.as_ref().unwrap().key == (*current_node).key {
    //             (*current_node).parent.as_mut().unwrap().right = Some(y);
    //         } else {
    //             (*current_node).parent.as_mut().unwrap().left = Some(y);
    //         }
    //         y.right = Some(Box::new(*current_node));
    //         (*current_node).parent = Some(y);
    //         (*current_node).height = 1 + std::cmp::max(
    //             (*current_node).left_height(), (*current_node).right_height());
    //         y.height = 1 + std::cmp::max(
    //             y.left_height(), y.right_height());
    //     }
    //     // let mut y = x.left.as_mut().unwrap();
    //     // x.left = y.right.take();
    //     // if current_node
    //     // node.set_right_child(&mut left.left);
    //     // node.left = left.right;
    //     // left.right = Some(node);
    //     // left.parent = node.parent;
    //     // node.parent = Some(left);
    // }

    // fn insert_aux(&mut self, current_node: &mut Node<K, T>, new_node: Node<K, T>) {
    //     // let nn_key = &new_node.key;
    //     if new_node.key == current_node.key {
    //         return ();
    //     } else if new_node.key < current_node.key {
    //         match current_node.left {
    //             Some(ref mut left) => self.insert_aux(left, new_node),
    //             None => {
    //                 current_node.left = Some(Box::new(new_node));

    //                 // Balance tree
    //                 current_node.height = 1 + std::cmp::max(
    //                     current_node.left_height(), current_node.right_height());
    //                 let balance = current_node.balance_factor();

    //                 if balance < -1 {
    //                     if current_node.left.as_ref().unwrap().key > current_node.right.as_ref().unwrap().key {
    //                         current_node.rotate_left();
    //                     } else if current_node.left.as_ref().unwrap().key < current_node.right.as_ref().unwrap().key {
    //                         current_node.right.as_mut().unwrap().rotate_right();
    //                         current_node.rotate_left()
    //                     }
    //                 }

    //                 // if balance > 1 && key new_node < key (left current_node) then
    //                 //     rotate_right current_node t
    //                 // else if balance < -1 && key new_node > key (right current_node) then
    //                 //     rotate_left current_node t
    //                 // else if balance > 1 && key new_node > key (left current_node) then
    //                 //     (rotate_left (left current_node) t; rotate_right current_node t)
    //                 // else if balance < -1 && key new_node < key (right current_node) then
    //                 //     (rotate_right (right current_node) t; rotate_left current_node t)
    //                 // let nn = current_node.left.as_ref().unwrap()
    //                 //     as *const Box<Node<K, T>> as *mut Box<Node<K, T>>;
    //                 // let cn = current_node as *const Node<K, T> as *mut Node<K, T>;
    //                 // self.balance_tree(cn, nn);
    //             },
    //         }
    //     } else {
    //         match current_node.right {
    //             Some(ref mut right) => self.insert_aux(right, new_node),
    //             None => {
    //                 current_node.right = Some(Box::new(new_node));

    //                 // Update height and get balance factor
    //                 current_node.height = 1 + std::cmp::max(
    //                     current_node.left_height(), current_node.right_height());
    //                 let balance = current_node.balance_factor();

    //                 // Balance tree
    //                 if balance > 1 {
    //                     if current_node.right.as_ref().unwrap().key > current_node.right.as_ref().unwrap().key {
    //                         current_node.rotate_right();
    //                     } else if current_node.left.as_ref().unwrap().key < current_node.right.as_ref().unwrap().key {
    //                         current_node.left.as_mut().unwrap().rotate_left();
    //                         current_node.rotate_right()
    //                     }
    //                 }
    //             },
    //         }
    //     }
    // }

    pub fn insert(&mut self, key: K, value: T) {
        let new_node = Node::new(key, value);
        if self.root.is_none() {
            self.root = Some(Box::new(new_node));
        } else {
            let root = self.root.as_mut().unwrap();
            root.insert_aux(new_node);
        }
    }

    pub fn search(&self, key: K) -> Option<&T> {
        match self.root {
            Some(ref root) => root.search_aux(key),
            None => None,
        }
    }

    
}