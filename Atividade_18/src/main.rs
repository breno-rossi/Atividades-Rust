use std::fmt::Debug;
use std::mem;

#[derive(Debug)]
enum LinkedList<T> {
    Node(T, Box<LinkedList<T>>),
    Nil,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::Nil
    }
}

impl<T> LinkedList<T> {
    /// Adiciona um valor ao final da lista.
    pub fn push_back(&mut self, value: T) {
        match self {
            // Se a lista está vazia, cria um novo nó
            LinkedList::Nil => *self = LinkedList::Node(value, Box::new(LinkedList::Nil)),
            // Caso contrário, continua recursivamente até encontrar o final
            LinkedList::Node(_, tail) => tail.push_back(value),
        }
    }
    
    /// Adiciona um valor no início da lista..
    pub fn push_front(&mut self, value: T) {
        // Move o conteúdo atual de self para old_list e coloca Nil temporariamente em self
        let old_list = mem::replace(self, LinkedList::Nil);
        
        *self = LinkedList::Node(value, Box::new(old_list));
    }
    
    /// Insere um valor em um índice específico da lista.
    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            // Inserir no índice 0 é o mesmo que push_front
            self.push_front(value);
        } else {
            match self {
                // Se chegamos ao final antes do índice, está fora dos limites
                LinkedList::Nil => panic!("Indice fora dos limites: Tentou inserir no indice {}", index),
                // Continua recursivamente decrementando o índice
                LinkedList::Node(_, tail) => tail.insert(index - 1, value),
            }
        }
    }
    
    /// Remove o nó em um índice específico e retorna seu valor.
    /// Retorna None se o índice estiver fora dos limites.
    pub fn delete(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            // Remover no índice 0 é o mesmo que pop_front
            self.pop_front()
        } else {
            match self {
                // Se a lista está vazia, não há nada para remover
                LinkedList::Nil => None,
                // Continua recursivamente decrementando o índice
                LinkedList::Node(_, tail) => tail.delete(index - 1),
            }
        }
    }
    
    /// Retorna o número de elementos na lista.
    pub fn len(&self) -> usize {
        match self {
            LinkedList::Nil => 0,
            LinkedList::Node(_, tail) => 1 + tail.len(),
        }
    }
    
    /// Remove o primeiro elemento da lista e o retorna.
    /// Retorna None se a lista estiver vazia.
    pub fn pop_front(&mut self) -> Option<T> {
        // Move o conteúdo de self para uma variável temporária
        // e coloca Nil em seu lugar
        match mem::replace(self, LinkedList::Nil) {
            LinkedList::Nil => None,
            LinkedList::Node(value, tail) => {
                // Promove a cauda para ser a nova lista
                *self = *tail;
                // Retorna o valor que estava no primeiro nó
                Some(value)
            }
        }
    }
    
    /// Remove o último elemento da lista e o retorna.
    /// Retorna None se a lista estiver vazia.
    /// Complexidade: O(N) - precisa percorrer até o penúltimo nó.
    pub fn pop_back(&mut self) -> Option<T> {
        match self {
            // Se a lista está vazia, retorna None
            LinkedList::Nil => None,
            LinkedList::Node(_, tail) => {
                // Verifica se a cauda é Nil (este é o último nó)
                if matches!(**tail, LinkedList::Nil) {
                    // Remove este nó e retorna seu valor
                    match mem::replace(self, LinkedList::Nil) {
                        LinkedList::Node(value, _) => Some(value),
                        LinkedList::Nil => None,
                    }
                } else {
                    // Continua recursivamente até encontrar o último nó
                    tail.pop_back()
                }
            }
        }
    }
    
    /// Remove e retorna o elemento em um índice específico.
    /// Retorna None se o índice estiver fora dos limites.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.delete(index)
    }
    
    /// Anexa todos os elementos de outra lista ao final desta.
    /// A lista other_list se torna vazia após a operação.
    pub fn append(&mut self, other_list: &mut Self) {
        match self {
            LinkedList::Nil => {
                // Se self está vazia, move other_list para self
                // e deixa other_list como Nil
                *self = mem::replace(other_list, LinkedList::Nil);
            }
            LinkedList::Node(_, tail) => {
                // Recursivamente encontra o final de self e anexa other_list
                tail.append(other_list);
            }
        }
    }
    
    /// Divide a lista em duas no índice fornecido
    pub fn split_off(&mut self, index: usize) -> Self {
        if index == 0 {
            // Se o índice é 0, retorna toda a lista atual
            // e deixa self como Nil
            mem::replace(self, LinkedList::Nil)
        } else {
            match self {
                // Se chegamos ao final antes do índice, retorna Nil
                LinkedList::Nil => LinkedList::Nil,
                LinkedList::Node(_, tail) => {
                    tail.split_off(index - 1)
                }
            }
        }
    }
}

// Implementação separada para métodos que requerem Debug
impl<T: Debug> LinkedList<T> {
    pub fn display_list(&self) {
        match self {
            LinkedList::Nil => println!("nil"),
            LinkedList::Node(value, tail) => {
                print!("{:?} -> ", value);
                tail.display_list();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::Nil;
        list.push_back(10);
        list.push_back(20);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::Nil;
        list.push_back(10);
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(10));
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::Nil;
        assert_eq!(list.pop_front(), None);
        
        list.push_back(10);
        list.push_back(20);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::Nil;
        assert_eq!(list.pop_back(), None);
        
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::Nil;
        assert_eq!(list.len(), 0);
        
        list.push_back(10);
        assert_eq!(list.len(), 1);
        
        list.push_back(20);
        list.push_back(30);
        assert_eq!(list.len(), 3);
        
        list.pop_front();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_delete() {
        let mut list = LinkedList::Nil;
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        
        assert_eq!(list.delete(1), Some(20));
        assert_eq!(list.len(), 2);
        assert_eq!(list.delete(0), Some(10));
        assert_eq!(list.delete(0), Some(30));
        assert_eq!(list.delete(0), None);
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::Nil;
        list.push_back(10);
        list.push_back(30);
        list.insert(1, 20);
        
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(30));
    }

    #[test]
    fn test_append() {
        let mut list1 = LinkedList::Nil;
        list1.push_back(10);
        list1.push_back(20);
        
        let mut list2 = LinkedList::Nil;
        list2.push_back(30);
        list2.push_back(40);
        
        list1.append(&mut list2);
        
        assert_eq!(list1.len(), 4);
        assert_eq!(list2.len(), 0);
        
        assert_eq!(list1.pop_front(), Some(10));
        assert_eq!(list1.pop_front(), Some(20));
        assert_eq!(list1.pop_front(), Some(30));
        assert_eq!(list1.pop_front(), Some(40));
    }

    #[test]
    fn test_split_off() {
        let mut list = LinkedList::Nil;
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        
        let mut split = list.split_off(2);
        
        assert_eq!(list.len(), 2);
        assert_eq!(split.len(), 2);
        
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(20));
        
        assert_eq!(split.pop_front(), Some(30));
        assert_eq!(split.pop_front(), Some(40));
    }

    #[test]
    fn test_with_strings() {
        let mut list = LinkedList::Nil;
        list.push_back(String::from("Ola"));
        list.push_back(String::from("Mundo"));
        
        assert_eq!(list.pop_front(), Some(String::from("Ola")));
        assert_eq!(list.pop_front(), Some(String::from("Mundo")));
    }

    // Teste com tipo que não implementa Clone
    struct NaoClonaval(i32);
    
    #[test]
    fn test_with_non_cloneable() {
        let mut list = LinkedList::Nil;
        list.push_back(NaoClonaval(10));
        list.push_back(NaoClonaval(20));
        
        assert_eq!(list.len(), 2);
        let val = list.pop_front();
        assert!(val.is_some());
        assert_eq!(val.unwrap().0, 10);
    }

    #[test]
    fn test_edge_cases() {
        // Teste com lista vazia
        let mut list: LinkedList<i32> = LinkedList::Nil;
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        
        // Teste com um único elemento
        list.push_back(42);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(42));
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_multiple_operations() {
        let mut list = LinkedList::Nil;
        
        // Adiciona elementos
        list.push_back(1);
        list.push_back(2);
        list.push_front(0);
        list.insert(2, 15);
        
        // Lista esperada: 0 -> 1 -> 15 -> 2
        assert_eq!(list.len(), 4);
        
        // Remove elementos
        assert_eq!(list.delete(2), Some(15));
        assert_eq!(list.len(), 3);
        
        // Lista esperada: 0 -> 1 -> 2
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 0);
    }
}

fn main() {
    let mut list = LinkedList::Nil;
    
    println!("Executando push_back de 10, 20, 30");
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    list.display_list();
    
    println!("Executando push_front de 5");
    list.push_front(5);
    list.display_list();
    
    println!("Inserindo 15 na 2a posicao (indice 2)");
    list.insert(2, 15);
    list.display_list();
    
    println!("Deletando elemento no indice 1");
    list.delete(1);
    list.display_list();
    
    println!("\nTestando outras operacoes:");
    println!("Tamanho da lista: {}", list.len());
    
    println!("Removendo primeiro elemento: {:?}", list.pop_front());
    list.display_list();
    
    println!("Removendo ultimo elemento: {:?}", list.pop_back());
    list.display_list();
    
    println!("\nTestando append:");
    let mut list2 = LinkedList::Nil;
    list2.push_back(100);
    list2.push_back(200);
    println!("Lista 2 antes do append:");
    list2.display_list();
    
    list.append(&mut list2);
    println!("Lista 1 apos append:");
    list.display_list();
    println!("Lista 2 apos append (deve estar vazia):");
    list2.display_list();
    
    println!("\nTestando split_off no indice 2:");
    let split = list.split_off(2);
    println!("Lista original apos split:");
    list.display_list();
    println!("Lista resultante do split:");
    split.display_list();
}