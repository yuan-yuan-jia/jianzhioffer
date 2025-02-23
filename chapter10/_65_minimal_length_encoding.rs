
struct TrieNode {
    is_word: bool,
    children: std::collections::HashMap<char, TrieNode>,
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
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self { root: TrieNode::new() }
    }

    fn insert(&mut self, word: &str) {
        if word.len() < 1 {
            return;
        }
        let mut node = &mut self.root;
        for i in word.chars() {
            node = node.children.entry(i).or_insert(TrieNode::new());
        }
        node.is_word = true;   
    }

    fn do_dfs(&self, node: &TrieNode,length: i32, sum_length: &mut i32) {
        if node.children.is_empty() {
            *sum_length = *sum_length + length;
            if node.is_word {
                // #分隔符的长度
                *sum_length = *sum_length + 1;
            }
        }else {
            let new_length = length + 1;
            for i in &node.children {
                self.do_dfs(i.1, new_length, sum_length);
            } 
        }
    }
    fn dfs(&self, sum_length: &mut i32) {
        self.do_dfs(&self.root, 0, sum_length);
    } 
}



fn minium_length_encoding(trie:&mut Trie,words: &[&str]) -> i32 {
    let mut r = 0;
    // 先插入
    for i in words {
       let mut s = String::new();
       for c in i.chars().rev() {
            s.push(c);
       }
       if s.len() > 0 {
            trie.insert(s.as_str());
       }   
    }

    // 开始计算长度
    trie.dfs(&mut r);
    r
}

fn main() {
    {// case one
        let mut trie = Trie::new();
        let words = ["time","me","bell"];
        let r = minium_length_encoding(&mut trie, &words[..]);
        println!("r = {r}");
        assert_eq!(r, 10);
    }


    {// case two
        let mut trie = Trie::new();
        let words = ["at","bat","cat"];
        let r = minium_length_encoding(&mut trie, &words[..]);
        println!("r = {r}");
        assert_eq!(r, 8);
    }

}