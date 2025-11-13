/*
 REFLEXÃO SOBRE BALANCEAMENTO DE CARGA

 1. Por que a estratégia de divisão linear de intervalos causa um
    desbalanceamento de carga entre as threads? Explique com suas
    próprias palavras.

    A divisão linear de intervalos causa desbalanceamento porque os números
    maiores requerem muito mais operações para verificar se são primos do que
    números menores. Os algoritmos de teste de primalidade precisam verificar
    divisibilidade por mais candidatos conforme o número cresce. Por exemplo,
    verificar se 100 é primo é muito mais rápido do que verificar se 1.900.000
    é primo. Isso significa que as threads que recebem os intervalos finais
    (com números maiores) vão trabalhar muito mais tempo, enquanto as threads
    com intervalos iniciais terminam rapidamente e ficam ociosas, desperdiçando
    recursos computacionais.

 2. Descreva em detalhes pelo menos UMA estratégia alternativa de
    distribuição de trabalho que resolveria ou melhoraria este
    problema de balanceamento. Round Robin

    Fila de Trabalho Dinâmica 

    Ao invés de dividir o intervalo total em blocos grandes e estáticos,
    poderíamos dividir o trabalho em pequenos "chunks" (lotes) e colocá-los
    em uma fila compartilhada protegida por um Mutex. Por exemplo, se o limite
    é 2.000.000, poderíamos criar chunks de 10.000 números cada, resultando
    em 200 tarefas menores.

    Como funcionaria:
    - A thread principal cria uma fila (Vec ou VecDeque) com todos os chunks
    - Esta fila é envolvida em Arc<Mutex<>> para ser compartilhada entre threads
    - Cada thread worker entra em um loop onde:
      1. Tenta obter o lock da fila
      2. Remove um chunk  da fila
      3. Libera o lock
      4. Processa o chunk localmente
      5. Envia os resultados pelo canal
      6. Repete até a fila estar vazia

    Vantagens desta abordagem:
    - Threads mais rápidas automaticamente pegam mais trabalho
    - Não há threads ociosas enquanto houver trabalho
    - O balanceamento acontece dinamicamente em tempo de execução
    - Threads que terminam um chunk difícil podem pegar um mais fácil depois

*/

use std::sync::mpsc;
use std::thread;
use std::io::{self, Write};

// Enum para mensagens que os workers enviam para a thread principal
#[derive(Debug)]
enum WorkerMessage {
    Progress(f32),              // Progresso percentual do worker
    FoundPrimes(Vec<u64>),      // Vetor com os primos encontrados
    Finished,                    // Sinaliza que o worker terminou
}

// Função executada por cada thread worker
fn worker_thread(
    worker_id: usize,
    start: u64,
    end: u64,
    tx: mpsc::Sender<WorkerMessage>,
) {
    let mut primes = Vec::new();
    let total_numbers = end - start + 1;
    let mut checked = 0u64;

    println!("Worker {} iniciado: verificando intervalo [{}, {}]", worker_id, start, end);

    // Verifica cada número no intervalo
    for num in start..=end {
        if primal::is_prime(num) {
            primes.push(num);
        }

        checked += 1;

        // Envia progresso a cada 1% do trabalho
        if checked % (total_numbers / 100).max(1) == 0 {
            let progress = (checked as f32 / total_numbers as f32) * 100.0;
            tx.send(WorkerMessage::Progress(progress)).unwrap();
        }
    }

    // Envia os primos encontrados e sinaliza término
    println!("Worker {} encontrou {} números primos", worker_id, primes.len());
    tx.send(WorkerMessage::FoundPrimes(primes)).unwrap();
    tx.send(WorkerMessage::Finished).unwrap();
}

fn main() {
    println!("=== Calculador de Números Primos Concorrente ===\n");

    // Solicita o limite superior
    print!("Digite o limite superior para busca de primos: ");
    io::stdout().flush().unwrap();
    let mut limit_input = String::new();
    io::stdin().read_line(&mut limit_input).unwrap();
    let limit: u64 = limit_input.trim().parse().expect("Por favor, digite um número válido");

    // Solicita o número de threads
    print!("Digite o número de threads workers: ");
    io::stdout().flush().unwrap();
    let mut threads_input = String::new();
    io::stdin().read_line(&mut threads_input).unwrap();
    let num_threads: usize = threads_input.trim().parse().expect("Por favor, digite um número válido");

    if limit < 2 {
        println!("O limite deve ser pelo menos 2");
        return;
    }

    if num_threads == 0 {
        println!("O número de threads deve ser pelo menos 1");
        return;
    }

    println!("\nIniciando busca de primos até {} usando {} threads...\n", limit, num_threads);

    // Cria o canal de comunicação
    let (tx, rx) = mpsc::channel();

    // Calcula o tamanho de cada intervalo (divisão linear)
    let range_size = (limit - 1) / num_threads as u64;
    let mut handles = Vec::new();

    // Inicia as threads workers
    for i in 0..num_threads {
        let start = if i == 0 { 2 } else { 2 + i as u64 * range_size };
        let end = if i == num_threads - 1 {
            limit
        } else {
            2 + (i as u64 + 1) * range_size - 1
        };

        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            worker_thread(i + 1, start, end, tx_clone);
        });

        handles.push(handle);
    }

    // Importante: dropa o transmissor original para que o canal feche quando todos os workers terminarem
    drop(tx);

    // Coleta os resultados
    let mut all_primes = Vec::new();
    let mut finished_workers = 0;
    let mut last_progress_print = std::time::Instant::now();

    // Processa as mensagens recebidas
    while let Ok(message) = rx.recv() {
        match message {
            WorkerMessage::Progress(_progress) => {
                // Atualiza display de progresso a cada 2 segundos
                if last_progress_print.elapsed().as_secs() >= 2 {
                    println!("Trabalhando... {} de {} workers finalizados", finished_workers, num_threads);
                    last_progress_print = std::time::Instant::now();
                }
            }
            WorkerMessage::FoundPrimes(primes) => {
                all_primes.extend(primes);
            }
            WorkerMessage::Finished => {
                finished_workers += 1;
                if finished_workers == num_threads {
                    break;
                }
            }
        }
    }

    // Aguarda todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    // Ordena os primos
    all_primes.sort_unstable();

    // Exibe os resultados
    println!("\n=== Resultados ===");
    println!("Total de números primos encontrados: {}", all_primes.len());

    // Mostra os primeiros e últimos primos se houver muitos
    if all_primes.len() <= 100 {
        println!("\nNúmeros primos encontrados:");
        for (i, prime) in all_primes.iter().enumerate() {
            print!("{:8}", prime);
            if (i + 1) % 10 == 0 {
                println!();
            }
        }
        println!();
    } else {
        println!("\nPrimeiros 20 primos:");
        for (i, prime) in all_primes.iter().take(20).enumerate() {
            print!("{:8}", prime);
            if (i + 1) % 10 == 0 {
                println!();
            }
        }
        println!("\n\nÚltimos 20 primos:");
        for (i, prime) in all_primes.iter().rev().take(20).rev().enumerate() {
            print!("{:8}", prime);
            if (i + 1) % 10 == 0 {
                println!();
            }
        }
        println!();
    }

    println!("\nBusca concluída com sucesso!");
}
