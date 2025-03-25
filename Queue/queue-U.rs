use core::ptr::NonNull;
#[derive(Debug)]
struct Node<T>{
    entry: T,
    next: NodeChild<T>,
}
type NodeChild<T> = Option<NonNull<Node<T>>>;
impl<T> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: None}
    }
}
#[derive(Debug)]
struct Queue<T>{
    front: NodeChild<T>,
    back: NodeChild<T>
}
impl<T> Queue<T>{
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
impl<T> Drop for Queue<T>{
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

pub struct IntoIter<T>(Queue<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}
pub struct Iter<'a, T>(Option<&'a Node<T>>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe{self.0.take().map(|node| {self.0 = node.next.map(|new| {new.as_ref()}); &node.entry})}
    }
}
pub struct IterMut<'b, T>(Option<&'b mut Node<T>>);
impl<'b, T> Iterator for IterMut<'b, T> {
    type Item = &'b mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe{self.0.take().map(|mut node| {self.0 = node.next.map(|mut new| {new.as_mut()}); &mut node.entry})}
    }
}
impl<T> Queue<T> {
    pub fn iter(&mut self) -> Iter<'_, T> {
        unsafe{Iter(self.front.map(|new| {new.as_ref()}))}
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe{IterMut(self.front.map(|mut new| {new.as_mut()}))}
    }
}
impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe{Iter(self.front.map(|new| {new.as_ref()}))}
    }
}
impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<T> FromIterator<T> for Queue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut queue = Queue::new();
        for item in iter {queue.enqueue(item);}
        queue
    }
}