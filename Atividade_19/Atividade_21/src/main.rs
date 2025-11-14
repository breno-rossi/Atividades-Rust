use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::Arc; // Importação necessária
use std::thread;

// Função auxiliar para ler o arquivo. (Não precisa mexer)
fn carregar_palavras(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() {
    // --- 1. Carregar Dados ---
    // (Não precisa mexer)
    let palavras = match carregar_palavras("palavras.txt") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro ao ler 'palavras.txt': {}", e);
            eprintln!("Certifique-se de que o arquivo existe na raiz do projeto (junto ao Cargo.toml).");
            return;
        }
    };

    if palavras.is_empty() {
        println!("Arquivo 'palavras.txt' está vazio.");
        return;
    }

    // TODO: Atividade 1
    // O 'Vec' 'palavras' precisa ser compartilhado entre threads.
    // Encapsule-o em um Arc.
    // Substitua '???' pela chamada correta para criar um Arc.
    let dados_compartilhados: Arc<Vec<String>> = Arc::new(palavras); // Ex: Arc::new(palavras)

    // --- 2. Preparar Threads ---
    // (Não precisa mexer)
    const NUM_THREADS: usize = 4;
    let data_len = dados_compartilhados.len();
    // Garante que a divisão de 'chunks' funcione mesmo se não for divisível por NUM_THREADS
    let chunk_size = (data_len + NUM_THREADS - 1) / NUM_THREADS;

    let mut handles = vec![];

    println!("Iniciando {} threads para processar {} palavras...", NUM_THREADS, data_len);

    // --- 3. Iniciar Threads ---
    for i in 0..NUM_THREADS {
        // (Não precisa mexer)
        let chunk_start = i * chunk_size;
        let chunk_end = ((i + 1) * chunk_size).min(data_len);

        // TODO: Atividade 2
        // Precisamos dar a esta nova thread acesso aos dados.
        // Clone o Arc 'dados_compartilhados'. Clonar um Arc é barato!
        // Substitua '???' pela chamada correta.
        let dados_clone: Arc<Vec<String>> = Arc::clone(&dados_compartilhados); // Ex: Arc::clone(&dados_compartilhados)

        // (Não precisa mexer)
        let handle = thread::spawn(move || {
            let mut palavra_local_mais_longa = "";

            // Evita pânico se a fatia estiver vazia
            if chunk_start < chunk_end {
                // Itera *apenas* sobre a fatia desta thread
                for j in chunk_start..chunk_end {

                    // TODO: Atividade 3
                    // Acesse a palavra no índice 'j' dentro do 'dados_clone'.
                    // Lembre-se que 'dados_clone' é um Arc<Vec<String>>.
                    // Substitua '???' pela expressão correta.
                    let palavra_atual = &dados_clone[j]; // Ex: dados_clone[j]

                    if palavra_atual.chars().count() > palavra_local_mais_longa.chars().count() {
                        palavra_local_mais_longa = palavra_atual;
                    }
                }
            }
            // Retorna o resultado local da thread
            palavra_local_mais_longa.to_string()
        });
        handles.push(handle);
    }

    // --- 4. Coletar Resultados ---
    // (Não precisa mexer)
    let mut resultados_locais = vec![];
    for handle in handles {
        match handle.join() {
            Ok(resultado) => resultados_locais.push(resultado),
            Err(_) => eprintln!("Uma thread deu panic!"),
        }
    }

    // --- 5. Calcular Resultado Final ---
    // (Não precisa mexer)
    let mut palavra_global_mais_longa = "";
    for local in &resultados_locais {
        if local.chars().count() > palavra_global_mais_longa.chars().count() {
            palavra_global_mais_longa = local;
        }
    }

    println!("\n--- Resultados ---");
    println!("Resultados locais de cada thread: {:?}", resultados_locais);
    println!("A palavra mais longa encontrada foi: '{}'", palavra_global_mais_longa);
}