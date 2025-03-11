use std::cmp::Ordering;
struct BinaryNode<T: Ord>{
    entry: T,
    left: Option<Box<BinaryNode<T>>>,
    right: Option<Box<BinaryNode<T>>>
}
    impl<T: Ord> BinaryNode<T>{
        fn new(entry: T) -> Self{
            Self{entry, left: None, right: None}
        }
    }
    impl<T: Ord> Ord for BinaryNode<T>{
        fn cmp(&self, other: &Self) -> Ordering {
            self.entry.cmp(&other.entry)
        }
    }
    impl<T: Ord> PartialOrd for BinaryNode<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl<T: Ord> Eq for BinaryNode<T> {}
impl<T: Ord> PartialEq for BinaryNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.entry == other.entry
    }
}

struct BinST<T: Ord>{
    root: Option<BinaryNode<T>>
}
impl<T: Ord + std::fmt::Display + Clone> BinST<T>{
    pub fn new() -> Self{
        BinST{root: None}
    }
    fn maxLST(current_node: &mut BinaryNode<T>) -> T{
        if current_node.right.as_ref().unwrap().right != None{
            Self::maxLST(current_node.right.as_mut().unwrap())
        }else{
            let temp = current_node.right.take();
            temp.unwrap().entry
        }
    }
    fn maxRST(current_node: &mut BinaryNode<T>) -> T{
        if current_node.left.as_ref().unwrap().right != None{
            Self::maxLST(current_node.left.as_mut().unwrap())
        }else{
            let temp = current_node.left.take();
            temp.unwrap().entry
        }
    }
    fn recInsert(entry: T, current_node: &mut BinaryNode<T>){
        if entry > current_node.entry{
            if current_node.right == None {
                current_node.right = Some(Box::new(BinaryNode::new(entry)));
            }else{
                Self::recInsert(entry, current_node.right.as_mut().unwrap());
            }
        }else if entry < current_node.entry{
            if current_node.left == None {
                current_node.left = Some(Box::new(BinaryNode::new(entry)));
            }else{
                Self::recInsert(entry, current_node.left.as_mut().unwrap());
            }
        }
    }
    fn recSearch(entry: T, current_node: &BinaryNode<T>) -> Result<T, bool>{
        if entry > current_node.entry{
            if current_node.right == None {
                Err(false)
            }else{
                Ok(Self::recSearch(entry, current_node.right.as_ref().unwrap())?)
            }
        }else if entry < current_node.entry{
            if current_node.left == None {
                Err(false)
            }else{
                Ok(Self::recSearch(entry, current_node.left.as_ref().unwrap())?)
            }
        }else{
            Ok(current_node.entry.clone())
        }   
    }
    fn recRemove(entry: T, current_node: &mut BinaryNode<T>) -> Option<Box<BinaryNode<T>>>{
        if entry > current_node.entry{
            if current_node.right == None {
                None
            }else{
                if current_node.right.as_ref().unwrap().entry == entry {
                    let rightCheck: bool = current_node.right.as_ref().unwrap().right == None;
                    let leftCheck: bool = current_node.right.as_ref().unwrap().left == None;
                    if leftCheck && rightCheck{
                        let temp = current_node.right.take();
                        temp
                    }else{
                        Self::recRemove(entry, current_node.right.as_mut().unwrap());
                    }
                }else{
                    Self::recRemove(entry, current_node.right.as_mut().unwrap());
                }
            }
        }else if entry < current_node.entry{
            if current_node.left == None {
                None
            }else{
                if current_node.left.as_ref().unwrap().entry == entry {
                    let rightCheck: bool = current_node.left.as_ref().unwrap().right == None;
                    let leftCheck: bool = current_node.left.as_ref().unwrap().left == None;
                    if leftCheck && rightCheck{
                        let temp = current_node.left.take();
                        temp
                    }else{
                        Self::recRemove(entry, current_node.right.as_mut().unwrap());
                    }
                }else{
                    Self::recRemove(entry, current_node.right.as_mut().unwrap());
                }
            }
        }else{
            let rightCheck: bool = current_node.right == None;
            if rightCheck {
                let rightCheck2: bool = current_node.left.as_ref().unwrap().right == None;
                let leftCheck2: bool = current_node.left.as_ref().unwrap().left == None;
                if leftCheck2 && rightCheck2 {
                    let temp1 = current_node.left.take();
                    let temp2 = current_node.left.take();
                    temp.unwrap().entry 
                }
            }
        }         
    }
    pub fn insert(&mut self, entry: T){
        if self.root == None{self.root = Some(BinaryNode::new(entry))}
        else{
            let root_mut = self.root.as_mut().unwrap();
            Self::recInsert(entry, root_mut);
        }
    }
    pub fn search(&self, entry: T) -> Result<T, bool>{
        if self.root == None{
            Err(false)
        }else{
            let root_mut = self.root.as_ref().unwrap();
            Ok(Self::recSearch(entry, root_mut)?)
        }
        
    }
    pub fn remove(&mut self, entry: T){
        let root_mut = self.root.as_mut().unwrap();
        Self::recRemove(entry, root_mut);
    }
}