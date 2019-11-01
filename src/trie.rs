use std::collections::HashMap;
use std::hash::Hash;

struct TrieNode<T> where T: Hash + Eq {
    value: Option<T>,
    children: HashMap<T, TrieNode<T>>,
    is_terminal: bool,
    level: i64
}

impl<T> TrieNode<T> where T: Hash + Eq + Copy{
    fn new(given_value: Option<T>, given_is_terminal: bool, given_level: i64) -> TrieNode<T>{
        TrieNode {
            value: given_value,
            children: HashMap::new(),
            is_terminal: given_is_terminal,
            level: given_level
        }
    }
    
}


pub struct Trie<T> where T: Hash + Eq + Copy {
    root: TrieNode<T>
}


impl<'a, T> Trie<T> where T: Hash + Eq + Copy {
    pub fn new() -> Trie<T>{
        Trie{
            root: TrieNode::new(None, false, -1)
        }
    }


    pub fn insert(&mut self, prefix: &Vec<T>){
        let mut last_node = &mut self.root;

        let mut count = 0;
        for i in prefix{
            count = count+1;
            last_node = last_node.children.entry(*i).or_insert(TrieNode::new(Some(*i), false, count));
        }

        last_node.is_terminal = true;
    }

    pub fn has_prefix (&mut self, prefix: &Vec<T>) -> bool{
        let mut last_node = &mut self.root;

        for i in prefix{
            match last_node.children.get_mut(i) {
                Some(v) => last_node = v,
                None => return false
            }
        }

        true
    }

    pub fn has_word (&mut self, word: &Vec<T>) -> bool{
        let mut last_node = &mut self.root;

        for i in word{
            match last_node.children.get_mut(i) {
                Some(v) => last_node = v,
                None => return false
            }
        }

        last_node.is_terminal
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_has_prefix(){
        let mut trie: Trie<char> = Trie::new();

        let words = vec!["aaaa"];

        let mut words_as_chars: Vec<Vec<char>> = Vec::new();

        assert!(!trie.has_prefix(&get_chars_str(words[0])));

        for i in &words{
            let word_as_chars: Vec<char> = get_chars_str(*i);
            words_as_chars.push(word_as_chars.clone());
            trie.insert(&word_as_chars);

        }

        assert!(trie.has_prefix(&words_as_chars[0]));

    }


    #[test]
    fn test_has_word (){
         let mut trie: Trie<char> = Trie::new();

        let words = vec!["aaaa"];

        let mut words_as_chars: Vec<Vec<char>> = Vec::new();

        assert!(!trie.has_word(&get_chars_str(words[0])));

        for i in &words{
            let word_as_chars: Vec<char> = get_chars_str(*i);
            words_as_chars.push(word_as_chars.clone());
            trie.insert(&word_as_chars);

        }

        let short_a = vec!['a', 'a', 'a'];

        assert!(!trie.has_word(&short_a));
        assert!(trie.has_prefix(&short_a));

        assert!(trie.has_word(&words_as_chars[0]));
    
    }




    fn get_chars(word: &String) -> Vec<char> {
        word.chars().collect()
    } 
    fn get_chars_str(word: &str) -> Vec<char> {
        word.chars().collect()
    } 
    
    
}