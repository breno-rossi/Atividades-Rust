use std::io;
use std::fs;
use rand::seq::SliceRandom;
use serde::Deserialize;

fn carregar_palavras_do_arquivo(caminho_arquivo: &str) -> Vec<String> {
    let conteudo = fs::read_to_string(caminho_arquivo)
        .expect("Erro ao ler o arquivo de palavras.");

    let palavras: Vec<String> = serde_json::from_str(&conteudo)
        .expect("Erro ao parsear o JSON.");

    palavras
}
fn get_user_input() -> String {
    let mut input = String::new(); // Inicializa variavel input
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    input.trim().to_uppercase()
}

fn selecionar_palavra(palavra: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    palavra.choose(&mut rng).unwrap().clone()
}

fn exibir_estado(palavra_exibida: &Vec<char>, letras_erradas: & Vec<char>, chutes_restantes: i16){

    println!("Palavra: ");
    for ch in palavra_exibida {
        println!("{}", ch);
    }
    println!();

    if !letras_erradas.is_empty() {
        println!("Letras Erradas: [");
        for (i, ch) in letras_erradas.iter().enumerate() {
            if i> 0 {
                println!(",");
            }
            println!("{}", ch);
        }
            println!("]");
    }else {
        println!("Letras Erradas: []");
    }
    println!("Tentativas restantes: {}", chutes_restantes);
    println!("{}", "-".repeat(40));

}

fn processar_tentativa(letra: char, palavra_secreta: &Vec<char>, palavra_exibida: &mut Vec<char>, letras_erradas: &mut Vec<char>) -> bool {
    let mut acertou = false;

    for (i, &ch) in palavra_secreta.iter().enumerate() {
        if ch == letra {
            palavra_exibida[i] = letra;
            acertou = true;
        }
    }

    if !acertou {
        letras_erradas.push(letra);
    }

    acertou
}

fn palavra_completa(palavra_exibida: &Vec<char>) -> bool{
    !palavra_exibida.contains(&'_')
}

fn letra_ja_tentada(letra: char, palavra_exibida: &Vec<char>, letras_erradas: &Vec<char>) -> bool {
    palavra_exibida.contains(&letra) || letras_erradas.contains(&letra)
}



fn main() {

    println!("BEM-VINDO AO JOGO DA FORCA!");
    println!("Tente adivinhar a palavra letra por letra.");
    println!("Você tem 6 tentativas erradas antes de perder!");

    // Carrega as palavras do arquivo
    let palavras = carregar_palavras_do_arquivo("banco_palavras.json");

    // Seleciona uma palavra aleatória
    let palavra_escolhida = selecionar_palavra(&palavras);
    let palavra_secreta: Vec<char> = palavra_escolhida.chars().collect();

    let mut palavra_exibida: Vec<char> = vec!['_'; palavra_secreta.len()];
    let mut letras_erradas: Vec<char> = Vec::new();
    let mut tentativas_restantes = 6;

    //Loop principal

    loop{
        exibir_estado(&palavra_exibida, &letras_erradas, tentativas_restantes);

        if palavra_completa(&palavra_exibida) {
            println!("PARABÉNS VOCÊ GANHOU!!! ");
            println!("A palavra era: {}", palavra_escolhida);
            break;
        }

        if tentativas_restantes <= 0 {
            println!("VOCÊ PERDEU :( ");
            println!("A palavra era: {}", palavra_escolhida);
            break
        }

        println!("\nDigite uma letra: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let input = get_user_input();

        if input.len() != 1 {
            println!("Digite apenas uma letra!");
            continue;
        }

        let letra = input.chars().next().unwrap();

        if !letra.is_alphabetic() {
            println!("Por favor, digite apenas letras.");
        }

        if letra_ja_tentada(letra,&palavra_exibida,&letras_erradas) {
            println!("Você ja tentou essa letra.");
            continue;
        }

        let acertou = processar_tentativa(letra,&palavra_secreta, &mut palavra_exibida, &mut letras_erradas);
        if acertou {
            println!("Acertou:");
        }else{
            tentativas_restantes -= 1;
            println!("Errou!");
        }

    }

}
