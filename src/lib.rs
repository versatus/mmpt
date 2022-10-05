pub mod node;
pub mod trie;
pub mod layer;
pub mod hash;

#[cfg(test)]
mod tests {
    use crate::trie::Trie;
    use crate::node::{Branch, Leaf, Root, Node};

    #[test]
    fn create_new_empyt_trie() {
        let trie: Trie<Vec<u8>> = Trie::default();
        assert_eq!(trie.root.get_next().get_layer(), 0u8);
    }

    #[test]
    fn insert_node_in_branch() {
        let mut root = Root::default();
        let payload = "Some Data".to_string();
        let new_leaf: Leaf<String> = Leaf::new([0u8; 32], payload);
        root.get_next_mut().insert(new_leaf.clone());
        let leaf = root.get_next().get(&0);
        assert!(leaf.is_data());
        match leaf {
            Node::Data { data, .. } => { assert_eq!(data, new_leaf) }
            _ => { panic!("Not the right type of Node") }
        }
    }

    #[test]
    fn add_node_to_trie() {
        let mut trie: Trie<String> = Trie::default();
        let payload = "Some Data".to_string();
        let new_leaf: Leaf<String> = Leaf::new([0u8; 32], payload);
        trie.add(new_leaf.clone());
        let leaf = trie.get(&0);
        assert!(leaf.is_data());
        match leaf {
            Node::Data { data, .. } => { assert_eq!(data, new_leaf) }
            _ => { panic!("Not the right type of Node") }
        }

    }

    #[test]
    fn adding_shared_nibble_node_creates_new_branch() {
        let mut branch: Branch<String> = Branch::new(0u8.into());
        let payload = "Some Data".to_string();
        let leaf_1: Leaf<String> = Leaf::new([0u8; 32], payload);
        let mut leaf_2_address = [0u8; 32];
        leaf_2_address[1] = 1;
        let leaf_2_payload = "Some More Data".to_string();
        let leaf_2: Leaf<String> = Leaf::new(leaf_2_address, leaf_2_payload);

        branch.insert(leaf_1.clone());
        branch.insert(leaf_2.clone());

        let node = branch.get(&0);
        assert!(node.is_fork());
        match node {
            Node::Fork { fork, .. } => { 
                let fork_leaf_1 = fork.get(&0);
                let fork_leaf_2 = fork.get(&1);
                assert!(fork_leaf_1.is_data());
                assert!(fork_leaf_2.is_data());
                if let Node::Data { data, .. } = fork_leaf_1 {
                    assert_eq!(leaf_1, data);
                }
                
                if let Node::Data { data, .. } = fork_leaf_2 {
                    assert_eq!(leaf_2, data);
                }
            }
            _ => { panic!("Wrong node type")}
        }

    }

    #[test]
    fn traverse_trie_to_find_node_that_exists_returns_the_node() {

    }

    #[test]
    fn traverse_trie_to_find_node_that_doesnt_exist_returns_none_node() {

    }

    #[test]
    fn traverse_trie_to_find_node_ending_with_shared_nibble_returns_fork() {

    }

    #[test]
    fn next_on_trie_iterator_of_empty_returns_none() {

    }

    #[test]
    fn next_on_trie_iterator_at_end_of_branch_moves_up_one_layer() {

    }

    #[test]
    fn next_on_trie_iterator_at_fork_moves_down_one_layer() {

    }

    #[test]
    fn calling_count_on_trie_iterator_returns_count_of_all_data_nodes_in_trie() {

    }

    #[test]
    fn calling_count_on_branch_iterator_returns_count_of_all_data_nodes_in_branch_and_all_data_nodes_in_all_forks_in_branch() {

    }
}
