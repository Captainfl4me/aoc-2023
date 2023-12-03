pub enum StringSearchTree {
    Root {
        neighboors: Vec<Box<StringSearchTree>>,
    },
    Leaf {
        value: char,
        neighboors: Vec<Box<StringSearchTree>>,
    },
    Empty {
        return_value: u32,
    },
}
pub struct SearchMatchResult {
    pub value: u32,
    pub length: u32,
}

impl StringSearchTree {
    pub fn new() -> StringSearchTree {
        StringSearchTree::Root { neighboors: Vec::new() }
    }

    pub fn insert_leaf(&mut self, value: char) {
        match self {
            StringSearchTree::Root { neighboors } => {
                if neighboors.len() == 1 {
                    if matches!(neighboors[0].as_ref(), StringSearchTree::Empty { .. }) {
                        panic!("Cannot insert leaf at same level than empty node");
                    }
                }
                neighboors.push(Box::new(StringSearchTree::Leaf {
                    value,
                    neighboors: Vec::new(),
                }));
            }
            StringSearchTree::Leaf { neighboors, .. } => {
                if neighboors.len() == 1 {
                    if matches!(neighboors[0].as_ref(), StringSearchTree::Empty { .. }) {
                        panic!("Cannot insert leaf at same level than empty node");
                    }
                }
                neighboors.push(Box::new(StringSearchTree::Leaf {
                    value,
                    neighboors: Vec::new(),
                }));
            }
            StringSearchTree::Empty { .. } => {
                panic!("Cannot insert leaf into empty");
            }
        }
    }

    pub fn insert_empty(&mut self, return_value: u32) {
        match self {
            StringSearchTree::Root { neighboors } => {
                if neighboors.len() > 0 {
                    panic!("Cannot insert empty into root with neighboors");
                }
                neighboors.push(Box::new(StringSearchTree::Empty {
                    return_value,
                }));
            }
            StringSearchTree::Leaf { neighboors, .. } => {
                if neighboors.len() > 0 {
                    panic!("Cannot insert empty into leaf with neighboors");
                }
                neighboors.push(Box::new(StringSearchTree::Empty {
                    return_value,
                }));
            }
            StringSearchTree::Empty { .. } => {
                panic!("Cannot insert empty into empty");
            }
        }
    }

    pub fn find_neighbour(&self, value: char) -> Option<&StringSearchTree> {
        match self {
            StringSearchTree::Root { neighboors } => {
                for neighboor in neighboors.iter() {
                    if let StringSearchTree::Leaf { value: v, .. } = neighboor.as_ref() {
                        if *v == value {
                            return Some(neighboor.as_ref());
                        }
                    }
                }
                None
            }
            StringSearchTree::Leaf { neighboors, .. } => {
                for neighboor in neighboors.iter() {
                    if let StringSearchTree::Leaf { value: v, .. } = neighboor.as_ref() {
                        if *v == value {
                            return Some(neighboor.as_ref());
                        }
                    }
                }
                None
            }
            StringSearchTree::Empty { .. } => None,
        }
    }
    
    pub fn find_neighbour_mut(&mut self, value: char) -> Option<&mut StringSearchTree> {
        match self {
            StringSearchTree::Root { neighboors } => {
                for neighboor in neighboors.iter_mut() {
                    if let StringSearchTree::Leaf { value: v, .. } = neighboor.as_ref() {
                        if *v == value {
                            return Some(neighboor.as_mut());
                        }
                    }
                }
                None
            }
            StringSearchTree::Leaf { neighboors, .. } => {
                for neighboor in neighboors.iter_mut() {
                    if let StringSearchTree::Leaf { value: v, .. } = neighboor.as_ref() {
                        if *v == value {
                            return Some(neighboor.as_mut());
                        }
                    }
                }
                None
            }
            StringSearchTree::Empty { .. } => None,
        }
    }

    pub fn find_empty(&self) -> Option<&StringSearchTree> {
        match self {
            StringSearchTree::Root { neighboors } => {
                for neighboor in neighboors.iter() {
                    if let StringSearchTree::Empty { .. } = neighboor.as_ref() {
                        return Some(neighboor.as_ref());
                    }
                }
                None
            }
            StringSearchTree::Leaf { neighboors, .. } => {
                for neighboor in neighboors.iter() {
                    if let StringSearchTree::Empty { .. } = neighboor.as_ref() {
                        return Some(neighboor.as_ref());
                    }
                }
                None
            }
            StringSearchTree::Empty { .. } => None,
        }
    }

    pub fn insert_string_on_root(&mut self, string: &str, value: u32) {
        if !matches!(self, StringSearchTree::Root { .. }) {
            panic!("Cannot insert string on non-root node");
        }

        let mut current_node = self;
        for char in string.chars() {
            let matching_neighboor = current_node.find_neighbour(char);
            if matching_neighboor.is_none() {
                current_node.insert_leaf(char);
            } 
            current_node = current_node.find_neighbour_mut(char).unwrap();
        }
        current_node.insert_empty(value);
    }


    pub fn match_string_from_root(&self, string: &str) -> SearchMatchResult {
        if matches!(self, StringSearchTree::Empty { .. }) {
            panic!("Cannot match string on empty node");
        }
        if matches!(self, StringSearchTree::Leaf { .. }) {
            panic!("Cannot match string on leaf node with this method!");
        }
        let mut result = SearchMatchResult { value: 0, length: 0 };
        let mut current_node = self;
        for char in string.chars() {
            let matching_neighboor = current_node.find_neighbour(char);
            if matching_neighboor.is_none() {
                break;
            }
            current_node = matching_neighboor.unwrap();
            result.length += 1;

            let return_node = current_node.find_empty();
            if return_node.is_some() {
                if let StringSearchTree::Empty { return_value } = return_node.unwrap() {
                    result.value = *return_value;
                }
            }
        }
        result
    }
}
