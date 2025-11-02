use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Um funcionário pertence a um departamento.
// A referência ao departamento agora é FRACA para quebrar o ciclo!
struct Funcionario {
    nome: String,
    departamento: Weak<Departamento>,
}

impl Funcionario {
    // Método para acessar o departamento através da referência fraca
    fn exibir_departamento(&self) {
        if let Some(depto_forte) = self.departamento.upgrade() {
            println!("{} trabalha no departamento: {}", self.nome, depto_forte.nome);
        } else {
            println!("{}: Meu departamento já foi dissolvido.", self.nome);
        }
    }
}

impl Drop for Funcionario {
    fn drop(&mut self) {
        println!("Notificação de desligamento para {}.", self.nome);
    }
}

// Um departamento tem uma lista de seus funcionários (membros).
struct Departamento {
    nome: String,
    membros: RefCell<Vec<Rc<Funcionario>>>,
}

impl Drop for Departamento {
    fn drop(&mut self) {
        println!("Dissolvendo o departamento: {}!", self.nome);
    }
}

fn main() {
    println!("--- Início da Simulação ---");

    let depto_inovacao = Rc::new(Departamento {
        nome: "Inovação".to_string(),
        membros: RefCell::new(vec![]),
    });

    let alice = Rc::new(Funcionario {
        nome: "Alice".to_string(),
        departamento: Rc::downgrade(&depto_inovacao),
    });

    let bob = Rc::new(Funcionario {
        nome: "Bob".to_string(),
        departamento: Rc::downgrade(&depto_inovacao),
    });

    depto_inovacao.membros.borrow_mut().push(Rc::clone(&alice));
    depto_inovacao.membros.borrow_mut().push(Rc::clone(&bob));

    println!("Contagem de referências fortes para o depto: {}", Rc::strong_count(&depto_inovacao));
    println!("Contagem de referências fracas para o depto: {}", Rc::weak_count(&depto_inovacao));

    // Demonstrando acesso ao departamento através da referência fraca
    alice.exibir_departamento();
    bob.exibir_departamento();

    println!("--- Fim da Simulação ---");
    // Agora, quando as variáveis saem de escopo, a memória é corretamente liberada!
}
