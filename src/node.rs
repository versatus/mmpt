#![allow(unused)]
use crate::hash::{Hasher, Sha256Algorithm};
use crate::layer::Layer;
use core::iter::Iterator;
use std::cmp::{Eq, PartialEq};
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
/// Fork, Branch and Leaf nodes.
///
/// Fork Nodes contain a shared nibble.
///
/// Branch nodes have 256 values representing each possible nibble. Branch nodes can be implemented as an inner trie
/// or as a hashmap with nibble -> node and a hash (the hash of all Fork and branch nodes beneath them.)
///
/// Leaf nodes contain the remainder of the address a value and a hash
use std::hash::Hash;

pub type RootHash = [u8; 32];
pub type Nibble = u8;
pub type Address = [u8; 32];

#[derive(Debug)]
pub struct InvalidBranchInsert;

impl Display for InvalidBranchInsert {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to insert the node into the Branch")
    }
}

impl Error for InvalidBranchInsert {
    fn description(&self) -> &str {
        "Unable to insert the node into the Branch"
    }
}

/// An enum to contain (and insert into Branches) different Node types, i.e. a Fork node or a Data node
/// This makes it simple to store the same `type` within a given `Branch` node in a `mpt::trie::Trie<P>`
/// Also contains some helper functions to quickly determine the type of the node.
/// Within the `Node::Data` variant, the `data` field contains a `Leaf<P>` and within the `Node::Fork`
/// variant, it contains a `Fork<P>`. There is also a `Node::None` variant for instances where the
/// nibble in a branch is unallocated to a node.
///
/// # Example
/// ```
/// use mmpt::node::{Leaf, Node};
///
/// let address: [u8; 32] = [0u8; 32];
/// let payload: String = "Some Data".to_string();
/// let leaf: Leaf<String> = Leaf::new(address, payload);
/// let data_node: Node<String> = Node::Data {
///     data: leaf.clone(),
///     hash: leaf.get_hash(),
/// };
/// ```
///
/// ```
/// use mmpt::layer::Layer;
/// use mmpt::node::{Fork, Node};
///
/// let nibble = 0u8;
/// let layer = Layer::One;
/// let fork: Fork<String> = Fork::new(nibble, layer);
/// let fork_node: Node<String> = Node::Fork {
///     fork: fork.clone(),
///     hash: fork.get_hash(),
/// };
/// ```
#[derive(Clone, Debug)]
pub enum Node<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    Data { data: Leaf<P>, hash: RootHash },
    Fork { fork: Fork<P>, hash: RootHash },
    None,
}

/// The `Root` node struct is the root of a Trie, contains the first branch, initialized
/// with 256 `Node::None` enums, representing each possible `Nibble`. The `Root` node's branch
/// is always `Layer::Zero`.
///
/// # Example
///
/// ```
/// use mmpt::node::Root;
///
/// let root: Root<String> = Root::default();
/// ```
#[derive(Clone, Debug)]
pub struct Root<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    next: Branch<P>,
    hash: RootHash,
}

/// The `Branch` struct is a container for the various nodes in a trie at a given layer.
/// The `Branch` struct maintains a `Layer` for quick determination of which layer within
/// the trie this particular `Branch` sits, and then a `Vec<Node<P>>` which always has
/// 256 `Node` variants, i.e. either a `Node::Data`, `Node::Fork` or `Node::None`. The
/// index positions of each `Node` in the `Branch` `nibbles` field is the `Nibble` that
/// this particular node represents. At a given `Nibble`, a node can either contain a `Node::Data`
/// if there is no shared nibble with any other `Leaf`, or if there is 1 more more `Leaf` sharing
/// a given `Nibble`, at a given `Layer`, then the `Node` at the `Branch` `nibble` will be a
/// `Node::Fork`, under which a new new `Branch` and the relevant leaves will sit.
///
/// # Example
///
/// ```
/// use mmpt::node::Branch;
/// use mmpt::layer::Layer;
///
/// let branch: Branch<String> = Branch::new(Layer::One);
///
/// ```
#[derive(Clone, Debug)]
pub struct Branch<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    layer: Layer,
    nibbles: Vec<Node<P>>,
    hash: RootHash,
}

/// `Fork` nodes are added to a `Trie` when there is a shared `Nibble` between
/// Two `Leaf` node's at the current `Layer` of the previous `Branch` node.
/// `Fork` nodes contain the shared `Nibble` and the `next` `Branch`. The `Branch`
/// in the `Fork` node is `Boxed` to prevent infinite recursion.
///
/// # Example
///
/// ```
/// use mmpt::node::{Fork, Branch, Leaf, Node};
/// use mmpt::layer::Layer;
///
/// let shared_nibble = 125u8;
/// let layer = Layer::Two;
/// let fork: Fork<String> = Fork::new(shared_nibble, layer);
///
/// ```
///
#[derive(Clone, Debug)]
pub struct Fork<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    nibble: Nibble,
    next: Box<Branch<P>>,
}

/// The `Leaf` is the basic data containing node for a `Trie`. The `Leaf` node
/// has a `nibble`, which is the first `Nibble` in it's address that is not shared
/// with any other `Leaf`, a `remainder`, which is the remaining portion of the
/// address, i.e. if we have two `Leaf` nodes, `leaf_1` and `leaf_2`:
/// `leaf_1` has an address of
///
/// `[0, 1, 2, 3, 4, 5, 6, 7, ... ]`
///
/// `leaf2` has an address of
/// `[0, 1, 3, 3, 4, 5, 6, 7, ... ]`
///
/// `leaf_1` and `leaf_2` have shared nibbles of 0 and 1, so both would find themselves
/// in a `Branch` in `Layer::Two`, and `leaf_1` would have `leaf_1.remainder == [2, 3, 4, 5, 6, 7, ... ]` while
/// `leaf_2.remainder == [3, 3, 4, 5, 6, 7, ...]`
///
/// The `payload` field in the Leaf node contains the data this leaf represents. In the context of a blockchain
/// this might be an Account or a Transaction Receipt, or some code, or something else. In our examples thus far
/// That data has simply represented a `String`
///
/// # Example
///
/// ```
/// use mmpt::node::Leaf;
///
/// let address: [u8; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];
/// let payload: String = "Some Data".to_string();
/// let leaf = Leaf::new(address, payload);
///
/// ```
#[derive(Clone, Debug)]
pub struct Leaf<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    pub nibble: Nibble,
    address: Address,
    remainder: Vec<u8>,
    payload: P,
}

/// A type that implements Iterator for a Branch Node
/// So that the Nodes in the Branch can be iterated over.
pub struct BranchIntoIter<P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    branch: Branch<P>,
    layer: Layer,
    index: u8,
}

/// A Type that implements Iterator for a borrowed and mutably borrowed
/// Branch. 
pub struct BranchIterator<'a, P>
where
    P: Clone + Debug + Into<Vec<u8>>,
{
    branch: &'a Branch<P>,
    layer: Layer,
    index: u8,
}

pub struct ForkIntoIterator<P> 
where
    P: Clone + Debug + Into<Vec<u8>>
{
    fork: Fork<P>,
    nibble: Nibble,
    index: u8,
}

pub struct ForkIterator<'a, P> 
where
    P: Clone + Debug + Into<Vec<u8>>
{
    fork: &'a Fork<P>,
    nibble: Nibble,
    index: u8
}

impl<P: Clone + Debug + Into<Vec<u8>>> Root<P> {
    /// Generates a new, empty `Root`, i.e. a `Root` with a `next` that
    /// has a `nibbles` field containing 256 `Node::None`. This method
    /// is also invoked by `Root::default()`
    pub fn new() -> Root<P> {
        let next = Branch::new(Layer::Zero);
        let hash = Sha256Algorithm::hash(&next.get_hash());

        Root { next, hash }
    }

    /// Returns the `Branch` in the `Root` node.
    pub fn get_next(&self) -> Branch<P> {
        self.next.clone()
    }

    /// Returns a mutable reference to the next branch.
    pub fn get_next_mut(&mut self) -> &mut Branch<P> {
        &mut self.next
    }

    /// Returns the branch's hash
    pub fn get_hash(&self) -> RootHash {
        self.hash
    }

    /// Get's a node from the `Root` `Branch`
    pub fn get(&self, index: &u8) -> Node<P> {
        self.get_next().get(index)
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Branch<P> {
    /// Given a `Layer`, returns a new `Branch`.
    pub fn new(layer: Layer) -> Branch<P> {
        let mut nibbles: Vec<Node<P>> = Vec::with_capacity(256);
        nibbles.extend(vec![Node::None; 256]);
        Branch {
            layer,
            nibbles,
            hash: [0u8; 32],
        }
    }

    /// Inserts a `Leaf` into the `Branch` if there is a shared
    /// `Nibble`, then it traverses to the next available non-shared
    /// `Nibble` and inserts it there. If there is a shared `Nibble`
    /// with a leaf that is currently sittle on the same `Branch`, i.e.
    /// there is no `Fork` where the shared `Nibble` occurs, a `Fork`
    /// is inserted, and the `Leaf` node that the new `Leaf` had a shared
    /// Nibble with, along with the new `Leaf` attempting to be inserted
    /// are moved down to a new `Branch` that is created when a new
    /// `Fork` is created.
    ///
    /// # Example
    ///
    /// ```
    /// use mmpt::node::*;
    ///
    /// let mut root = Root::default();
    /// let payload = "Some Data".to_string();
    /// let new_leaf: Leaf<String> = Leaf::new([0u8; 32], payload);
    /// root.get_next_mut().insert(new_leaf.clone());
    /// println!("{:?}", root);
    /// let leaf = root.get_next().get(&0);
    /// println!("{:?}", leaf);
    /// assert!(leaf.is_data());
    /// match leaf {
    ///     Node::Data { data, .. } => { assert_eq!(data, new_leaf) }
    ///     _ => { panic!("Not the right type of Node") }
    /// }
    /// ```
    pub fn insert(&mut self, leaf: Leaf<P>) {
        let index = leaf.nibble as usize;
        let node = &self.nibbles[index].clone();
        match node.clone() {
            Node::Fork { mut fork, hash } => {
                fork.insert(leaf);
                let hash = fork.get_hash();
                self.nibbles[index] = Node::Fork { fork, hash };
                self.hash_nibbles();
            }
            Node::Data { data, hash } => {
                let mut layer: u8 = self.layer.clone().into();
                let fork = Fork::from((leaf, data.clone(), layer as usize));
                let hash = fork.get_hash();
                self.nibbles[index] = Node::Fork { fork, hash };
                self.hash_nibbles();
            }
            Node::None => {
                let hash = leaf.get_hash();
                self.nibbles[index] = Node::Data {
                    data: leaf,
                    hash: hash,
                };
                self.hash_nibbles();
            }
        }
    }

    /// Returns the `Node` sitting at index position `nibble`
    pub fn get(&self, nibble: &Nibble) -> Node<P> {
        self.nibbles[*nibble as usize].clone()
    }

    /// Returns the u8 representation of the `Layer`
    /// self sits at.
    pub fn get_layer(&self) -> u8 {
        self.layer.clone().into()
    }

    /// Returns a [u8; 32] representing the Sha256 hash
    /// of the current branch (i.e. the hash of all the hashes at each `nibble`)
    pub fn get_hash(&self) -> RootHash {
        self.hash
    }

    /// Get's all the not-None Nodes from the branch, concatenates their hashes
    /// in order of their index, and hashes the concatenated hash.
    pub fn hash_nibbles(&mut self) {
        let mut hash_options: Vec<Option<[u8; 32]>> =
            self.nibbles.iter().map(|node| node.get_hash()).collect();
        hash_options.retain(|hash| hash.is_some());
        let mut hashes: Vec<[u8; 32]> = hash_options.iter().map(|hash| hash.unwrap()).collect();
        let concat: Vec<u8> = hashes.into_iter().flatten().collect();
        let hash = Sha256Algorithm::hash(&concat);
        self.hash = hash;
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Fork<P> {
    /// Creates a new `Fork` given a shared `nibble` and the `layer` + 1
    /// at which the shared `nibble` was discovered, so that a new
    /// `Branch` with the conflicting `Leaf` nodes can be created.
    ///
    /// # Examples
    ///
    /// ```
    /// use mmpt::node::Fork;
    /// use mmpt::layer::Layer;
    ///
    /// let fork: Fork<String> = Fork::new(5, Layer::Two);
    /// println!("{:?}", fork);
    /// ```
    pub fn new(nibble: Nibble, layer: Layer) -> Fork<P> {
        Fork {
            nibble,
            next: Box::new(Branch::new(layer)),
        }
    }

    /// Returns the `dereferenced` i.e. `Unboxed` `Branch`
    /// underpinning this `Fork`
    pub fn get_next(&self) -> Branch<P> {
        *self.next.clone()
    }

    /// Returns the hash of the `Branch` underpinning this `Fork`
    /// hash.
    pub fn get_hash(&self) -> RootHash {
        Sha256Algorithm::hash(&self.next.get_hash())
    }

    /// Inserts a leaf into the `Branch` in the `Fork`.
    pub fn insert(&mut self, leaf: Leaf<P>) {
        self.next.insert(leaf);
    }

    /// Get the node at the index in the `Fork` `Branch`
    pub fn get(&self, index: &u8) -> Node<P> {
        self.get_next().get(index)
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Leaf<P> {
    /// Returns a new `Leaf` node given an `address`
    /// and a `payload`.
    ///
    /// # Example
    ///
    /// ```
    /// use mmpt::node::Leaf;
    ///
    /// let address = [0u8; 32];
    /// let payload = "Some Data".to_string();
    ///
    /// let leaf: Leaf<String> = Leaf::new(address, payload);
    ///
    /// println!("{:?}", leaf.get_payload());
    /// println!("{:?}", leaf.get_hash());
    /// ```
    pub fn new(address: [u8; 32], payload: P) -> Leaf<P> {
        let nibble = address[0];
        let remainder = address[1..].to_vec();
        let payload = payload;

        Leaf {
            nibble,
            address,
            remainder,
            payload,
        }
    }
    /// Returns the payload for the current leaf
    pub fn get_payload(&self) -> P {
        self.payload.clone()
    }

    pub fn get_address(&self) -> Address {
        self.address
    }

    /// Returns the hash of the current leaf
    pub fn get_hash(&self) -> RootHash {
        self.hash()
    }

    /// Hashes the serialized payload of the current leaf.
    fn hash(&self) -> RootHash {
        let mut to_hash = vec![];
        to_hash.extend(self.address);
        to_hash.extend(&self.payload.clone().into());
        Sha256Algorithm::hash(&to_hash)
    }
}

/// Converts two `Leaf` nodes with a shared nibble at a given layer, into a `Fork` with a new `Branch`
/// with the two `Leaf` nodes inserted into the new `Branch`. If another shared `Nibble` exists,
/// the `branch.insert()` method recursively keeps adding new `Fork` nodes and `Branch` nodes
/// until a unique nibble is found.
impl<P: Clone + Debug + Into<Vec<u8>>> From<(Leaf<P>, Leaf<P>, usize)> for Fork<P> {
    /// Takes two `Leaf` nodes w a shared `Nibble` and a `Layer`
    /// (represented as a `u8`) and converts them to and returns a new
    /// `Fork`
    fn from(i: (Leaf<P>, Leaf<P>, usize)) -> Fork<P> {
        let nibble = i.0.nibble;
        let leaf_1 = Leaf {
            nibble: i.0.remainder[0],
            address: i.0.get_address(),
            remainder: i.0.remainder[1..].to_vec(),
            payload: i.0.payload,
        };
        let leaf_2 = Leaf {
            nibble: i.1.remainder[0],
            address: i.1.get_address(),
            remainder: i.1.remainder[1..].to_vec(),
            payload: i.1.payload,
        };

        let layer = i.2 + 1;
        let mut next: Box<Branch<P>> = Box::new(Branch::new(layer.into()));
        let hash = [0u8; 32];

        let mut fork = Fork { nibble, next };
        fork.insert(leaf_1);
        fork.insert(leaf_2);

        fork
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Default for Root<P> {
    /// Creates and returns a `Root` node. `Root` node is always
    /// the default, i.e. initialized with an empty `Branch`
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Node<P> {
    /// If the `Node` variant is `Node::None` return true
    /// Otherwise return false
    pub fn is_none(&self) -> bool {
        match self {
            Node::None => true,
            _ => false,
        }
    }

    /// If the `Node` variant is `Node::Fork` return true
    /// otherwise, return false
    pub fn is_fork(&self) -> bool {
        match self {
            Node::Fork { .. } => true,
            _ => false,
        }
    }

    /// If the `Node` variant is `Node::Data` return true
    /// otherwise return false
    pub fn is_data(&self) -> bool {
        match self {
            Node::Data { .. } => true,
            _ => false,
        }
    }

    pub fn get_hash(&self) -> Option<[u8; 32]> {
        match self {
            Node::Fork { hash, .. } => return Some(*hash),
            Node::Data { hash, .. } => return Some(*hash),
            Node::None => return None,
        }
    }
}

/// Implements PartialEq for the `Leaf` node.
/// Two `Leaf` nodes are equal if they have the same hash.
impl<P: Clone + Debug + Into<Vec<u8>>> PartialEq for Leaf<P> {
    fn eq(&self, other: &Leaf<P>) -> bool {
        self.get_hash() == other.get_hash()
    }
    fn ne(&self, other: &Leaf<P>) -> bool {
        self.get_hash() != other.get_hash()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Eq for Leaf<P> {}

/// Implements PartialEq for `Fork` node. Two `Fork nodes
/// are equal if they have the same hash.
impl<P: Clone + Debug + Into<Vec<u8>>> PartialEq for Fork<P> {
    fn eq(&self, other: &Fork<P>) -> bool {
        self.get_hash() == other.get_hash()
    }

    fn ne(&self, other: &Fork<P>) -> bool {
        self.get_hash() != other.get_hash()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Eq for Fork<P> {}

/// Implements PartialEq for the `Branch` node. Two `Branch` nodes
/// are equal if they have the same hash.
impl<P: Clone + Debug + Into<Vec<u8>>> PartialEq for Branch<P> {
    fn eq(&self, other: &Branch<P>) -> bool {
        self.get_hash() == other.get_hash()
    }

    fn ne(&self, other: &Branch<P>) -> bool {
        self.get_hash() != other.get_hash()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Eq for Branch<P> {}

/// Implements PartialEq for the `Root` node. Two `Root` nodes
/// are equal if they have the same hash.
impl<P: Clone + Debug + Into<Vec<u8>>> PartialEq for Root<P> {
    fn eq(&self, other: &Root<P>) -> bool {
        self.get_hash() == other.get_hash()
    }

    fn ne(&self, other: &Root<P>) -> bool {
        self.get_hash() != other.get_hash()
    }
}

impl<P: Clone + Debug + Into<Vec<u8>>> Eq for Root<P> {}

/// Implements IntoIterator, converting a `Branch` node into a `BranchIntoIter`
/// which can then be iterated over. 
impl<P: Clone + Debug + Into<Vec<u8>>> IntoIterator for Branch<P> {
    type Item = Node<P>;
    type IntoIter = BranchIntoIter<P>;

    fn into_iter(self) -> Self::IntoIter {
        let layer = self.layer.clone();
        BranchIntoIter {
            branch: self,
            layer: layer,
            index: 0u8,
        }
    }
}

/// Build a type from Branch that implements Iterator
impl<'a, P: Clone + Debug + Into<Vec<u8>>> IntoIterator for &'a Branch<P> {
    type Item = Node<P>;
    type IntoIter = BranchIterator<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        let layer = self.layer.clone();
        BranchIterator {
            branch: self,
            layer: layer,
            index: 0u8,
        }
    }
}

/// Builds a type from a borrowed mutable Branch that implements Iterator
impl<'a, P: Clone + Debug + Into<Vec<u8>>> IntoIterator for &'a mut Branch<P> {
    type Item = Node<P>;
    type IntoIter = BranchIterator<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        let layer = self.layer.clone();
        BranchIterator {
            branch: self,
            layer: layer,
            index: 0u8,
        }
    }
}

/// Implements Iterator for the BranchIterator type.
impl<'a, P: Clone + Debug + Into<Vec<u8>>> Iterator for BranchIterator<'a, P> {
    type Item = Node<P>;
    
    fn next(&mut self) -> Option<Node<P>> {
        if let None = self.index.checked_add(1) {
            return None;
        } else {
            return Some(self.branch.nibbles[self.index as usize].clone());
        }
    }
}

/// Implements Iterator for BranchIntoIterator type.
impl<P: Clone + Debug + Into<Vec<u8>>> Iterator for BranchIntoIter<P> {
    type Item = Node<P>;

    fn next(&mut self) -> Option<Node<P>> {
        if let None = self.index.checked_add(1) {
            return None;
        } else {
            return Some(self.branch.nibbles[self.index as usize].clone());
        }
    }
}
