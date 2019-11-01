mod trie;

use trie::Trie;
use std::collections::HashMap;


struct ResolveQuadrado{
    quadrado: Vec<Vec<char>>,
    quadrado_otimo: Vec<Vec<char>>,
    conjuntos_palavras: Vec<Vec<String>>,
    palavras_atuais: Vec<String>,
    prefixos: Trie<char>
}

impl ResolveQuadrado {
    fn new () -> ResolveQuadrado{
        ResolveQuadrado{quadrado: Vec::new(), quadrado_otimo: Vec::new(),
                                 conjuntos_palavras: Vec::new(), palavras_atuais: Vec::new(), 
                                 prefixos: Trie::new()}
    }

    fn resolve(&mut self, palavras: &Vec<String>){
        self.conjuntos_palavras.resize(palavras.len()+1, Vec::new());
        for pal in palavras {

            self.conjuntos_palavras[pal.len()].push(pal.clone());
                
            
            self.prefixos.insert(&pal.chars().collect());
        }

        let mut count = self.conjuntos_palavras.len() - 1;
        loop{
            if count == 0 {
                println!("Não é possível formar um quadrado!");
                break;
            }
            self.palavras_atuais = self.conjuntos_palavras[count].clone();
            self.recorre(count);
            if self.quadrado_otimo.len() > 0 {
                println!("{:?}", self.quadrado_otimo);
                break;
            } else {
                self.quadrado.clear();
            }
            count = count - 1;
        }


    }

    fn recorre(&mut self, index: usize){
        if self.palavras_atuais.is_empty() && self.quadrado.len() == index{
            self.quadrado_otimo = self.quadrado.clone();
            return;
        }


        for i in 0..self.palavras_atuais.len(){
            let pal = self.palavras_atuais[i].clone();
            self.palavras_atuais.pop().unwrap();
            let mut char_index = 0;
            let mut can_add = true;
            for c in pal.chars(){
                let mut pref = Vec::new();
                for ind in 0..self.quadrado.len(){
                    pref.push(self.quadrado[ind][char_index]);
                }
                pref.push(c);
                if !self.prefixos.has_prefix(&pref){
                    can_add = false;
                    break;
                }
                char_index = char_index + 1;
            }

            if can_add {
                self.quadrado.push(pal.chars().collect());
            }
            self.palavras_atuais.push(pal.clone());
        }
    }
}


fn main() {
    let palavras = vec![String::from("ama"), String::from("mas"), String::from("asa")];
    let mut resolve_quadrado = ResolveQuadrado::new();
    resolve_quadrado.resolve(&palavras);
}