use std::str::Chars;



struct TrieNode {
    is_word:bool,
    children: std::collections::HashMap<char, TrieNode>
}


impl TrieNode {
    fn new() -> Self {
        Self {
            is_word: false,
            children: std::collections::HashMap::new(),
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

    fn insert(&mut self, str:&str) {
        let mut node = &mut self.root;
        for c in str.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());   
        }
        node.is_word = true;
    }

    
    fn do_search(&self, mut chars: Chars, changed: bool,node: &TrieNode) -> bool {
        let char = chars.next();
        if let Some(cc) = char {
            if node.children.contains_key(&cc) {
                let n = node.children.get(&cc).unwrap();
                if n.is_word && chars.clone().next().is_none() {
                    // 找到最终的单词
                    return true;
                }
                return self.do_search(chars, changed, n);
            }else {

                if changed {
                    return false;
                }
                for key in node.children.keys() {
                    let cur = self.do_search(chars.clone(), true, node.children.get(key).unwrap());
                    if cur {
                        return true;
                    }
                }
                false
            }
            
        }else {
            return false;
        }   
    }

    fn search(&self, str: &str, changed: bool) -> bool {
       let chars = str.chars();
       self.do_search(chars, changed, &self.root)
    } 


}

struct MagicDictinary {
    trie: Trie,
}

impl MagicDictinary {
    fn build_dict(&mut self,strs: &[&str]) {
        for str in strs {
            self.trie.insert(str);
        }
    }    

    fn search(&self, str: &str) -> bool {
        self.trie.search(str, false)
    } 

    fn new() -> Self {
        Self { trie: Trie::new() }
    }
}




/*Drive Code */
fn main() {
    let strs = ["happy", "new", "year"];
    let mut magic_dictionary  = MagicDictinary::new();
    magic_dictionary.build_dict(&strs[..]);

    let word = "now";
    let result = magic_dictionary.search(word);
    println!("result: {}", result);

    assert_eq!(result, true);

}