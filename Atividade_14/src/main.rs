use std::fmt::Debug;

// 1. Definindo o enum OrdemComparacao
#[derive(Debug, PartialEq)]
pub enum OrdemComparacao {
    Menor,
    Igual,
    Maior,
}

// 2. Implementando a função ordenacao_bolha_custom
fn ordenacao_bolha_custom<T, F>(vec_ref: &mut Vec<T>, comparador: F)
where F: Fn(&T, &T) -> OrdemComparacao
{
    let n = vec_ref.len();

    // Se o vetor tem 0 ou 1 elemento, já está ordenado
    if n <= 1 {
        return;
    }

    // Bubble Sort 
    for i in 0..n {
        let mut houve_troca = false;

        // O último i elementos já estão na posição correta
        for j in 0..(n - 1 - i) {
            // Compara elementos adjacentes usando a closure
            match comparador(&vec_ref[j], &vec_ref[j + 1]) {
                OrdemComparacao::Maior => {
                    // Se o elemento atual é maior que o próximo, troca
                    vec_ref.swap(j, j + 1);
                    houve_troca = true;
                }
                _ => {} // Não faz nada se for Menor ou Igual
            }
        }

        // Se não houve trocas nesta passagem, o vetor já está ordenado
        if !houve_troca {
            break;
        }
    }
}

// Estrutura para o teste com structs
#[derive(Debug, Clone)]
struct Item {
    valor: f32,
    nome: String,
}

fn main() {
    // --- Teste com Inteiros ---
    let mut numeros: Vec<i32> = vec![64, 34, -25, 12, 22, 11, -90, 0, 12];
    println!("Números Originais: {:?}", numeros);

    ordenacao_bolha_custom(&mut numeros, |a: &i32, b: &i32| {
        // Implementando comparação de i32 em ordem ascendente
        if a < b {
            OrdemComparacao::Menor
        } else if a > b {
            OrdemComparacao::Maior
        } else {
            OrdemComparacao::Igual
        }
    });

    println!("Números Ordenados: {:?}", numeros);
    println!("---");

    // --- Teste com Strings ---
    let mut palavras: Vec<String> = vec![
        "Rust".to_string(), "é".to_string(), "poderoso".to_string(), "e".to_string(),
        "seguro".to_string(), "!".to_string(), "é".to_string(), "".to_string(),
    ];
    println!("Palavras Originais: {:?}", palavras);

    ordenacao_bolha_custom(&mut palavras, |a: &String, b: &String| {
        // Implementando comparação lexicográfica de String em ordem ascendente
        use std::cmp::Ordering;
        match a.cmp(b) {
            Ordering::Less => OrdemComparacao::Menor,
            Ordering::Greater => OrdemComparacao::Maior,
            Ordering::Equal => OrdemComparacao::Igual,
        }
    });

    println!("Palavras Ordenadas: {:?}", palavras);
    println!("---");

    // --- Teste com Structs (Item) ---
    let mut itens: Vec<Item> = vec![
        Item { valor: 10.5, nome: "Banana".to_string() },
        Item { valor: 5.2, nome: "Maçã".to_string() },
        Item { valor: 5.2, nome: "Uva".to_string() },
        Item { valor: 10.5, nome: "Abacaxi".to_string() },
        Item { valor: 8.0, nome: "Laranja".to_string() },
    ];
    println!("Itens Originais: {:?}", itens);

    ordenacao_bolha_custom(&mut itens, |a: &Item, b: &Item| {
        // Implementando comparação multi-chave: primeiro por valor, depois por nome
        use std::cmp::Ordering;

        // Primeiro compara por valor
        match a.valor.partial_cmp(&b.valor) {
            Some(Ordering::Less) => OrdemComparacao::Menor,
            Some(Ordering::Greater) => OrdemComparacao::Maior,
            Some(Ordering::Equal) | None => {
                // Se valores são iguais (ou incomparáveis), compara por nome
                match a.nome.cmp(&b.nome) {
                    Ordering::Less => OrdemComparacao::Menor,
                    Ordering::Greater => OrdemComparacao::Maior,
                    Ordering::Equal => OrdemComparacao::Igual,
                }
            }
        }
    });

    println!("Itens Ordenados: {:?}", itens);
    println!("---");

    println!("\n Implementação completa! Todos os testes executados com sucesso.");

    // Demonstração adicional: ordenação decrescente de números
    let mut numeros_desc: Vec<i32> = vec![5, 2, 8, 1, 9];
    println!("\nBonus - Números para ordenação decrescente: {:?}", numeros_desc);

    ordenacao_bolha_custom(&mut numeros_desc, |a: &i32, b: &i32| {
        // Invertendo a lógica para ordem decrescente
        if a > b {
            OrdemComparacao::Menor
        } else if a < b {
            OrdemComparacao::Maior
        } else {
            OrdemComparacao::Igual
        }
    });

    println!("Números Ordenados (decrescente): {:?}", numeros_desc);
}