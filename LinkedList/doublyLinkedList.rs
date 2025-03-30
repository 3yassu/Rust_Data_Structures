use std::cmp::Ordering;
use std::NonNull;
struct Node<T: Ord>{
    entry: T,
    next: NodeChild<T>
}
struct NodeChild<T>(Option<NonNull<Node<T>>>);