pub mod huffman {
    use std::{rc::Rc, cell::RefCell, collections::BTreeMap};


    struct Node {
        frequency: f64,
        letter: Option<char>,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
    }

    pub struct Encoder {
        root: Option<Node>,
    }

    impl Encoder {
        pub fn new(table: &[(char, f64)]) -> Encoder {
            let mut new_table = BTreeMap::new();
            for val in table {
                let (c, f) = val;
                new_table.insert(*f, *c);
            }
            let new_encoder = Encoder {
                root: None,
            };
            new_encoder
        }

        fn get_root(table: BTreeMap<char, f64>) -> Node {
            let root = Node {
                frequency: 0.,
                letter: None,
                left: None,
                right: None,
            };

            while table.len() > 1 {
                let (smallest, next_smallest) = table.
            }

            Node {
                frequency: 0.,
                letter: None,
                left: None,
                right: None,
            }
        }
    }
}