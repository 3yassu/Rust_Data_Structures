use std::cmp::Ordering;
use std::NonNull;
use core::marker::PhantomData;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>
}
type NodeChild<T> = Option<NonNull<Node<T>>>;
struct LinkedList<T>{
    front: NodeChild<T>,
    back: NodeChild<T>,
    _boo: PhantomData<T>
}