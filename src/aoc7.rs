use std::{collections::HashMap, cmp::Ordering};

// P1
const CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
// P2
const CARDS_P2: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

pub struct AOC7 {
    winnings: u32,
    count: u32
}


impl AOC7 {

    pub fn new() -> Self {
        Self {
            winnings: 0,
            count: 0,
        }
    }

    fn create_node(&self, line: &str) -> Node {
        let mut iterator = line.split_whitespace();
        let cards = iterator.next().unwrap();
        let bid: u32 = iterator.next().unwrap().parse().unwrap();
        let mut map: HashMap<char, u32> = HashMap::new();
        for character in cards.chars() {
            map.insert(character, *map.get(&character).unwrap_or(&0) + 1);
        }
        let mut values: Vec<u32> =  map.values().map(|val| *val).collect();
        values.sort_unstable();
        let card_type = match values[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2 ,2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1
        };
        Node {
            is_p2: false,
            card_type,
            cards: cards.to_owned(),
            bid,
            left: None,
            right: None
        }
    }


    fn add_to_tree(parent: &mut Node, child: Node) {
        if child > *parent {
            if parent.right.is_some() {
                AOC7::add_to_tree(&mut parent.right.as_mut().unwrap(), child);
            }
            else {
                parent.right = Some(Box::new(child));
            }
        }
        else {
            if parent.left.is_some() {
                AOC7::add_to_tree(&mut parent.left.as_mut().unwrap(),child);
            }
            else {
                parent.left = Some(Box::new(child));
            }
        }
    }

    fn get_winnings(&mut self, node: Option<Box<Node>>) {
        match node {
            None => (),
            Some(node) => {
                self.get_winnings(node.left);
                self.count += 1;
                self.winnings += self.count * node.bid;
                self.get_winnings(node.right);
            }
        }
    }

    pub fn solve_p1(&mut self, lines: Vec<String>) -> u32 {
        let mut root: Option<Box<Node>> = None;
        for line in lines {
            let node = self.create_node(&line);
            if root.is_none() {
                root = Some(Box::new(node));
            }
            else {
                AOC7::add_to_tree(root.as_mut().unwrap(), node);
            }
        }
        self.get_winnings(root);
        self.winnings
    }


    // P2


    fn create_node_p2(&self, line: &str) -> Node {
        let mut iterator = line.split_whitespace();
        let cards = iterator.next().unwrap();
        let bid: u32 = iterator.next().unwrap().parse().unwrap();
        let mut map: HashMap<char, u32> = HashMap::new();
        for character in cards.chars() {
            map.insert(character, *map.get(&character).unwrap_or(&0) + 1);
        }
        // P2
        if map.get(&'J').is_some() {
            let joker_num = *map.get(&'J').unwrap();
            let mut max: char = ' ';
            let mut count: u32 = 0;
            for key in map.keys() {
                if key == &'J' { 
                    continue;
                }
                let current = map.get(&key).unwrap();
                if current > &count {
                    count = *current;
                    max = *key;
                }
            }
            // Could be all Js
            if max != ' ' {
                map.remove_entry(&'J').unwrap();
                map.insert(max, joker_num + count);
            }
        }

        let mut values: Vec<u32> =  map.values().map(|val| *val).collect();
        values.sort_unstable();
        let card_type = match values[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2 ,2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1
        };
        Node {
            is_p2: true,
            card_type,
            cards: cards.to_owned(),
            bid,
            left: None,
            right: None
        }
    }



    pub fn solve_p2(&mut self, lines: Vec<String>) -> u32 {
        let mut root: Option<Box<Node>> = None;
        for line in lines {
            let node = self.create_node_p2(&line);
            if root.is_none() {
                root = Some(Box::new(node));
            }
            else {
                AOC7::add_to_tree(root.as_mut().unwrap(), node);
            }
        }
        self.get_winnings(root);
        self.winnings
    }
}



struct Node {
    is_p2: bool,
    card_type: u8,
    cards: String,
    bid: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.card_type > other.card_type {
            return Some(Ordering::Greater);
        }
        else if self.card_type < other.card_type {
            return Some(Ordering::Less);
        }
        let iter = self.cards.chars().zip(other.cards.chars());
        for (s,o) in iter {
            let mut s_val = 0;
            let mut o_val = 0;
            let iter = if self.is_p2 { CARDS_P2.iter().enumerate() } else { CARDS.iter().enumerate() };
            for (i, card) in iter {
                if s == *card {
                    s_val = i + 1;
                }
                if o == *card {
                    o_val = i + 1;
                }
                if o_val != 0 && s_val != 0 {
                    break;
                }
            }
            if s_val != o_val {
                if s_val > o_val {
                    return Some(Ordering::Greater);
                }
                else {
                    return Some(Ordering::Less);
                }
            }
        }
        Some(Ordering::Equal)
    }
}
