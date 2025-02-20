


struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    
    fn new() -> Self {
        Self { 
            children: std::collections::HashMap::new(), 
            is_word: false 
        }
    }
}

struct Trie {
    root: TrieNode
}

#[allow(unused)]
impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, str: &str) {
        let mut node = &mut self.root;

        for i in str.chars() {
            node = node.children.entry(i).or_insert(TrieNode::new());   
        }
        node.is_word = true;
    }
    
    
    fn search(&self, str: &str) -> bool {
        let mut node = &self.root;
        for i in str.chars() {
            if let Some(e) = node.children.get(&i) {
                node = e;
            }else {
                return false;
            }
        }
        node.is_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for i in prefix.chars() {
            if let Some(e) = node.children.get(&i) {
                node = e;
            }else {
                return true;
            }
        }
        true
    }


    fn get_prefix(&self,str: &str) -> Option<String> {
        let mut prefix = String::new();
        let mut node = &self.root;
        for i in str.chars() {
            if let Some(e) = node.children.get(&i) {
                node = e;
                prefix.push(i);
            }else {
                break;
            }
        }

        if prefix.is_empty() || !node.is_word {
            None
        }else {
            Some(prefix)
        }
    }


}




fn main() {
    let mut trie = Trie::new();
    trie.insert("cat");
    trie.insert("bat");
    trie.insert("rat");

    let statement = "the cattle was rattled by the battery";
    let words = statement.split(" ");
    let mut new_statement = String::new();    
    for i in words {
        if let Some(prefix) = trie.get_prefix(i) {
            new_statement.push_str(prefix.as_str());
        }else {
            new_statement.push_str(i);
        }
        new_statement.push_str(" ");
    }
    new_statement = new_statement.trim_end().to_string();
    println!("{}", new_statement);
    assert_eq!(new_statement, "the cat was rat by the bat")
}