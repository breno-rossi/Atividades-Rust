// ------------------------------------------------------------------
// TODO: Atividade (Global)
// 1. Adicione 'Mutex' E 'RwLock' a esta linha de 'use'.
// ------------------------------------------------------------------
use std::sync::{Arc, Mutex, RwLock}; // <-- TODO 1
use std::thread;
use std::time::{Duration, Instant};

// --- PARÂMETROS DA SIMULAÇÃO (Globais) ---
const NUM_READERS: u32 = 10;
const NUM_WRITERS: u32 = 2;
const READER_WORK_MS: u64 = 200;
const WRITER_WORK_MS: u64 = 100;


// ==================================================================
// TESTE 1: IMPLEMENTAÇÃO COM MUTEX
// ==================================================================
fn run_mutex_test() {
    println!("\nIniciando teste com MUTEX...");

    // ------------------------------------------------------------------
    // TODO: Atividade (Mutex)
    // 2. Crie o 'data' usando: Arc::new(Mutex::new(Vec::<u32>::new()))
    // ------------------------------------------------------------------
    let data: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::<u32>::new())); // <-- TODO 2

    let mut handles = vec![];
    let start = Instant::now();

    // --- Inicia as Threads Escritoras (Mutex) ---
    for i in 0..NUM_WRITERS {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (Mutex)
            // 3. Obtenha o lock com: data_clone.lock().unwrap()
            // ------------------------------------------------------------------
            let mut guard = data_clone.lock().unwrap(); // <-- TODO 3

            thread::sleep(Duration::from_millis(WRITER_WORK_MS));
            guard.push(i);
        });
        handles.push(handle);
    }

    // --- Inicia as Threads Leitoras (Mutex) ---
    for _ in 0..NUM_READERS {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (Mutex)
            // 4. Obtenha o lock com: data_clone.lock().unwrap()
            // ------------------------------------------------------------------
            let guard = data_clone.lock().unwrap(); // <-- TODO 4

            thread::sleep(Duration::from_millis(READER_WORK_MS));
            let _len = guard.len();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("[Mutex] Tempo total: {:?}", duration);

    // ------------------------------------------------------------------
    // TODO: Atividade (Mutex)
    // 5. Leia o resultado com: data.lock().unwrap().len()
    // ------------------------------------------------------------------
    let final_len = data.lock().unwrap().len(); // <-- TODO 5

    println!("[Mutex] Valor final dos dados (len): {}", final_len);
}


// ==================================================================
// TESTE 2: IMPLEMENTAÇÃO COM RWLOCK
// ==================================================================
fn run_rwlock_test() {
    println!("\nIniciando teste com RWLOCK...");

    // ------------------------------------------------------------------
    // TODO: Atividade (RwLock)
    // 6. Crie o 'data' usando: Arc::new(RwLock::new(Vec::<u32>::new()))
    // ------------------------------------------------------------------
    let data: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(Vec::<u32>::new())); // <-- TODO 6

    let mut handles = vec![];
    let start = Instant::now();

    // --- Inicia as Threads Escritoras (RwLock) ---
    for i in 0..NUM_WRITERS {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (RwLock)
            // 7. Obtenha o lock de *escrita* com: data_clone.write().unwrap()
            // ------------------------------------------------------------------
            let mut guard = data_clone.write().unwrap(); // <-- TODO 7

            thread::sleep(Duration::from_millis(WRITER_WORK_MS));
            guard.push(i);
        });
        handles.push(handle);
    }

    // --- Inicia as Threads Leitoras (RwLock) ---
    for _ in 0..NUM_READERS {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // ------------------------------------------------------------------
            // TODO: Atividade (RwLock)
            // 8. Obtenha o lock de *leitura* com: data_clone.read().unwrap()
            // ------------------------------------------------------------------
            let guard = data_clone.read().unwrap(); // <-- TODO 8

            thread::sleep(Duration::from_millis(READER_WORK_MS));
            let _len = guard.len();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("[RwLock] Tempo total: {:?}", duration);

    // ------------------------------------------------------------------
    // TODO: Atividade (RwLock)
    // 9. Leia o resultado com: data.read().unwrap().len()
    // ------------------------------------------------------------------
    let final_len = data.read().unwrap().len(); // <-- TODO 9

    println!("[RwLock] Valor final dos dados (len): {}", final_len);
}


/// Executa os dois benchmarks para comparação
fn main() {
    println!("--- Comparação de Desempenho Mutex vs. RwLock ---");
    println!("Parâmetros: {} Leitores ({}ms) | {} Escritores ({}ms)\n",
             NUM_READERS, READER_WORK_MS, NUM_WRITERS, WRITER_WORK_MS);

    // 1. Preencha os TODOs na função run_mutex_test
    run_mutex_test();

    // Pequena pausa para garantir que a saída do console não se misture
    thread::sleep(Duration::from_secs(1));

    // 2. Preencha os TODOs na função run_rwlock_test
    run_rwlock_test();

    println!("\n--- FIM DA COMPARAÇÃO ---");
    println!("Observe a diferença no 'Tempo total' entre os dois testes.");
}