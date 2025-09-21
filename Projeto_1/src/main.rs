use std::io;
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json::{to_writer, from_reader};

/// Estrutura que representa as informações de um animal
///
/// # Campos
/// - `tipo`: O tipo do animal (ex: Elefante, Leão, etc.)
/// - `nome`: O nome específico do animal
#[derive(Serialize, Deserialize, Clone)]
struct InfoAnimal {
    tipo: String,
    nome: String,
}

impl InfoAnimal {
    /// Cria uma nova instância de InfoAnimal
    ///
    /// #Parâmetros
    /// - `tipo`: Tipo do animal
    /// - `nome`: Nome do animal
    ///
    /// #Retorno
    /// Nova instância de InfoAnimal
    fn new(tipo: String, nome: String) -> InfoAnimal {
        InfoAnimal { tipo, nome }
    }
}

/// Função auxiliar para ler entrada do usuário
///
/// #Retorno
/// String com a entrada do usuário (sem quebras de linha)
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().to_string()
}

/// Salva o vetor de animais em um arquivo JSON
///
/// # Parâmetros
/// - `animais`: Referência ao vetor de animais para salvar
///
/// Esta função serializa todo o vetor de InfoAnimal para JSON
/// e salva no arquivo "animais.json"
fn salvar_animais(animais: &Vec<InfoAnimal>) {
    match File::create("animais.json") {
        Ok(file) => {
            if let Err(e) = to_writer(file, animais) {
                eprintln!("Erro ao salvar dados: {}", e);
            } else {
                println!("Dados salvos com sucesso!");
            }
        }
        Err(e) => eprintln!("Erro ao criar arquivo: {}", e),
    }
}

/// Carrega o vetor de animais de um arquivo JSON
///
/// # Retorno
/// Vetor de InfoAnimal carregado do arquivo, ou vetor vazio se houver erro
///
/// Esta função verifica se o arquivo existe antes de tentar carregá-lo
fn carregar_animais() -> Vec<InfoAnimal> {
    match File::open("animais.json") {
        Ok(file) => {
            match from_reader(file) {
                Ok(animais) => {
                    println!("Dados carregados com sucesso!");
                    animais
                }
                Err(e) => {
                    eprintln!("Erro ao ler arquivo JSON: {}", e);
                    Vec::new()
                }
            }
        }
        Err(_) => {
            println!("Arquivo não encontrado. Iniciando com base vazia.");
            Vec::new()
        }
    }
}

/// Exibe todos os animais cadastrados na base de dados
///
/// #Parâmetros
/// - `animais`: Referência ao vetor de animais para listar
///
/// Mostra o índice, tipo e nome de cada animal cadastrado
fn listar_animais(animais: &Vec<InfoAnimal>) {
    println!("\n  === ANIMAIS CADASTRADOS ===");

    if animais.is_empty() {
        println!("   Nenhum animal cadastrado no sistema.");
        println!("   Use a opção 2 para incluir o primeiro animal!");
        return;
    }

    println!("   Lista completa dos animais:");
    println!("   ÍNDICE | TIPO (Espécie) | NOME");
    println!("   -------|----------------|----------");

    for (index, animal) in animais.iter().enumerate() {
        println!("   {}      | {}           | {}",
                 index,
                 animal.tipo,
                 animal.nome);
    }

    println!();
    println!("  Total de animais cadastrados: {}", animais.len());
}

/// Inclui um novo animal na base de dados
///
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais
///
/// Solicita tipo e nome do usuário, cria novo InfoAnimal e salva no arquivo
fn incluir_animal(animais: &mut Vec<InfoAnimal>) {
    println!("\n=== INCLUIR NOVO ANIMAL ===");
    println!("Para cadastrar um animal, você precisa informar:");
    println!("- TIPO: A espécie do animal");
    println!("- NOME: O nome específico do animal)");
    println!();

    print!("Digite o TIPO do animal (espécie): ");
    let tipo = read_input();

    if tipo.is_empty() {
        println!("  Erro: O tipo do animal não pode estar vazio!");
        return;
    }

    print!("Digite o NOME do animal: ");
    let nome = read_input();

    if nome.is_empty() {
        println!("  Erro: O nome do animal não pode estar vazio!");
        return;
    }

    let novo_animal = InfoAnimal::new(tipo.clone(), nome.clone());
    animais.push(novo_animal);
    salvar_animais(animais);

    println!(" Animal incluído com sucesso!");
    println!("   Tipo: {}", tipo);
    println!("   Nome: {}", nome);
}

/// Edita um animal existente na base de dados
///
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais
///
/// Solicita o índice do animal a ser editado e os novos dados
fn editar_animal(animais: &mut Vec<InfoAnimal>) {
    println!("\n=== EDITAR ANIMAL ===");

    if animais.is_empty() {
        println!("  Nenhum animal cadastrado para editar.");
        return;
    }

    listar_animais(animais);

    print!("Digite o ÍNDICE (número) do animal a ser editado: ");
    let input = read_input();

    let index: usize = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("  Erro: Por favor, insira um número válido.");
            return;
        }
    };

    if index >= animais.len() {
        println!("  Erro: Índice inválido! Escolha um número entre 0 e {}", animais.len() - 1);
        return;
    }

    println!("\n Animal atual:");
    println!("   Tipo: {}", animais[index].tipo);
    println!("   Nome: {}", animais[index].nome);
    println!();

    print!("Digite o novo TIPO do animal (espécie): ");
    let novo_tipo = read_input();

    if novo_tipo.is_empty() {
        println!("  Erro: O tipo do animal não pode estar vazio!");
        return;
    }

    print!("Digite o novo NOME do animal: ");
    let novo_nome = read_input();

    if novo_nome.is_empty() {
        println!("  Erro: O nome do animal não pode estar vazio!");
        return;
    }

    // Atualiza os dados do animal
    animais[index].tipo = novo_tipo.clone();
    animais[index].nome = novo_nome.clone();

    salvar_animais(animais);
    println!("   Animal editado com sucesso!");
    println!("   Novo tipo: {}", novo_tipo);
    println!("   Novo nome: {}", novo_nome);
}

/// Exclui um animal da base de dados
///
/// # Parâmetros
/// - `animais`: Referência mutável ao vetor de animais
///
/// Solicita o índice do animal a ser excluído e remove do vetor
fn excluir_animal(animais: &mut Vec<InfoAnimal>) {
    println!("\n=== EXCLUIR ANIMAL ===");

    if animais.is_empty() {
        println!("Nenhum animal cadastrado para excluir.");
        return;
    }

    listar_animais(animais);

    print!("Digite o ÍNDICE (número) do animal a ser excluído: ");
    let input = read_input();

    let index: usize = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Erro: Por favor, insira um número válido.");
            return;
        }
    };

    if index >= animais.len() {
        println!("Erro: Índice inválido! Escolha um número entre 0 e {}", animais.len() - 1);
        return;
    }

    let animal_removido = animais.remove(index);
    salvar_animais(animais);

    println!("  Animal excluído com sucesso!");
    println!("   Animal removido: {} ({})",
             animal_removido.nome, animal_removido.tipo);
}

/// Exibe o menu principal do sistema
///
/// Mostra todas as opções disponíveis para o usuário
fn exibir_menu() {
    println!("\n === SISTEMA DE GERENCIAMENTO DE ANIMAIS === ");
    println!("Escolha uma das opções abaixo:");
    println!("1. Listar animais cadastrados");
    println!("2. Incluir novo animal");
    println!("3. Editar animal existente");
    println!("4. Excluir animal");
    println!("5. Sair do sistema");
    println!();
    print!("Digite o número da opção desejada (1-5): ");
}

/// Função principal do programa
///
/// Inicializa o sistema, carrega os dados e executa o loop principal do menu
fn main() {
    println!("Bem-vindo ao Sistema de Gerenciamento de Animais!");

    // Carrega os dados do arquivo JSON no início do programa
    let mut animais = carregar_animais();

    // Loop principal do programa
    loop {
        exibir_menu();
        let opcao = read_input();

        match opcao.as_str() {
            "1" => listar_animais(&animais),
            "2" => incluir_animal(&mut animais),
            "3" => editar_animal(&mut animais),
            "4" => excluir_animal(&mut animais),
            "5" => {
                println!("Saindo do sistema. Obrigado por usar o Sistema de Gerenciamento de Animais!");
                break;
            }
            _ => println!("Opção inválida! Por favor, escolha uma opção de 1 a 5."),
        }


        println!("\nPressione Enter para continuar...");
        read_input();
    }
}