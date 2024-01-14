use std::{cell::RefCell, collections::HashMap};

trait BaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> u8;
}

struct LeafNode {
    element: char,
    weight: u8,
}

struct InternalNode<'r> {
    left: &'r Box<dyn BaseNode>,
    right: Box<dyn BaseNode>,
    weight: u8,
}

impl LeafNode {
    fn new(elem: char, weight: u8) -> Self {
        Self {
            element: elem,
            weight,
        }
    }
    fn value(&self) -> char {
        self.element
    }
}

impl<'r> InternalNode<'r> {
    fn new(right: impl BaseNode + 'static, left: &'r Box<dyn BaseNode>, weight: u8) -> Self {
        Self {
            weight,
            right: Box::new(right),
            left,
        }
    }

    fn right(&self) -> &Box<dyn BaseNode> {
        &self.right
    }

    fn left(&self) -> &Box<dyn BaseNode> {
        &self.left
    }
}

impl<'r> BaseNode for InternalNode<'r> {
    fn is_leaf(&self) -> bool {
        false
    }
    fn weight(&self) -> u8 {
        self.weight
    }
}

impl BaseNode for LeafNode {
    fn is_leaf(&self) -> bool {
        true
    }
    fn weight(&self) -> u8 {
        self.weight
    }
}

struct Tree {
    data: Vec<Box<dyn BaseNode>>,
}

impl Tree {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn add(&mut self, node: impl BaseNode + 'static) {
        self.data.push(Box::new(node));
    }

    fn get_last(&self) -> &Box<dyn BaseNode + 'static> {
        self.data.last().unwrap()
    }

    fn print(&self) {
        for (i, item) in self.data.iter().enumerate() {
            println!("node {}, weight: {}", i, item.weight());
            // println!(
            //     "right: leaf? {}, weight? {}",
            //     item.right.is_leaf(),
            //     item.right.weight()
            // );
            // println!(
            //     "left: leaf? {}, weight? {}",
            //     item.left.is_leaf(),
            //     item.left.weight()
            // );
        }
    }
}

pub fn buildTree(map: HashMap<char, u8>) {
    let mut sorted_list: Vec<(Option<char>, u8)> = map
        .iter()
        .map(|(k, v)| (Some(k.clone()), v.clone()))
        .collect();
    sorted_list.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    println!("{:?}", sorted_list);

    let mut tree = RefCell::new(Tree::new());
    while sorted_list.len() > 1 {
        let left = sorted_list.remove(0);
        let right = sorted_list.remove(1);
        let sum = left.1 + right.1;

        let right_node = LeafNode::new(right.0.unwrap(), right.1);
        match left.0 {
            // the first iteration
            Some(char) => {
                let left_node = LeafNode::new(left.0.unwrap(), left.1);
                let mut_tree = tree.get_mut();
                mut_tree.add(left_node);
                drop(mut_tree);
                tree.get_mut().add(InternalNode::new(right_node, tree.borrow().get_last(), sum));
            }
            // the second ahead
            None => {
                let int_node = InternalNode::new(right_node, tree.borrow().get_last(), sum);
                tree.get_mut().add(int_node);
            }
        };
        //

        sorted_list.push((None, sum));
        sorted_list.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    }

    // tree.borrow().print();
}
