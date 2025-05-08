use std::cmp::Ordering;
#[derive(Debug)]
struct BinaryNode<T: Ord>{
	entry: T,
	left: BinaryChild<T>,
	right: BinaryChild<T>
}
#[derive(Debug)]
struct BinaryChild<T: Ord>(Option<Box<BinaryNode<T>>>);
impl<T:Ord> BinaryChild<T>{fn new() -> Self {Self(None)}}
impl<T: Ord> BinaryNode<T>{
	pub fn new(entry: T) -> Self{
		Self{entry, left: BinaryChild::new(), right: BinaryChild::new()}
	}
}
#[derive(Debug)]
struct BinST<T: Ord>{
	root: BinaryChild<T>
}
impl<T: Ord> BinST<T>{
	pub fn new() -> Self{
		Self{root: BinaryChild::new()}
	}
	fn maxLST(current_node: &mut BinaryChild<T>) -> T{
		match &mut current_node.0.as_mut().unwrap().right.0 {
			None => {
                let ret: T; 
			    (*current_node, ret) = Self::rem(current_node.0.take());
			    ret
			},
			Some(node) => Self::maxLST(&mut current_node.0.as_mut().unwrap().right),
		}
	}
	fn minRST(current_node: &mut BinaryChild<T>) -> T{
		match &mut current_node.0.as_mut().unwrap().left.0 {
			None => {
                let ret: T; 
			    (*current_node, ret) = Self::rem(current_node.0.take());
			    ret
			},
			Some(node) => Self::minRST(&mut current_node.0.as_mut().unwrap().left),
		}
	}
	fn recInsert(entry: T, current_node: &mut BinaryChild<T>){
		match &mut current_node.0 {
			None => current_node.0 = Some(Box::new(BinaryNode::new(entry))),
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::recInsert(entry, &mut current.left),
				Ordering::Equal => {},
				Ordering::Greater => Self::recInsert(entry, &mut current.right),
			},
		}
	}
	fn recSearch(entry: T, current_node: &BinaryChild<T>) -> Option<&T>{
		match &current_node.0 {
			None => None,
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::recSearch(entry, &current.left),
				Ordering::Equal => Some(&current.entry),
				Ordering::Greater => Self::recSearch(entry, &current.right),
			},
		}
	}
	fn rem(current_node: Option<Box<BinaryNode<T>>>) -> (BinaryChild<T>, T){
	    if let Some(mut current) = current_node{
	        let mut ret_cur: BinaryChild<T> = BinaryChild::new();
        	let ret = current.entry;
        	match (&mut current.left.0, &mut current.right.0){
        		(Some(_left), None) => {
        			current.entry = Self::maxLST(&mut current.left);
        			ret_cur.0 = Some(current);
        		},
        		(_, Some(_right)) => {
        			current.entry = Self::minRST(&mut current.right);
        			ret_cur.0 = Some(current);
        		}
        		_ => (),
        	};
        	(ret_cur, ret)
    	}else{panic!("rem should be given a non-empty BinaryChild");}
	}
	fn recRemove(entry: T, current_node: &mut BinaryChild<T>) -> Option<T>{
		match &mut current_node.0 {
			None => None,
			Some(current) => match entry.cmp(&current.entry) {
				Ordering::Less => Self::recRemove(entry, &mut current.left),
				Ordering::Equal => {
				    let ret: T; 
				    (*current_node, ret) = Self::rem(current_node.0.take()); 
				    Some(ret)
				},
				Ordering::Greater => Self::recRemove(entry, &mut current.right),
			},
		}
	}
	pub fn insert(&mut self, entry: T){
		if let Some(_root) = &mut self.root.0 {
			let root_mut = &mut self.root;
			Self::recInsert(entry, root_mut);
		}
		else{
			self.root.0 = Some(Box::new(BinaryNode::new(entry)));
		}
	}
	pub fn search(&self, entry: T) -> Option<&T>{
		if let Some(_root) = &self.root.0{
			let root_mut = &self.root;
			Self::recSearch(entry, root_mut)
		}else{
			None
		}		
	}
	pub fn remove(&mut self, entry: T)->Option<T>{
		let root_mut = &mut self.root;
		Self::recRemove(entry, root_mut)
	}
}
