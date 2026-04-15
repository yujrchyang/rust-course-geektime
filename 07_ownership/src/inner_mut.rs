use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    test_refcell();
    test_dag();
}

#[derive(Debug)]
#[allow(unused)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn test_dag() {
    let mut n1 = Node::new(1);
    let mut n2 = Node::new(2);
    let mut n3 = Node::new(3);
    let n4 = Node::new(4);

    n3.update_downstream(Rc::new(RefCell::new(n4)));
    n1.update_downstream(Rc::new(RefCell::new(n3)));
    n2.update_downstream(n1.get_downstream().unwrap());
    println!("node1: {:?}", n1);
    println!("node2: {:?}", n2);

    let n5 = Node::new(5);
    let n3 = n1.get_downstream().unwrap();
    n3.borrow_mut().downstream = Some(Rc::new(RefCell::new(n5)));
    println!("node1: {:?}", n1);
    println!("node2: {:?}", n2);
}

fn test_refcell() {
    use std::cell::RefCell;

    let x = RefCell::new(1);
    {
        let mut y = x.borrow_mut();
        *y += 1;
    }
    let z = x.borrow();
    println!("now x = {}", z);
}
