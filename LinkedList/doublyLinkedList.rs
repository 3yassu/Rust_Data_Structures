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
    pub fn pop_front(&mut self) -> Option<T>{
        unsafe{
            self.front.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.front = boxed.forw;
                match &mut self.front{
                    Some(new) => (*new.as_ptr()).back = None,
                    None => self.back = None,
                }
                self.len -= 1;
                return_entry
            })
        }
    }
    pub fn pop_back(&mut self) -> Option<T>{
        unsafe{
            self.back.map( |node| {
                let boxed = Box::from_raw(node.as_ptr());
                let return_entry = boxed.entry;
                self.front = boxed.forw;
                match &mut self.front{
                    Some(new) => (*new.as_ptr()).forw = None,
                    None => self.front = None,
                }
                self.len -= 1;
                return_entry
            })
        }
    }
    pub fn peek_front(&self) -> Option<&T>{
        unsafe{self.front.map(|node| &(*node.as_ptr()).entry)}
    }
    pub fn peek_front_mut(&mut self) -> Option<&mut T>{
        unsafe{self.front.map(|node| &mut (*node.as_ptr()).entry)}
    }
    pub fn peek_back(&self) -> Option<&T>{
        unsafe{self.back.map(|node| &(*node.as_ptr()).entry)}
    }
    pub fn peek_back_mut(&mut self) -> Option<&mut T>{
        unsafe{self.back.map(|node| &mut (*node.as_ptr()).entry)}
    }
    pub fn len(&self) -> usize{
        self.len
    }
    pub fn is_empty(&self) -> bool{
        self.len == 0
    }
    pub fn clear(&mut self){
        while self.pop_front().is_some(){}
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self){
        while self.pop_front().is_some(){}
    }
}