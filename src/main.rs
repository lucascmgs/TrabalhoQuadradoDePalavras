mod trie;

use trie::Trie;

struct ResolveQuadrado {
    quadrado: Vec<Vec<char>>,
    quadrado_otimo: Vec<Vec<char>>,
    conjuntos_palavras: Vec<Vec<String>>,
    palavras_atuais: Vec<String>,
    prefixos: Trie<char>,
}

impl ResolveQuadrado {
    fn new() -> ResolveQuadrado {
        ResolveQuadrado {
            quadrado: Vec::new(),
            quadrado_otimo: Vec::new(),
            conjuntos_palavras: Vec::new(),
            palavras_atuais: Vec::new(),
            prefixos: Trie::new(),
        }
    }

    fn resolve(&mut self, palavras: &Vec<String>) {
        //Para cada palavra vamos colocá-la em um "bucket" com o tamanho correspondente. Os buckets são as células de índices correspodentes em self.conjuntos_palavras
        for pal in palavras {
            //Se o bucket não existe, damos resize para que seja criado
            if pal.len() > self.conjuntos_palavras.len() {
                self.conjuntos_palavras.resize(pal.len() + 1, Vec::new());
            }
            self.conjuntos_palavras[pal.len()].push(pal.clone());
            //Inserimos a palavra nova na árvore de prefixos
            self.prefixos.insert(&pal.chars().collect());
        }
        println!("{:?}", self.conjuntos_palavras);
        //Pegamos o maior índice de bucket para testar se é possível formar um quadrado de palavras com ele
        let mut count = self.conjuntos_palavras.len() - 1;
        loop {
            //Se chegamos a 0, que pode ser porque testamos todos os buckets ou nos foi passado 0 palavras, paramos
            if count == 0 {
                println!("Não é possível formar um quadrado!");
                break;
            }
            //Clonamos o bucket de palavras a serem testadas
            self.palavras_atuais = self.conjuntos_palavras[count].clone();
            self.recorre(count);
            //Se, após chamar a recursão temos um quadrado ótimo, imprimimos ele
            if self.quadrado_otimo.len() > 0 {
                println!("Temos o quadrado {:?}", self.quadrado_otimo);
                break;
            } 
            //Se não, limpamos o quadrado e tentamos na próxima iteração
            else {
                self.quadrado.clear();
            }
            count = count - 1;
        }
    }

    //Checa se as colunas do quadrado atual com os caracteres correspondentes da nova palavra formam um prefixo
    fn checa_prefixos(&mut self, palavra: &String) -> bool{
        println!("{:?}", self.quadrado);
        //Vetorizamos os caracteres da palavra
        let chars: Vec<char> = palavra.chars().collect();
        for ind_char in 0..palavra.len() {
            let mut pref = Vec::new();
            //Percorremos as colunas do quadrado atual, acrescentando os caracteres encontrados ao prefixo que queremos checar
            for ind_pal in 0..self.quadrado.len(){
                pref.push(self.quadrado[ind_pal][ind_char]);
            }
            pref.push(chars[ind_char]);
            if !self.prefixos.has_prefix(&pref){
                println!("Não temos o prefixo {:?}", pref);
                return false;
            }
        }

        true
    }

    fn recorre(&mut self, index: usize) {
        println!("{:?}", self.palavras_atuais);
        //Se obtemos um quadrado com a quantidade de palavras igual ao tamanho de palavra atual, encontramos o quadrado ótimo
        if self.quadrado.len() == index {
            self.quadrado_otimo = self.quadrado.clone();
            return;
        }

        if self.palavras_atuais.len() > 0 {
            //Iteramos sobre todas as palavras atuais
            let mut ind = self.palavras_atuais.len() - 1;
            loop {
                let mut recorreu = false;
                let pal = self.palavras_atuais.get_mut(ind).unwrap().clone();
                self.palavras_atuais.remove(ind);
                
                //Se os caracteres da palavra atual formam um prefixo com o quadrado atual, adicionamos essa palavra ao quadrado e seguimos o backtrack
                if self.checa_prefixos(&pal){
                    self.quadrado.push(pal.chars().collect());
                    //Backtrack
                    self.recorre(index);
                    self.quadrado.pop();
                    recorreu = true;
                }
                self.palavras_atuais.insert(ind, pal);
                
                if ind == 0{
                    break;
                }
                ind = ind - 1;
            }
        }
    }
}

fn main() {
    let palavras = vec![
        String::from("ama"),
        String::from("tec"),
        String::from("mas"),
        String::from("pic"),
        String::from("asa"),
    ];
    let mut resolve_quadrado = ResolveQuadrado::new();
    resolve_quadrado.resolve(&palavras);
}
