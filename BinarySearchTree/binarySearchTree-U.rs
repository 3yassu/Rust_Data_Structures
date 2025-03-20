use std::cmp::Ordering;
use std::ptr::NonNull;
struct BinaryNode<T: Ord>{
    entry: T,
    left: BinaryChild<T>,
    right: BinaryChild<T>
}
type BinaryChild<T: Ord> = (Option<NonNull<BinaryNode<T>>>);
impl<T: Ord> BinaryNode<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, left: None, right: None}
    }
}

struct BinST<T: Ord>{
    root: BinaryChild<T>
}
impl<T: Ord + std::fmt::Display> BinST<T>{
    pub fn new() -> Self{
        Self{root: BinaryChild::new()}
    }
    fn maxLST(current_node: &mut BinaryChild<T>) -> T{
        match &mut current_node.0.as_mut().unwrap().right.0 {
            None => {
                let returnVal = current_node.0.take();
                returnVal.unwrap().entry
            },
            Some(node) => Self::maxLST(&mut node.right),
        }
    }
    fn minRST(current_node: &mut BinaryChild<T>) -> T{
        match &mut current_node.0.as_mut().unwrap().left.0 {
            None => {
                let returnVal = current_node.0.take();
                returnVal.unwrap().entry
            },
            Some(node) => Self::minRST(&mut node.left),
        }
    }
    fn recInsert(entry: T, current_node: &mut BinaryChild<T>){
        match &mut current_node.0 {
            None => current_node.0 = Some(Box::new(BinaryNode::new(entry))),
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => Self::recInsert(entry, &mut current.left),
                Ordering::Equal => {},
                Ordering::Greater => Self::recInsert(entry, &mut current.right),
            },
        }
    }
    fn recSearch(entry: T, current_node: &BinaryChild<T>) -> Option<&T>{
        match &current_node.0 {
            None => None,
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => Self::recSearch(entry, &current.left),
                Ordering::Equal => Some(&current.entry),
                Ordering::Greater => Self::recSearch(entry, &current.right),
            },
        }
    }
    fn recRemove(entry: T, current_node: &mut BinaryChild<T>){
        match &mut current_node.0 {
            None => {}
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => Self::recRemove(entry, &mut current.left),
                Ordering::Equal => {
                    match (&mut current.left.0, &mut current.right.0){
                        (None, None) => {
                            current_node.0 = None;
                        },
                        (Some(_left), None) => {
                            current_node.0.as_mut().unwrap().entry = Self::maxLST(&mut current.left);
                        },
                        (_, Some(_right)) => {
                            current_node.0.as_mut().unwrap().entry = Self::minRST(&mut current.right);
                        }
                    }
                }
                Ordering::Greater => Self::recRemove(entry, &mut current.right),
            },
        }
    }
    fn recOrder(func: fn(&T), current_node: &BinaryChild<T>, order: i32){
        let Some(entry) = &current_node.0 else{return;};
        if order == 0 {func(&entry.entry);}
        Self::recOrder(func, &current_node.0.as_ref().unwrap().left, order);
        if order == 1 {func(&entry.entry);}
        Self::recOrder(func, &current_node.0.as_ref().unwrap().right, order);
        if order == 2 {func(&entry.entry);}
    }
    pub fn insert(&mut self, entry: T){
        if let Some(_root) = &mut self.root.0 {
            let root_mut = &mut self.root;
            Self::recInsert(entry, root_mut);
        }
        else{
            self.root.0 = Some(Box::new(BinaryNode::new(entry)));
        }
    }
    pub fn search(&self, entry: T) -> Option<&T>{
        if let Some(_root) = &self.root.0{
            let root_mut = &self.root;
            Self::recSearch(entry, root_mut)
        }else{
            None
        }
        
    }
    pub fn remove(&mut self, entry: T){
        let root_mut = &mut self.root;
        Self::recRemove(entry, root_mut);
    }
    pub fn order(&self, func: fn(&T), order: &str){
        let root_mut = &self.root;
        if order == "pre" {Self::recOrder(func, root_mut, 0);}
        else if order == "in" {Self::recOrder(func, root_mut, 1);}
        else if order == "post" {Self::recOrder(func, root_mut, 2);}
    }
}