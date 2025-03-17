use std::rc::Rc;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>,
}
struct NodeChild<T: Ord>(Option<Box<Node<T>>>);
impl<T:Ord> NodeChild<T>{fn new() -> Self {Self(None)}}
impl<T: Ord> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: NodeChild::new()}
    }
}

struct Queue<T: Ord>{
    front: NodeChild<T>,
    back: &mut NodeChild<T>
}
impl<T: Ord + std::fmt::Display + Clone> Queue<T>{
    pub fn new() -> Self{
        let front = NodeChild::new(); let back = &mut front;
        Self{front, back}
    }
    pub fn is_empty(&self) -> bool{
        match &self.front.0 {
            None => true,
            Some(_node) => false,
        }
    }
    pub fn enqueue(&mut self, entry: T){
        match &mut self.back.0{
            None => {
                    self.front.0.as_mut().unwrap().entry = entry;                       
                }
            Some(back) => {
                back.next = NodeChild::new();
                back.next.0.as_mut().unwrap().entry = entry;
                self.back.0 = back.next.0.take();
            }
        }
    }
}
