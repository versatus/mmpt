use crate::node::{Nibble, Node, Root, Leaf, BranchIntoIter};
use crate::layer::Layer;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::cmp::{PartialEq, Eq};

#[derive(Debug)]
pub struct InvalidInsertError;

impl Display for InvalidInsertError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to insert the node into the Trie")
    }
}

impl Error for InvalidInsertError {
    fn description(&self) -> &str {
        "Unable to insert the node into the Trie"
    }
}

#[derive(Clone, Debug)]
pub struct Trie<P> 
where
    P: Clone + Debug + Into<Vec<u8>>
{
    pub root: Box<Root<P>>,
}

#[derive(Clone, Debug)]
pub struct TrieIntoIter<P> 
where
    P: Clone + Debug + Into<Vec<u8>>
{
    curr_branch: BranchIntoIter<P>,
    layer: Layer,
    branches: Vec<BranchIntoIter<P>>,

}

// TODO: Implement IntoIterator and Iterator for "borrowed" & mutably "borrowed" Tries.
// pub struct TrieIterator<P> 
// where
//     P: Clone + Debug + Into<Vec<u8>>
// {
//     branch: Branch<P>,
//     layer: Layer,
//     layer_indices: [u8; 32],
// }

impl<P: Clone + Debug + Into<Vec<u8>>> Trie<P> {
    /// Creates a new blank trie with a Root (which is initialized with
    /// a Branch node)
    /// 
    /// # Example
    /// ```
    /// use mmpt::trie::Trie;
    /// 
    /// let trie: Trie<String> = Trie::default();
    /// assert_eq!(trie.root.get_next().get_layer(), 0u8);
    /// ```
    pub fn new() -> Trie<P> {
        let root: Box<Root<P>> = Box::new(Root::default());
        Trie { root }
    }

    /// Traverses the trie, every time there's a Fork node discovered
    /// `.get_next()` is called on the Fork node to get the next branch
    /// and check the next nibble for an entry.
    /// However if a data node is discovered, it is returned since a Data node
    /// is the "end of the road" so to speak. If a None node is discovered then
    /// the last Fork node discovered is returned, if there was no Fork node discovered
    /// then a None node is returned.
    /// # Example
    /// 
    /// ```
    /// use mmpt::node::{Root, Node};
    /// use mmpt::trie::Trie;
    /// 
    /// let trie: Trie<String> = Trie::default();
    /// let nibbles: [u8; 4] = [0u8, 1u8, 2u8, 3u8];
    /// let res = trie.traverse(&nibbles);
    /// 
    /// assert!(res.0 == 0);
    /// assert!(res.1.is_some());
    /// 
    /// let node_type_correct = {
    ///     if let Node::None = res.2 {
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// };
    /// assert!(node_type_correct);
    /// 
    /// ```
    pub fn traverse(&self, nibbles: &[u8]) -> (usize, Option<Nibble>, Node<P>) {
        let mut branch = self.root.get_next();
        let mut iter = nibbles.iter().enumerate();
        let mut data: Node<P> = Node::None;
        loop {
            match iter.next() {
                Some((index, nibble)) => 
                    match branch.get(nibble) {
                        Node::Fork { fork, hash } => {
                            branch = fork.get_next();
                            data = Node::Fork { fork, hash };
                        }
                        Node::None => return (index, Some(*nibble), data),
                        Node::Data { data, hash } => {
                            return (
                                index,
                                Some(*nibble),
                                Node::Data {
                                    data: data.clone(),
                                    hash: hash.clone(),
                                },
                            )
                        }
                    },
                None => {
                    if let Some(nibble) = nibbles.last() {
                        return (nibbles.len(), Some(*nibble), data);
                    } else {
                        return (nibbles.len(), Some(0), data);
                    }
                }
            }
        }
    }

    /// Adds a node to the `Trie`, recursively traversing through the `Trie`, starting
    /// with the `Branch` underpinning the `Root` of the `Trie` and, if there is a
    /// conflicting `Leaf` node with a shared `Nibble`, then a new `Fork` is inserted.
    /// 
    /// # Example
    /// 
    /// ```
    /// use mmpt::trie::Trie;
    /// use mmpt::node::{Root, Leaf};
    /// 
    /// let mut trie: Trie<String> = Trie::default();
    /// let address = [0u8; 32];
    /// let payload = "Some Data".to_string();
    /// let new_leaf: Leaf<String> = Leaf::new(address, payload);
    /// trie.add(new_leaf);
    /// ```
    pub fn add(&mut self, leaf: Leaf<P>) {
        self.root.get_next_mut().insert(leaf);
    }

    pub fn get(&self, nibble: &u8) -> Node<P> {
        self.root.get(nibble)
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Default for Trie<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> PartialEq for Trie<P> {
    fn eq(&self, other: &Trie<P>) -> bool {
        self.root.eq(&other.root)
    }

    fn ne(&self, other: &Trie<P>) -> bool {
        self.root.ne(&other.root)
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Eq for Trie<P> { }

impl<P: Clone + Debug + Into<Vec<u8>>> IntoIterator for Trie<P> {
    type Item = Node<P>;
    type IntoIter = TrieIntoIter<P>;
    
    fn into_iter(self) -> Self::IntoIter {
        let layer = self.root.get_next().get_layer();
        TrieIntoIter {
            curr_branch: self.root.get_next().into_iter(),
            layer: layer.into(),
            branches: vec![self.root.get_next().into_iter()],
        }
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Iterator for TrieIntoIter<P> {
    type Item = Node<P>;
    fn next(&mut self) -> Option<Node<P>> {
        while let Some(node) = self.curr_branch.next() {
            match node.clone() {
                Node::Data { .. } => { return Some(node) },
                Node::Fork { fork, .. } => {
                    self.branches.push(fork.get_next().into_iter());
                    self.curr_branch = fork.get_next().into_iter();
                    let mut layer: u8 = self.layer.clone().into();
                    layer += 1;
                    self.layer = layer.into();
                }
                Node::None => { self.next(); }

            }
        }
        if self.layer.clone() as u8 == 0u8 {
            return None
        } else {
            let mut layer = self.layer.clone() as u8;
            layer -= 1;
            self.layer = layer.into();
            self.curr_branch = self.branches[self.layer.clone() as usize].clone();
            self.branches.pop();
            self.next()
        }
    }        
}