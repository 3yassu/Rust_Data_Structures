use core::ptr::NonNull;
#[derive(Debug)]
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>,
}
type NodeChild<T: Ord> = Option<NonNull<Node<T>>>;
impl<T: Ord> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: None}
    }
}
#[derive(Debug)]
struct Queue<T: Ord>{
    front: NodeChild<T>,
    back: NodeChild<T>
}
impl<T: Ord + std::fmt::Display> Queue<T>{
    pub fn new() -> Self{
        Self{front: None, back: None}
    }
    pub fn is_empty(&self) -> bool{
        match &self.front {
            None => true,
            Some(_node) => false,
        }
    }
    pub fn enqueue(&mut self, entry: T){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match self.back.take(){
                None => {
                    self.front = Some(node);
                    self.back = Some(node);
                },
                Some(old) => {
                    (*old.as_ptr()).next = Some(node);
                    self.back = Some(node);          
                }
            }
        }
    }
    pub fn dequeue(&mut self) -> Option<T>{
        unsafe {
            self.front.map(|node| {
                let boxed = Box::from_raw(node.as_ptr()); let result = boxed.entry;
                self.front = boxed.next;
                match &self.front{
                    None => self.back = None,
                    Some(_val) => {}
                }
                result
            })
        }
    }
    pub fn peek_front(&self) -> Option<&T>{
        unsafe{self.front.map(|node| &(*node.as_ptr()).entry)}
    }
}