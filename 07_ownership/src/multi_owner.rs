use std::rc::Rc;

fn main() {
    test_rc();
    test_dag();
}

#[derive(Debug)]
#[allow(unused)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn test_dag() {
    let mut n1 = Node::new(1);
    let mut n2 = Node::new(2);
    let mut n3 = Node::new(3);
    let n4 = Node::new(4);

    n3.update_downstream(Rc::new(n4));
    n1.update_downstream(Rc::new(n3));
    n2.update_downstream(n1.get_downstream().unwrap());

    println!("node1: {:?}", n1);
    println!("node2: {:?}", n2);
}

#[allow(unused)]
fn test_rc() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    let d = b.clone();
}
