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

struct Stack<T: Ord>{
    top: NodeChild<T>
}
impl<T: Ord + std::fmt::Display> Stack<T>{
    pub fn new() -> Self{
        Self{top: NodeChild::new()}
    }
    pub fn is_empty(&self) -> bool{
        match &self.top.0 {
            None => true,
            Some(_node) => false,
        }
    }
    pub fn push(&mut self, entry: T){
        let mut newTop = NodeChild::new();
        newTop.0 = Some(Box::new(Node::new(entry)));
        newTop.0.as_mut().unwrap().next = NodeChild(self.top.0.take());
        self.top = newTop;
    }
    pub fn pop(&mut self){
        self.top = NodeChild(self.top.0.as_mut().unwrap().next.0.take());
    }
    pub fn peek(&mut self) -> &T{
        &self.top.0.as_mut().unwrap().entry
    }
}