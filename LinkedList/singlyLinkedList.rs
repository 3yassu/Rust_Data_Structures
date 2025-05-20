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
impl<T> SinglyLinkedList<T>{
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
		let node = Box::new(Node::new(entry));
		let mut jumper = &mut self.front;
		while let Some(node) = jumper{
			jumper = &mut node.next;
		}
		*jumper = Some(node);
		self.len += 1;
	}
	pub fn push_index(&mut self, index: usize, entry: T) {
		if index >= self.len {panic!("IndexError: [SinglyLinkedList]self.peek()")}
		let mut node = Box::new(Node::new(entry));
		let mut jumper = self.front.as_mut();
		for _ in 1..index{
			jumper = jumper.unwrap().next.as_mut();
		}
		node.next = jumper.as_mut().unwrap().next.take();
		jumper.unwrap().next = Some(node);
		self.len += 1;
	}
	pub fn pop_front(&mut self) -> Option<T>{
		self.front.take().map(|node|{
			self.front = node.next;
			self.len -= 1;
			node.entry
		})
	}
	pub fn pop_back(&mut self) -> Option<T>{ 
		let mut jumper = &mut self.front;
		for _ in 1..self.len{
			jumper = &mut jumper.as_mut().unwrap().next;
		}
		jumper.take().map(|node|{
			self.len -= 1;
			node.entry
		})
	}
	pub fn peek_front(&self) -> Option<&T> {
		self.front.as_ref().map(|node| {&node.entry})
	}
	pub fn peek_front_mut(&mut self) -> Option<&mut T> {
		self.front.as_mut().map(|node| {&mut node.entry})
	}
	pub fn peek(&mut self, index: usize) -> &T {
		if index >= self.len {panic!("IndexError: [SinglyLinkedList]self.peek()")}
		let mut jumper = self.front.as_mut();
		for _ in 0..index{
			jumper = jumper.unwrap().next.as_mut();
		}
		&jumper.unwrap().entry
	}
	pub fn peek_mut(&mut self, index: usize) -> &mut T {
		if index >= self.len {panic!("IndexError: [SinglyLinkedList]self.peek_mut()")}
		let mut jumper = self.front.as_mut();
		for _ in 0..index{
			jumper = jumper.unwrap().next.as_mut();
		}
		&mut jumper.unwrap().entry
	}
	pub fn is_empty(&self) -> bool {
		self.len == 0
	}
}
mod test{
	use super::SinglyLinkedList;
	#[test]
	fn general() {
		let mut x = SinglyLinkedList::new();
		println!("{:?}", x.pop_back());
		x.push_front(3);
		x.peek_front();
		x.peek_front_mut();
		x.peek_mut(0);
		x.is_empty();
		x.push_front(8);
		x.push_back(7);
		x.push_front(4);
		x.push_index(1, 4);
		dbg!(&x);
		x.pop_front();
		x.pop_back();
		dbg!(&x);
		println!("{}", x.peek(0));
		dbg!(x);
	}
}
