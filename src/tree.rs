use std::{cell::RefCell, collections::HashMap, rc::Rc};

trait BaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> u8;
}

#[derive(Debug)]
struct LeafNode {
    element: char,
    weight: u8,
}

struct InternalNode {
    left: Rc<dyn BaseNode>,
    right: Rc<dyn BaseNode>,
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

impl InternalNode {
    fn new(
        right: Rc<impl BaseNode + 'static>,
        left: Rc<impl BaseNode + 'static>,
        weight: u8,
    ) -> Self {
        Self {
            weight,
            right,
            left,
        }
    }

    fn right(&self) -> Rc<dyn BaseNode> {
        self.right.clone()
    }

    fn left(&self) -> Rc<dyn BaseNode> {
        self.left.clone()
    }
}

impl BaseNode for InternalNode {
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
    data: Vec<Rc<InternalNode>>,
}

impl Tree {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn add(&mut self, node: InternalNode) {
        self.data.push(Rc::new(node));
    }

    fn get_last(&self) -> Rc<InternalNode> {
        self.data.last().unwrap().clone()
    }

    fn print(&self) {
        println!("\n");
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

enum SortedListType {
    Raw((char, u8)),
    Node(Rc<InternalNode>),
}

fn sort_list(list: &mut Vec<SortedListType>) {
    list.sort_by(|a, b| match a {
        SortedListType::Raw((_, v1)) => match b {
            SortedListType::Raw((_, v2)) => {
                return v1.cmp(v2);
            }
            SortedListType::Node(node) => return node.weight().cmp(v1),
        },
        SortedListType::Node(node) => match b {
            SortedListType::Raw((_, v2)) => {
                return node.weight().cmp(v2);
            }
            SortedListType::Node(node2) => return node.weight().cmp(&node2.weight()),
        },
    });
}

fn print_list(list: &Vec<SortedListType>) {
    let _ = list
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            SortedListType::Raw(a) => {
                println!("{} - [list] raw: {:?}", i, a);
                return 1;
            }
            SortedListType::Node(b) => {
                println!("{} - [list] node: {:?}", i, b.weight());
                return 1;
            }
        })
        .last();
}

pub fn buildTree(map: HashMap<char, u8>) {
    let mut sorted_list: Vec<SortedListType> = map
        .iter()
        .map(|(k, v)| SortedListType::Raw((k.clone(), v.clone())))
        .collect();

    sort_list(&mut sorted_list);

    let mut tree = RefCell::new(Tree::new());
    while sorted_list.len() > 1 {
        println!("new iteration");
        print_list(&sorted_list);
        tree.borrow().print();
        // println!("{:?}", sorted_list);
        match sorted_list.remove(0) {
            SortedListType::Raw((left_char, left_weight)) => match sorted_list.remove(0) {
                SortedListType::Raw((right_char, right_weight)) => {
                    let left_node = LeafNode::new(left_char, left_weight);
                    let right_node = LeafNode::new(right_char, right_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    tree.get_mut().add(InternalNode::new(
                        Rc::new(right_node),
                        Rc::new(left_node),
                        sum_weight,
                    ));
                }
                SortedListType::Node(right_node) => {
                    let left_node = LeafNode::new(left_char, left_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    tree.get_mut().add(InternalNode::new(
                        right_node,
                        Rc::new(left_node),
                        sum_weight,
                    ));
                }
            },
            SortedListType::Node(left_node) => match sorted_list.remove(0) {
                SortedListType::Raw((right_char, right_weight)) => {
                    let right_node = LeafNode::new(right_char, right_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    tree.get_mut().add(InternalNode::new(
                        Rc::new(right_node),
                        left_node,
                        sum_weight,
                    ));
                }
                SortedListType::Node(right_node) => {
                    let sum_weight = left_node.weight() + right_node.weight();
                    tree.get_mut()
                        .add(InternalNode::new(right_node, left_node, sum_weight));
                }
            },
        }

        println!("result (last) node: {:?}", tree.borrow().get_last().weight);

        sorted_list.push(SortedListType::Node(tree.borrow().get_last()));
        sort_list(&mut sorted_list);
    }

    tree.borrow().print();
}
