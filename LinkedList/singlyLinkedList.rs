use std::cmp::Ordering;
#[derive(Debug)]
struct Node<T>{
    entry: T,
    next: NodeChild<T>
}
type NodeChild<T> = Option<Box<Node<T>>>;
impl<T> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: None}
    }
}
#[derive(Debug)]
struct SinglyLinkedList<T>{
    front: NodeChild<T>,
    len: usize
}
impl<T: std::fmt::Debug> SinglyLinkedList<T>{
    pub fn new() -> Self{
        Self{front: None, len: 0}
    }
    pub fn push_front(&mut self, entry: T){
        let mut node = Box::new(Node::new(entry));
        if let Some(old) = self.front.take(){
            node.next = Some(old)
        }
        self.front = Some(node);
        self.len += 1;
    }
    pub fn push_back(&mut self, entry: T) {
        let mut node = Box::new(Node::new(entry));
        let mut jumper = &mut self.front;
        while let Some(node) = jumper {
            jumper = &mut node.next;
        }
        *jumper = Some(node);
        self.len += 1;
    }
    pub fn peek_front(&self) -> Option<&T>{
        self.front.as_ref().map(|node| {&node.entry})
    }
    pub fn peek_front_mut(&mut self) -> Option<&mut T>{
        self.front.as_mut().map(|node| {&mut node.entry})
    }
    pub fn peek_index(&mut self, mut index: usize) -> Option<&T> {
        let mut jumper = self.front.as_mut();
        while let Some(node) = jumper {
            if index == 0 {
                return Some(&node.entry);
            }
            jumper = node.next.as_mut();
            index -= 1;
        }
        None
    }
    pub fn peek_index_mut(&mut self, mut index: usize) -> Option<&mut T> {
        let mut jumper = self.front.as_mut();
        while let Some(node) = jumper {
            if index == 0 {
                return Some(&mut node.entry);
            }
            jumper = node.next.as_mut();
            index -= 1;
        }
        None
    }
}
fn main(){
    let mut x = SinglyLinkedList::new();
    x.push_front(3);
    x.push_back(5);
    x.push_back(6);
    x.push_front(8);
    dbg!(x);
}