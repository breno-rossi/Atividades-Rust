
pub struct PilhaLimitada<T>{
    elementos: Vec<T>,
    capacidade: usize
}

impl<T> PilhaLimitada<T>{

    //Metodo construtor
    fn new(capacidade:usize) -> Self{
        PilhaLimitada{
            elementos: Vec::with_capacity(capacidade),
            capacidade: capacidade
        }
    }

    //Metodo adiciona item no topo da fila
    fn push(&mut self,item:T) -> Result<(), String>{
        if self.is_full(){
            Err("A pilha está cheia".to_string())
        }else {
            self.elementos.push(item);
            Ok(())
        }
    }

    fn pop(& mut self) -> Option<T>{
        self.elementos.pop()
    }

    fn peek(&self) -> Option<&T>{
        self.elementos.last()
    }

    fn is_empty(&self) -> bool{
        self.elementos.is_empty()
    }

    fn is_full (&self) -> bool {
        self.elementos.len() == self.capacidade
    }

    pub fn capacity(&self) -> usize {
        self.capacidade
    }

    pub fn len(&self) -> usize{
        self.elementos.len()
    }
}



fn main() {
    println!("=========DEMOSNTRAÇÂO com i32==========");

    println!("PILHA 1 <i32> Capacidade 3");
    let mut pilha1 = PilhaLimitada::<i32>::new(3);
    println!("Capacidade: {} Tamanho Atual: {}", pilha1.capacity(), pilha1.len());
    println!("Está vazia? {}", pilha1.is_empty());
    println!("Está cheia? {}\n", pilha1.is_full());


    for valor in [1,2,3]{
        match  pilha1.push(valor){
            Ok(()) =>  println!("Adicionado: {}",valor),
            Err(e) => println!("Erro ao adiciona:{}, {}", valor, e),
        }
    }
    println!("Esta cheia: {} Tamanho Atual: {}", pilha1.is_full(), pilha1.capacity());


    if let Some(topo) = pilha1.peek(){
        println!("elemento no topo: {}\n",topo)
    }

    match pilha1.push(555){
        Ok(()) => println!("Adicionado: {}",555),
        Err(e) => println!("Erro ao adicionar:{}, {}\n", 555, e),
    }

    println!("=========DEMOSNTRAÇÂO com string==========");

    let mut pilha2 = PilhaLimitada::<String>::new(3);

    let palavras = ["PUC".to_string(),"RUST".to_string(),"PROGRAMAÇÂO".to_string()];
    for palavra in palavras{
        match pilha2.push(palavra.clone()) {
            Ok(()) => println!("Adicionado: {}",palavra),
            Err(e) => println!("Erro ao adiciona:{}, {}", palavra, e),
        }
    }

    println!("\nConteúdo atual da pilha de strings: {}",pilha2.len());



    while let Some(palavra) = pilha2.pop() {
        println!("Removido: '{}'", palavra);
    }

    println!("\nConteúdo atual da pilha de strings: {}",pilha2.len());
}
