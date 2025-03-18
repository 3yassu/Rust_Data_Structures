use std::rc::Rc;
use std::cell::RefCell;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>,
}
struct NodeChild<T: Ord>(Option<Rc<RefCell<Node<T>>>>);
impl<T:Ord> NodeChild<T>{fn new() -> Self {Self(None)}}
impl<T: Ord> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: NodeChild::new()}
    }
}

struct Queue<T: Ord>{
    front: NodeChild<T>,
    back: NodeChild<T>
}
impl<T: Ord + std::fmt::Display> Queue<T>{
    pub fn new() -> Self{
        Self{front: NodeChild::new(), back: NodeChild::new()}
    }
    pub fn is_empty(&self) -> bool{
        match &self.front.0 {
            None => true,
            Some(_node) => false,
        }
    }
    pub fn enqueue(&mut self, entry: T){
        let new_back = Node::new(entry);
        match self.back.0.take(){
            None => {
                self.front.0 = Some(Rc::new(RefCell::new(new_back)));
                self.back.0 = Some(Rc::clone(self.front.0.as_mut().unwrap()));
            },
            Some(back) => {
                back.borrow_mut().next = NodeChild::new();
                back.borrow_mut().next.0 = Some(Rc::new(RefCell::new(new_back)));
                self.back.0 = Some(Rc::clone(back.borrow_mut().next.0.as_ref().unwrap()));            
                
            }
        }
    }
    pub fn dequeue(&mut self){
        match self.front.0.take(){
            None => {},
            Some(front) => self.front.0 = Some(Rc::clone(front.borrow_mut().next.0.as_ref().unwrap()))
        }
    }
    pub fn peek_front(&self) -> Result<&T, bool>{
        match &self.front.0{
            None => Err(false),
            Some(front) => Ok(&front.borrow().entry)
        }
    }
}