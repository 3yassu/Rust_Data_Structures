use core::ptr::NonNull;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>,
}
type NodeChild<T: Ord> = Option<NonNull<Node<T>>>>;
impl<T: Ord> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: None}
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
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match self.back.take(){
                None => {
                    self.front = Some(new);
                    self.back = Some(new);
                },
                Some(old) => {
                    (*old.as_ptr()).next = Some(new);
                    self.back = Some(new);          
                }
            }
        }
    }
    pub fn dequeue(&mut self){
            match self.front.0.take(){
                None => {},
                Some(front) => unsafe{
                    self.front = Some((*front.as_ptr()).next)
                }
            }
    }
    pub fn peek_front(&self) -> Option<Ref<T>>{
        match &self.front.0{
            None => None,
            Some(front) => Some(Ref::map(front.borrow(), |node| &node.entry)),
        }
    }
}