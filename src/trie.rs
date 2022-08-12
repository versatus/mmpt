use crate::node::{Nibble, Node, Root};

#[derive(Clone, Debug)]
pub struct Trie<P> {
    pub root: Box<Root<P>>,
}

impl<P: Clone> Trie<P> {
    pub fn new(root: Box<Root<P>>) -> Trie<P> {
        Trie { root }
    }

    /// Traverses the trie, every time there's a Fork node discovered
    /// `.get_next()` is called on the Fork node to get the next branch
    /// and check the next nibble for an entry.
    /// However if a data node is discovered, it is returned since a Data node
    /// is the "end of the road" so to speak. If a None node is discovered then
    /// the last Fork node discovered is returned, if there was no Fork node discovered
    /// then a None node is returned.
    pub fn traverse(&self, nibbles: &[u8]) -> (usize, Option<Nibble>, Node<P>) {
        let mut branch = self.root.get_next();
        let mut iter = nibbles.iter().enumerate();
        let mut data: Node<P> = Node::None;
        loop {
            match iter.next() {
                Some((index, nibble)) => match branch.get(nibble) {
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
}
