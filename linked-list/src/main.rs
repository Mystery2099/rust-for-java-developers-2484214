type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}
impl<T> Node<T> {
    fn get(&self) -> &T {
        &self.value
    }
}
/// Hint: To avoid ownership issues, create two types: oen for the list and one for nodes within the list

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add an item to the end
    fn push(&mut self, item: T) {
        let new_node = Some(Box::new(Node {
            value: item,
            next: None,
        }));
        if self.head.is_none() == true {
            self.head = new_node;
        } else {
            let mut node = self.head.as_mut().unwrap();
            while node.next.is_some() {
                node = node.next.as_mut().unwrap();
            }
            node.next = new_node;
        }
    }

    // Remove an item from the start
    fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        let mut result: Option<T> = None;
        if head.is_some() {
            let old_head = head.unwrap();
            self.head = old_head.next;
            result = Some(old_head.value);
        }
        return result;
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
