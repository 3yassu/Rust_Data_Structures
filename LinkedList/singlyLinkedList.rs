use std::cmp::Ordering;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>
}
struct NodeChild<T: Ord>(Option<Box<Node<T>>>);
impl<T:Ord> NodeChild<T>{fn new() -> Self {Self(None)}}
impl<T: Ord> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: NodeChild::new()}
    }
}

struct SinglyLinkedList<T: Ord>{
    front: NodeChild<T>,
    length: i32
}
impl<T: Ord + std::fmt::Display> SinglyLinkedList<T>{
    pub fn new() -> Self{
        Self{front: NodeChild::new(), length: 0}
    }
    fn recInsert(entry: T, current_node: &mut NodeChild<T>){
        match &mut current_node.0 {
            None => current_node.0 = Some(Box::new(Node::new(entry))),
            Some(current) => Self::recInsert(entry, &mut current.next),
        }
    }
    fn recSearch(entry: T, current_node: &NodeChild<T>) -> Option<&T>{
        match &current_node.0 {
            None => None,
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Equal => Some(&current.entry),
                _ => Self::recSearch(entry, &current.next),
            },
        }
    }
    /*
    fn recRemove(entry: T, current_node: &mut NodeChild<T>){
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
    */
    pub fn insert(&mut self, entry: T){
        if let Some(_root) = &mut self.front.0 {
            let root_mut = &mut self.front;
            Self::recInsert(entry, root_mut);
        }
        else{
            self.front.0 = Some(Box::new(Node::new(entry)));
        }
    }
    pub fn search(&self, entry: T) -> Option<&T>{
        if let Some(_root) = &self.front.0{
            let root_mut = &self.front;
            Self::recSearch(entry, root_mut)
        }else{
            None
        }
        
    }
    /*
    pub fn remove(&mut self, entry: T){
        let root_mut = &mut self.root;
        Self::recRemove(entry, root_mut);
    }
    */
}