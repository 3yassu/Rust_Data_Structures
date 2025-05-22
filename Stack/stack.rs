struct Node<T>{
    entry: T,
    next: NodeChild<T>,
}
struct NodeChild<T>(Option<Box<Node<T>>>);
impl<T> NodeChild<T>{
    fn new() -> Self {Self(None)}
    fn new_node(node: Node<T>) -> Self {Self(Some(Box::new(node)))}
}
impl<T> Node<T>{
    pub fn new(entry: T) -> Self{
        Self{entry, next: NodeChild::new()}
    }
    pub fn new_next(entry: T, next: NodeChild<T>) -> Self{
        Self{entry, next}
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
        let new_top = NodeChild::new_node(Node::new_next(entry, NodeChild(self.top.0.take())));
        self.top = new_top;
    }
    pub fn pop(&mut self) -> Option<T>{
        self.top.0.take().map(|top| {self.top = top.next; top.entry})
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

pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
pub struct Iter<'a, T>(Option<&'a Node<T>>);
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {self.0 = node.next.0.as_deref(); &node.entry})
    }
}
pub struct IterMut<'b, T>(Option<&'b mut Node<T>>);
impl<'b, T> Iterator for IterMut<'b, T> {
    type Item = &'b mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {self.0 = node.next.0.as_deref_mut(); &mut node.entry})
    }
}
impl<T> Stack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(self.top.0.as_deref())
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.top.0.as_deref_mut())
    }
}
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for item in iter {stack.push(item);}
        let mut return_stack = Stack::new();
        for item in stack {return_stack.push(item)}
        return_stack
    }
}

