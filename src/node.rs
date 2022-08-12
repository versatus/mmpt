#![allow(unused)]
/// Fork, Branch and Leaf nodes.
/// 
/// Fork Nodes contain a shared nibble.
/// 
/// Branch nodes have 256 values representing each possible nibble. Branch nodes can be implemented as an inner trie
/// or as a hashmap with nibble -> node and a hash (the hash of all Fork and branch nodes beneath them.)
/// 
/// Leaf nodes contain the remainder of the address a value and a hash
pub type RootHash = [u8; 32];
pub type Nibble = u8;

#[derive(Clone, Debug)]
pub enum Node<P> {
    Data {
        data: Leaf<P>,
        hash: RootHash,
    },
    Fork {
        fork: Fork<P>,
        hash: RootHash,
    },
    None,
}

#[derive(Clone, Debug)]
pub struct Root<P> {
    next: Branch<P>,
    hash: RootHash
}

#[derive(Clone, Debug)]
pub struct Branch<P> {
    layer: Layer,
    nibbles: Vec<Node<P>>,
    hash: RootHash,
}

#[derive(Clone, Debug)]
pub struct Fork<P> {
    nibble: Nibble,
    next: Box<Branch<P>>,
}

#[derive(Clone, Debug)]
pub struct Leaf<P> {
    pub nibble: Nibble,
    remainder: Vec<u8>,
    payload: P,
}

#[derive(Clone, Debug)]
pub enum Layer {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
    ThirtyOne,
    OutOfRange,

}

impl<P: Clone> Root<P> {
    pub fn new() -> Root<P> {
        let next = Branch::new(Layer::Zero);
        let hash = [0u8; 32];

        Root { next, hash }
    }

    pub fn get_next(&self) -> Branch<P> {
        self.next.clone()
    }
}

impl<P: Clone> Branch<P> {
    pub fn new(layer: Layer) -> Branch<P> {
        let mut nibbles: Vec<Node<P>> = Vec::with_capacity(256);
        nibbles.extend(vec![Node::None; 256]);
        Branch { layer, nibbles, hash: [0u8; 32] }
    }

    pub fn insert(&mut self, leaf: Leaf<P>) {
        let index = leaf.nibble as usize;
        let node = &self.nibbles[index].clone();
        match node.clone() {
            Node::Fork { mut fork, hash } => {
                fork.next.insert(leaf);
                self.nibbles[index] = Node::Fork { fork, hash };
            }
            Node::Data { data, hash } => {
                let layer: u8 = self.layer.clone().into();
                let fork: Fork<P> = (leaf, data.clone(), layer as usize).into();
                self.nibbles[index] = Node::Fork { fork, hash: [0u8; 32]};
            } 
            Node::None => {
                self.nibbles[index] = Node::Data { data: leaf, hash: [0u8; 32] }
            }
        }
    }

    pub fn get(&self, nibble: &Nibble) -> Node<P> {
        self.nibbles[*nibble as usize].clone()
    }
}

impl<P: Clone> Fork<P> {
    pub fn new(nibble: Nibble, layer: Layer) -> Fork<P> {
        Fork { nibble, next: Box::new(Branch::new(layer)) }
    }

    pub fn get_next(&self) -> Branch<P> {
        *self.next.clone()
    }
}

impl<P: Clone> Leaf<P> {
    pub fn new(address: [u8; 32], payload: P) -> Leaf<P> {
        let nibble = address[0];
        let remainder = address[1..].to_vec();
        let payload = payload;
        
        Leaf { nibble, remainder, payload }
    }   
}

impl<P: Clone> From<(Leaf<P>, Leaf<P>, usize)> for Fork<P> {
    
    fn from(i: (Leaf<P>, Leaf<P>, usize)) -> Fork<P> {
        let nibble = i.0.nibble;
        let leaf_1 = Leaf { nibble: i.0.remainder[0], remainder: i.0.remainder[1..].to_vec(), payload: i.0.payload };
        let leaf_2 = Leaf { nibble: i.1.remainder[0], remainder: i.1.remainder[1..].to_vec(), payload: i.1.payload };
        let layer = i.2 + 1; 
        let mut next: Box<Branch<P>> = Box::new(Branch::new(layer.into()));
        next.insert(leaf_1);
        next.insert(leaf_2);
        let hash = [0u8; 32];

        Fork { nibble, next}
    }

}

impl<P: Clone> Default for Root<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Layer> for u8 {
    fn from(i: Layer) -> u8 {
        match i {
            Layer::Zero => 0,
            Layer::One => 1,
            Layer::Two => 2,
            Layer::Three => 3,
            Layer::Four => 4,
            Layer::Five => 5,
            Layer::Six => 6,
            Layer::Seven => 7,
            Layer::Eight => 8,
            Layer::Nine => 9,
            Layer::Ten => 10,
            Layer::Eleven => 11,
            Layer::Twelve => 12,
            Layer::Thirteen => 13,
            Layer::Fourteen => 14,
            Layer::Fifteen => 15,
            Layer::Sixteen => 16,
            Layer::Seventeen => 17,
            Layer::Eighteen => 18,
            Layer::Nineteen => 19,
            Layer::Twenty => 20,
            Layer::TwentyOne => 21,
            Layer::TwentyTwo => 22,
            Layer::TwentyThree => 23,
            Layer::TwentyFour => 24,
            Layer::TwentyFive => 25,
            Layer::TwentySix => 26,
            Layer::TwentySeven => 27,
            Layer::TwentyEight => 28,
            Layer::TwentyNine => 29,
            Layer::Thirty => 30,
            Layer::ThirtyOne => 31,
            Layer::OutOfRange => 255,
        }
    }
}

impl From<u8> for Layer {
    fn from(i: u8) -> Layer {
        match i {
            0 => Layer::Zero,
            1 => Layer::One,
            2 => Layer::Two,
            3 => Layer::Three,
            4 => Layer::Four,
            5 => Layer::Five,
            6 => Layer::Six,
            7 => Layer::Seven,
            8 => Layer::Eight,
            9 => Layer::Nine,
            10 => Layer::Ten,
            11 => Layer::Eleven,
            12 => Layer::Twelve,
            13 => Layer::Thirteen,
            14 => Layer::Fourteen,
            15 => Layer::Fifteen,
            16 => Layer::Sixteen,
            17 => Layer::Seventeen,
            18 => Layer::Eighteen,
            19 => Layer::Nineteen,
            20 => Layer::Twenty,
            21 => Layer::TwentyOne,
            22 => Layer::TwentyTwo,
            23 => Layer::TwentyThree,
            24 => Layer::TwentyFour,
            25 => Layer::TwentyFive,
            26 => Layer::TwentySix,
            27 => Layer::TwentySeven,
            28 => Layer::TwentyEight,
            29 => Layer::TwentyNine,
            30 => Layer::Thirty,
            31 => Layer::ThirtyOne,
            _ => Layer::OutOfRange,
        }
    }
}

impl From<usize> for Layer {
    fn from(i: usize) -> Layer {
        match i {
            0 => Layer::Zero,
            1 => Layer::One,
            2 => Layer::Two,
            3 => Layer::Three,
            4 => Layer::Four,
            5 => Layer::Five,
            6 => Layer::Six,
            7 => Layer::Seven,
            8 => Layer::Eight,
            9 => Layer::Nine,
            10 => Layer::Ten,
            11 => Layer::Eleven,
            12 => Layer::Twelve,
            13 => Layer::Thirteen,
            14 => Layer::Fourteen,
            15 => Layer::Fifteen,
            16 => Layer::Sixteen,
            17 => Layer::Seventeen,
            18 => Layer::Eighteen,
            19 => Layer::Nineteen,
            20 => Layer::Twenty,
            21 => Layer::TwentyOne,
            22 => Layer::TwentyTwo,
            23 => Layer::TwentyThree,
            24 => Layer::TwentyFour,
            25 => Layer::TwentyFive,
            26 => Layer::TwentySix,
            27 => Layer::TwentySeven,
            28 => Layer::TwentyEight,
            29 => Layer::TwentyNine,
            30 => Layer::Thirty,
            31 => Layer::ThirtyOne,
            _ => Layer::OutOfRange,
        }
    }
}