fn main() {
    let pontuacoes = vec![100, -5, 92, 0, 34, -12, 88, 55, 10, -20, 99];

    // Tarefa 1: Contar pontuações válidas (> 0)
    let quantidade_validas = pontuacoes
        .iter()
        .filter(|&&p| p > 0)
        .count();
    println!("Quantidade de pontuações válidas: {}", quantidade_validas);

    // Tarefa 2: Coletar pontuações válidas com bônus de 10 pontos
    let pontuacoes_com_bonus: Vec<u32> = pontuacoes
        .iter()
        .filter(|&&p| p > 0)
        .map(|&p| (p + 10) as u32)
        .collect();
    println!("Pontuações com bônus: {:?}", pontuacoes_com_bonus);

    // Tarefa 3: Somar pontuações pares e válidas
    let soma_pares_validas: i32 = pontuacoes
        .iter()
        .filter(|&&p| p > 0 && p % 2 == 0)
        .sum();
    println!("Soma das pontuações pares e válidas: {}", soma_pares_validas);
}
