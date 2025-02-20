
struct TrieNode {
    chidren: Vec<Option<TrieNode>>,
    is_word: bool,
}


impl TrieNode {
    fn new() -> Self {
        let mut v: Vec<Option<TrieNode>> = vec![];
        for _ in 0..26 {
            v.push(None);
        }
        Self {
            chidren: v,
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, str: &str) {

        let mut node = &mut self.root;

        for c in str.chars() {
            let idx = c as usize - 'a' as usize;
            if idx > 25 {
                panic!("不是英文字符");
            }
            
            if node.chidren.get(idx).unwrap().is_none() {
                
                node.chidren[idx]  = Some(TrieNode::new());
            }
            
            node = node.chidren.get_mut(idx).unwrap().as_mut().unwrap();

        }
        node.is_word = true;
    }

    fn search(&self, str: &str) -> bool {
        let mut node = &self.root;
        for c in str.chars() {
            let idx = c as usize - 'a' as usize;
            if idx > 25 {
                panic!("only support english char");
            }
            // 本身存的值已经是None，所以是有值的
            if node.chidren.get(idx).unwrap().is_none() {
                return false;
            }
            
            node = node.chidren.get(idx).unwrap().as_ref().unwrap();
        }

        node.is_word
    }

    fn startsWith(&self, str: &str) -> bool {
        let mut node = &self.root;
        for c in str.chars() {
            let idx = c as usize - 'a' as usize;
            if idx > 25 {
                panic!("only support english char");
            }
            if node.chidren.get(idx).unwrap().is_none() {
                return false;
            }

            node = node.chidren.get(idx).unwrap().as_ref().unwrap();
        }

        true
    }

}




fn main() {
    let mut trie = Trie::new();

    trie.insert("goodbye");

    assert_eq!(trie.startsWith("good"), true);
    trie.insert("good");
    assert_eq!(trie.search("good"), true);
}