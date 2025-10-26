#[derive(Clone)]
enum LinkedList{
    Nil,
    Node(i32, Box<LinkedList>),
}

impl LinkedList{
    pub fn display_list(&self){
        match self{
            LinkedList::Nil => println!("nil"),
            LinkedList::Node(value, tail ) => {
                print!("{:?}->", value);
                tail.display_list();
            }
        }
    }

    pub fn push_back(&mut self, value: i32){
        match self{
            LinkedList::Nil => *self = LinkedList::Node(value, Box::new(LinkedList::Nil)),
            LinkedList::Node(v, tail) => tail.push_back(value),
        }
    }

    pub fn push_front(&mut self, value: i32){
        *self = LinkedList::Node(value, Box::new(self.clone()));
    }

    pub fn insert(&mut self, index: usize, value: i32){
        if index == 0 {
            self.push_front(value);
        } else {
            match self {
                LinkedList::Nil => panic!("Index error!"),
                LinkedList::Node(_, tail) => {
                    tail.insert(index-1, value);
                }
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<i32>{
        if index == 0{
            match self{
                LinkedList::Nil => None,
                LinkedList::Node(v, tail ) => {
                    let temp = *v;
                    *self = *tail.clone();
                    Some(temp)
                }
            }
        } else{
            match self{
                LinkedList::Nil => panic!("Index error!"),
                LinkedList::Node(_, tail) => tail.delete(index-1)
            }
        }
    }
}

use LinkedList::{Nil, Node};

fn main() {
    let mut lista = Nil;
    println!("Criando a lista...");
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);
    lista.display_list();

    println!("push_front(5)");
    lista.push_front(5);
    lista.display_list();

    lista.insert(2, 15);
    lista.display_list();

    lista.delete(3);
    lista.display_list();
}

//Testes 123