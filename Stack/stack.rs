struct Node<T>{
    entry: T,
    next: NodeChild<T>,
}
struct NodeChild<T>(Option<Box<Node<T>>>);
impl<T> NodeChild<T>{fn new() -> Self {Self(None)}}
impl<T> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: NodeChild::new()}
    }
}

struct Stack<T>{
    top: NodeChild<T>
}
impl<T> Stack<T>{
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
    pub fn peek(&self) -> Option<&T>{
        self.top.0.as_ref().map(|top| {&top.entry})
    }
    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.top.0.as_mut().map(|top| {&mut top.entry})
    }
}
impl<T> Drop for Stack<T>{
    fn drop(&mut self) {
        let mut cur_node = self.top.0.take();
        while let Some(mut node) = cur_node {
            cur_node = node.next.0.take();
        }
    }
}
