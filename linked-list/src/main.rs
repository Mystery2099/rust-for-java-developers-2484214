#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
/// Hint: To avoid ownership issues, create two types: oen for the list and one for nodes within the list

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add an item to the end
    fn push(&mut self, item: T) {
        if (self.)
    }

    // Remove an item from the start
    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    // TODO: optional
    // Returns the maximum/largest value
    // fn greatest(&self) -> Option<&T>
    // where
    //     T: Ord,
    // {
    //  ..
    // }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(4);
    list.push(5);
    list.push(3);
    list.pop();

    if let Some(node) = &list.head {
        assert_eq!(node.get(), &2);
    }

    // assert_eq!(list.greatest(), Some(&5));
}
