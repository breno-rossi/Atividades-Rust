use rand::Rng;
use std::collections::HashSet;
use std::io;

fn avaliacao_usuario(codigo: &str, chute: &str) -> (usize, usize) {
    let mut posicao_correta = 0;
    let mut letra_correta = 0;

    for (i, g_char) in chute.chars().enumerate() {
        if let Some(s_char) = codigo.chars().nth(i) {
            if g_char == s_char {
                posicao_correta += 1;
            } else if codigo.contains(g_char) {
                letra_correta += 1;
            }
        }
    }

    (posicao_correta, letra_correta)
}

fn get_user_input() -> String {
    let mut input = String::new(); // Inicializa variavel input
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    input.trim().to_uppercase()
}

fn generate_secret_code() -> String {
    let mut letters = String::from("ABCD");
    let mut secret_code = String::new();

    while !letters.is_empty() {
        let index = rand::thread_rng().gen_range(0..letters.len());
        secret_code.push(letters.remove(index));
    }

    secret_code
}

fn main() {
    println!("=== Jogo Mastermind ===");
    println!("Adivinhe o código secreto (A, B, C, D sem repetir).");

    let codigo_secreto = generate_secret_code();

    let mut tentativas: i32 = 0;

    loop {
        println!("\nDigite sua tentativa:");
        let chute = get_user_input();

        // validação 1: tamanho
        if chute.len() != 4 {
            println!("Digite exatamente 4 letras!");
            continue;
        }

        // validação 2: apenas letras A, B, C ou D
        if !chute.chars().all(|c| "ABCD".contains(c)) {
            println!(" Use apenas as letras A, B, C e D!");
            continue;
        }

        // validação 3: não pode repetir letras
        let caracter_unico: HashSet<char> = chute.chars().collect();
        if caracter_unico.len() != chute.len() {
            println!("⚠️ Não repita letras!");
            continue;
        }

        tentativas += 1;
        let (posicoes, letras) = avaliacao_usuario(&codigo_secreto, &chute);

        println!(
            "Posições corretas: {} | Letras corretas em posição errada: {}",
            posicoes, letras
        );

        if posicoes == 4 {
            println!(
                "\n Parabéns! Você acertou o código {} em {} tentativas!",
                codigo_secreto, tentativas
            );
            break;
        }
    }
}
