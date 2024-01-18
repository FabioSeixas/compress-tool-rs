use std::{collections::HashMap, rc::Rc};

pub type CodeTable = HashMap<char, String>;
pub type DecodingTable = HashMap<String, char>;

trait BaseNode {
    fn is_leaf(&self) -> bool;
    fn weight(&self) -> u32;
}

#[derive(Debug)]
struct LeafNode {
    element: char,
    weight: u32,
}

enum NodeType {
    Leaf(LeafNode),
    Internal(Rc<InternalNode>),
}

struct InternalNode {
    left: Rc<NodeType>,
    right: Rc<NodeType>,
    weight: u32,
}

impl LeafNode {
    fn new(elem: char, weight: u32) -> Self {
        Self {
            element: elem,
            weight,
        }
    }
}

impl InternalNode {
    fn new(right: Rc<NodeType>, left: Rc<NodeType>, weight: u32) -> Self {
        Self {
            weight,
            right,
            left,
        }
    }

    fn right(&self) -> Rc<NodeType> {
        self.right.clone()
    }

    fn left(&self) -> Rc<NodeType> {
        self.left.clone()
    }
}

impl BaseNode for InternalNode {
    fn is_leaf(&self) -> bool {
        false
    }
    fn weight(&self) -> u32 {
        self.weight
    }
}

impl BaseNode for LeafNode {
    fn is_leaf(&self) -> bool {
        true
    }
    fn weight(&self) -> u32 {
        self.weight
    }
}

enum SortedListType {
    Raw((char, u32)),
    Node(Rc<InternalNode>),
}

fn sort_list(list: &mut Vec<SortedListType>) {
    list.sort_by(|a, b| match a {
        SortedListType::Raw((_, v1)) => match b {
            SortedListType::Raw((_, v2)) => {
                return v1.cmp(v2);
            }
            SortedListType::Node(node) => return v1.cmp(&node.weight()),
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

fn go_through_nodes<'r>(node: NodeType, code: String, map: &mut HashMap<char, String>) {
    match node {
        NodeType::Internal(node) => {
            match &*node.left() {
                NodeType::Internal(node) => {
                    let mut new_code = String::from(code.clone());
                    new_code.push_str("0");
                    go_through_nodes(NodeType::Internal(node.clone()), new_code, map);
                }
                NodeType::Leaf(node) => {
                    let mut new_code = String::from(code.clone());
                    new_code.push_str("0");
                    map.insert(node.element, new_code);
                }
            };
            match &*node.right() {
                NodeType::Internal(node) => {
                    let mut new_code = String::from(code);
                    new_code.push_str("1");
                    return go_through_nodes(NodeType::Internal(node.clone()), new_code, map);
                }
                NodeType::Leaf(node) => {
                    let mut new_code = String::from(code);
                    new_code.push_str("1");
                    map.insert(node.element, new_code);
                }
            }
        }
        NodeType::Leaf(node) => {
            let mut new_code = String::from(code);
            new_code.push_str("1");
            map.insert(node.element, new_code);
        }
    }
}

pub fn build_tree(freq_map: HashMap<char, u32>) -> CodeTable {
    let mut sorted_list: Vec<SortedListType> = freq_map
        .iter()
        .map(|(k, v)| SortedListType::Raw((k.clone(), v.clone())))
        .collect();

    sort_list(&mut sorted_list);

    while sorted_list.len() > 1 {
        // print_list(&sorted_list);

        let new_node = match sorted_list.remove(0) {
            SortedListType::Raw((left_char, left_weight)) => match sorted_list.remove(0) {
                SortedListType::Raw((right_char, right_weight)) => {
                    let left_node = LeafNode::new(left_char, left_weight);
                    let right_node = LeafNode::new(right_char, right_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    InternalNode::new(
                        Rc::new(NodeType::Leaf(right_node)),
                        Rc::new(NodeType::Leaf(left_node)),
                        sum_weight,
                    )
                }
                SortedListType::Node(right_node) => {
                    let left_node = LeafNode::new(left_char, left_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    InternalNode::new(
                        Rc::new(NodeType::Internal(right_node)),
                        Rc::new(NodeType::Leaf(left_node)),
                        sum_weight,
                    )
                }
            },
            SortedListType::Node(left_node) => match sorted_list.remove(0) {
                SortedListType::Raw((right_char, right_weight)) => {
                    let right_node = LeafNode::new(right_char, right_weight);
                    let sum_weight = left_node.weight() + right_node.weight();
                    InternalNode::new(
                        Rc::new(NodeType::Leaf(right_node)),
                        Rc::new(NodeType::Internal(left_node)),
                        sum_weight,
                    )
                }
                SortedListType::Node(right_node) => {
                    let sum_weight = left_node.weight() + right_node.weight();
                    InternalNode::new(
                        Rc::new(NodeType::Internal(right_node)),
                        Rc::new(NodeType::Internal(left_node)),
                        sum_weight,
                    )
                }
            },
        };
        sorted_list.push(SortedListType::Node(Rc::new(new_node)));

        sort_list(&mut sorted_list);
    }
    // print_list(&sorted_list);
    let mut code_table: HashMap<char, String> = HashMap::new();
    if let SortedListType::Node(start_node) = sorted_list.remove(0) {
        go_through_nodes(
            NodeType::Internal(start_node),
            String::from(""),
            &mut code_table,
        );
    } else {
        panic!("something went wrong, 'start_node' is not what is expected")
    }

    code_table
}
