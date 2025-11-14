#[derive(Debug, PartialEq)] // Adicionado PartialEq para facilitar a asserção
struct Produto {
    id: u32,
    nome: String,
    preco_centavos: u32,
    em_estoque: bool,
}

fn main() {
    let catalogo = vec![
        Produto { id: 1, nome: "Teclado Mecânico".to_string(), preco_centavos: 25000, em_estoque: true },
        Produto { id: 2, nome: "Mouse Gamer".to_string(), preco_centavos: 12050, em_estoque: true },
        Produto { id: 3, nome: "Monitor 4K".to_string(), preco_centavos: 185000, em_estoque: false },
        Produto { id: 4, nome: "Webcam HD".to_string(), preco_centavos: 9990, em_estoque: true },
        Produto { id: 5, nome: "SSD 1TB".to_string(), preco_centavos: 45000, em_estoque: false },
        Produto { id: 6, nome: "Cadeira Gamer".to_string(), preco_centavos: 95000, em_estoque: true },
    ];

    // Tarefa 1: Encontrar o produto mais barato em estoque
    let mais_barato = catalogo
        .iter()
        .filter(|p| p.em_estoque)
        .min_by_key(|p| p.preco_centavos);

    println!("Produto mais barato em estoque: {:?}", mais_barato);

    // Teste para garantir que o produto correto foi encontrado
    assert_eq!(mais_barato.unwrap().id, 4);

    // Tarefa 2: Gerar lista de nomes de produtos em estoque
    let nomes_em_estoque: Vec<String> = catalogo
        .iter()
        .filter(|p| p.em_estoque)
        .map(|p| p.nome.clone())
        .collect();

    println!("\nProdutos em estoque: {:?}", nomes_em_estoque);

    // Tarefa 3: Calcular o valor total do estoque em centavos
    let valor_total_centavos: u32 = catalogo
        .iter()
        .filter(|p| p.em_estoque)
        .map(|p| p.preco_centavos)
        .sum();

    println!("\nValor total do estoque: R$ {:.2}", valor_total_centavos as f32 / 100.0);
}
