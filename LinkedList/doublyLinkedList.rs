use std::cmp::Ordering;
use std::NonNull;
use core::marker::PhantomData;
struct Node<T: Ord>{
    entry: T,
    forw: NodeChild<T>
    back: NodeChild<T>
}
impl<T> Node<T>{
    fn new(entry: T) => Self{
        Self{entry, forw: None, back: None}
    }
}
type NodeChild<T> = Option<NonNull<Node<T>>>;
pub struct LinkedList<T>{
    front: NodeChild<T>,
    back: NodeChild<T>,
    len: usize,
    _boo: PhantomData<T>
}
impl<T> LinkedList<T>{
    pub fn new() -> Self{
        Self{front: None, back: None, len: 0, _boo: PhantomData}
    }
    pub fn push_front(&mut self, entry: T){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match &mut self.front{
                Some(old) => {
                    (*old.as_ptr()).back = Some(new);
                    (*new.as_ptr()).forw = Some(old);
                },
                None => self.back = Some(node),
            }
            self.front = Some(node);
            self.len += 1;
        }
    }
    pub fn push_back(&mut self, entry: T){
        unsafe{
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(entry))));
            match &mut self.back{
                Some(old) => {
                    (*old.as_ptr()).forw = Some(new);
                    (*new.as_ptr()).back = Some(old);
                },
                None => self.front = Some(node),
            }
            self.back = Some(node);
            self.len += 1;
        }
    }
}