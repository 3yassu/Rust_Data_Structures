use std::cmp::Ordering;
#[derive(Debug)]
struct BinaryNode<T: Ord>{
	entry: T,
	left: BinaryChild<T>,
	right: BinaryChild<T>
}
type BinaryChild<T> = Option<Box<BinaryNode<T>>>;
impl<T: Ord> BinaryNode<T>{
	pub fn new(entry: T) -> Self{
		Self{entry, left: None, right: None}
	}
}
#[derive(Debug)]
struct BinST<T: Ord>{
	root: BinaryChild<T>
}
impl<T: Ord> BinST<T>{
	pub fn new() -> Self{
		Self{root: None}
	}
	fn max_lst(current_node: &mut BinaryChild<T>) -> T{
		match &mut current_node.as_mut().unwrap().right {
			None => {
				let ret: T; 
				(*current_node, ret) = Self::rem(current_node.take().unwrap());
				ret
			},
			_ => Self::max_lst(&mut current_node.as_mut().unwrap().right),
		}
	}
	fn min_rst(current_node: &mut BinaryChild<T>) -> T{
		match &mut current_node.as_mut().unwrap().left {
			None => {
				let ret: T; 
				(*current_node, ret) = Self::rem(current_node.take().unwrap());
				ret
			},
			_ => Self::min_rst(&mut current_node.as_mut().unwrap().left),
		}
	}
	fn rec_insert(entry: T, current_node: &mut BinaryChild<T>){
		match current_node {
			None => *current_node = Some(Box::new(BinaryNode::new(entry))),
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::rec_insert(entry, &mut current.left),
				Ordering::Equal => {},
				Ordering::Greater => Self::rec_insert(entry, &mut current.right),
			},
		}
	}
	fn rec_search(entry: T, current_node: &BinaryChild<T>) -> Option<&T>{
		match current_node{
			None => None,
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::rec_search(entry, &current.left),
				Ordering::Equal => Some(&current.entry),
				Ordering::Greater => Self::rec_search(entry, &current.right),
			}
		}
	}
	fn rem(mut current: Box<BinaryNode<T>>) -> (BinaryChild<T>, T){
			let mut ret_cur: BinaryChild<T> = None;
			let ret = current.entry;
			match (&mut current.left, &mut current.right){
				(Some(_left), None) => {
					current.entry = Self::max_lst(&mut current.left);
					ret_cur = Some(current);
				},
				(_, Some(_right)) => {
					current.entry = Self::min_rst(&mut current.right);
					ret_cur = Some(current);
				}
				_ => (),
			};
			(ret_cur, ret)
	}
	fn rec_remove(entry: T, current_node: &mut BinaryChild<T>) -> Option<T>{
		match current_node {
			None => None,
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::rec_remove(entry, &mut current.left),
				Ordering::Equal => {
					let ret: T; 
					(*current_node, ret) = Self::rem(current_node.take().unwrap()); 
					Some(ret)
				},
				Ordering::Greater => Self::rec_remove(entry, &mut current.right),
			},
		}
	}
	pub fn insert(&mut self, entry: T){
		match &mut self.root{
			None => self.root = Some(Box::new(BinaryNode::new(entry))),
			_ => Self::rec_insert(entry, &mut self.root)
		}
	}
	pub fn search(&self, entry: T) -> Option<&T>{
		self.root.as_ref().and_then(|_| Self::rec_search(entry, &self.root))
	}
	pub fn remove(&mut self, entry: T)->Option<T>{
		Self::rec_remove(entry, &mut self.root)
	}
	pub fn rec_drop(current_node: &mut BinaryChild<T>){
		if let Some(current) = current_node{
			Self::rec_drop(&mut current.left);
			Self::rec_drop(&mut current.right);
			Self::rem(current_node.take().unwrap());
		}
	}
}
impl<T: Ord> Drop for BinST<T> {
    fn drop(&mut self){
        Self::rec_drop(&mut self.root);
    }
}
mod test{
	use super::BinST;
	#[test]
	fn general() {
		let mut x: BinST<i32> = BinST::new();
		x.insert(100); x.insert(50); x.insert(150);
		x.insert(75);x.insert(125);x.insert(76);x.insert(124);
		x.insert(70);x.insert(130);x.insert(73);x.insert(127);
		print!("{:?}", x.search(74));
		x.remove(100);
		dbg!(x);
	}
}
