use std::cmp::Ordering;
struct BinaryNode<T: Ord>{
    entry: T,
    left: BinaryChild<T>,
    right: BinaryChild<T>
}
struct BinaryChild<T: Ord>(Option<Box<BinaryNode<T>>>);
impl<T:Ord> BinaryChild<T>{fn new() -> Self {Self(None)}}
    impl<T: Ord> BinaryNode<T>{
        pub fn new(entry: T) -> Self{
            Self{entry, left: BinaryChild::new(), right: BinaryChild::new()}
        }
    }

struct BinST<T: Ord>{
    root: BinaryChild<T>
}
impl<T: Ord + std::fmt::Display + Clone> BinST<T>{
    pub fn new() -> Self{
        BinST{root: BinaryChild::new()}
    }
    fn maxLST(current_node: &mut BinaryChild<T>) -> T{
        let Some(current) = &mut current_node.0 else{panic!("What")};
        match &mut current.right.0 {
            None => {
                let returnVal = current_node.0.take();
                returnVal.unwrap().entry
            },
            Some(node) => Self::maxLST(&mut node.right),
        }
    }
    fn maxRST(current_node: &mut BinaryChild<T>) -> T{
        let Some(current) = &mut current_node.0 else{panic!("What")};
        match &mut current.left.0 {
            None => {
                let returnVal = current_node.0.take();
                returnVal.unwrap().entry
            },
            Some(node) => Self::maxRST(&mut node.left),
        }
    }
    fn recInsert(entry: T, current_node: &mut BinaryChild<T>){
        match &mut current_node.0 {
            None => current_node.0 = Some(Box::new(BinaryNode::new(entry))),
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => {Self::recInsert(entry, &mut current.left)},
                Ordering::Equal => {}
                Ordering::Greater => Self::recInsert(entry, &mut current.right),
            },
        }
    }
    fn recSearch(entry: T, current_node: &BinaryChild<T>) -> Result<T, bool>{
        match &current_node.0 {
            None => Err(false),
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => Ok(Self::recSearch(entry, &current.left)?),
                Ordering::Equal => Ok(current.entry.clone()),
                Ordering::Greater => Ok(Self::recSearch(entry, &current.right)?),
            },
        }
    }
    fn recRemove(entry: T, current_node: &mut BinaryChild<T>){
        match &mut current_node.0 {
            None => {}
            Some(current) => match entry.cmp(&current.entry) {
                Ordering::Less => Self::recInsert(entry, &mut current.left),
                Ordering::Equal => {
                    match (&mut current.left.0, &mut current.right.0){
                        (None, None) => {
                            current_node.0 = None;
                        },
                        (Some(_left), None) => {
                            current_node.0.as_mut().unwrap().entry = Self::maxLST(&mut current.left);
                        },
                        (_, Some(_right)) => {
                            current_node.0.as_mut().unwrap().entry = Self::maxRST(&mut current.right);
                        }
                    }
                }
                Ordering::Greater => Self::recInsert(entry, &mut current.right),
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
    pub fn search(&self, entry: T) -> Result<T, bool>{
        if let Some(_root) = &self.root.0{
            let root_mut = &self.root;
            Ok(Self::recSearch(entry, root_mut)?)
        }else{
            Err(false)
        }
        
    }
    pub fn remove(&mut self, entry: T){
        let root_mut = &mut self.root;
        Self::recRemove(entry, root_mut);
    }
}