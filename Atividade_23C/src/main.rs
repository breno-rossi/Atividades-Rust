// Tarefa 1: Defina a struct
struct Intervalo {
    atual: i32,
    fim: i32,
    passo: i32,
}

impl Intervalo {
    // Uma funcao "construtora" para facilitar a criacao
    fn new(inicio: i32, fim: i32, passo: i32) -> Self {
        Intervalo {
            atual: inicio,
            fim,
            passo,
        }
    }
}

// Tarefa 2: Implemente a trait `Iterator`
impl Iterator for Intervalo {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // Verifica se `atual` ultrapassou `fim`
        if self.atual > self.fim {
            return None;
        }

        // Guarda o valor atual
        let valor_atual = self.atual;

        // Avanca `atual` pelo `passo`
        self.atual += self.passo;

        // Retorna o valor guardado
        Some(valor_atual)
    }
}

fn main() {
    println!("Iterando de 0 a 10 com passo 2:");
    // Tarefa 3: Teste com um loop `for`
    for i in Intervalo::new(0, 10, 2) {
        println!("{}", i);
    }

    // Tarefa 3: Teste com um adaptador de iterator
    // A sequencia e 1, 4, 7, 10, 13, 16, 19
    let soma: i32 = Intervalo::new(1, 20, 3).sum();
    println!("\nA soma do intervalo (1 a 20, passo 3) e: {}", soma);
    assert_eq!(soma, 70); // Teste automatico para garantir o resultado
}
