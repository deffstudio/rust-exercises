struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn append(&mut self, data: i32) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node { data, next: None }));
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.append(4);
    list.append(5);
    list.push_front(0);
    list.append(11);
    list.print(); // Output: 1 -> 2 -> 3 -> 4 -> 5 -> None
}
