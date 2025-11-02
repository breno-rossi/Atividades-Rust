use biblioteca_virtual::biblioteca::Biblioteca;
use std::io::{self, Write};
use std::path::PathBuf;
use uuid::Uuid;

fn main() {
    let caminho = PathBuf::from("biblioteca_dados.json");
    let mut biblioteca = Biblioteca::carregar(&caminho).unwrap_or_else(|e| {
        println!("Aviso ao carregar: {}. Criando nova biblioteca.", e);
        Biblioteca::new(caminho.clone())
    });

    println!("=== Sistema de Gerenciamento de Biblioteca Virtual ===\n");

    loop {
        exibir_menu();

        let opcao = ler_entrada("Escolha uma opção: ");

        match opcao.trim() {
            "1" => adicionar_livro(&mut biblioteca),
            "2" => listar_livros(&biblioteca),
            "3" => buscar_livro(&biblioteca),
            "4" => remover_livro(&mut biblioteca),
            "5" => adicionar_usuario(&mut biblioteca),
            "6" => listar_usuarios(&biblioteca),
            "7" => emprestar_livro(&mut biblioteca),
            "8" => devolver_livro(&mut biblioteca),
            "9" => listar_emprestimos(&biblioteca),
            "10" => {
                salvar_biblioteca(&biblioteca);
                println!("\nEncerrando sistema...");
                break;
            }
            _ => println!("\nOpção inválida! Tente novamente.\n"),
        }
    }
}

fn exibir_menu() {
    println!("--- MENU ---");
    println!("1.  Adicionar Livro");
    println!("2.  Listar Livros");
    println!("3.  Buscar Livro");
    println!("4.  Remover Livro");
    println!("5.  Adicionar Usuário");
    println!("6.  Listar Usuários");
    println!("7.  Emprestar Livro");
    println!("8.  Devolver Livro");
    println!("9.  Listar Empréstimos Ativos");
    println!("10. Salvar e Sair");
    println!();
}

fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada
}

fn adicionar_livro(biblioteca: &mut Biblioteca) {
    println!("\n--- Adicionar Livro ---");
    let titulo = ler_entrada("Título: ").trim().to_string();
    let autor = ler_entrada("Autor: ").trim().to_string();
    let ano_str = ler_entrada("Ano de publicação: ").trim().to_string();

    match ano_str.parse::<u16>() {
        Ok(ano) => match biblioteca.adicionar_livro(titulo, autor, ano) {
            Ok(id) => {
                println!("\nLivro adicionado com sucesso! ID: {}", id);
                salvar_biblioteca(biblioteca);
            }
            Err(e) => println!("\nErro ao adicionar livro: {}", e),
        },
        Err(_) => println!("\nAno inválido!"),
    }
    println!();
}

fn listar_livros(biblioteca: &Biblioteca) {
    println!("\n--- Listar Livros ---");
    println!("1. Todos os livros");
    println!("2. Livros disponíveis");
    println!("3. Livros emprestados");

    let opcao = ler_entrada("Escolha uma opção: ");

    let livros = match opcao.trim() {
        "1" => biblioteca.listar_todos_livros(),
        "2" => biblioteca.listar_livros_disponiveis(),
        "3" => biblioteca.listar_livros_emprestados(),
        _ => {
            println!("\nOpção inválida!");
            return;
        }
    };

    if livros.is_empty() {
        println!("\nNenhum livro encontrado.");
    } else {
        println!("\nLivros encontrados:");
        for livro in livros {
            println!(
                "- ID: {}\n  Título: {}\n  Autor: {}\n  Ano: {}\n  Status: {:?}\n",
                livro.id, livro.titulo, livro.autor, livro.ano, livro.status
            );
        }
    }
    println!();
}

fn buscar_livro(biblioteca: &Biblioteca) {
    println!("\n--- Buscar Livro ---");
    println!("1. Por ID");
    println!("2. Por Título");
    println!("3. Por Autor");

    let opcao = ler_entrada("Escolha uma opção: ");

    match opcao.trim() {
        "1" => {
            let id_str = ler_entrada("Digite o ID do livro: ").trim().to_string();
            match Uuid::parse_str(&id_str) {
                Ok(id) => {
                    if let Some(livro) = biblioteca.buscar_livro_por_id(id) {
                        println!(
                            "\nLivro encontrado:\n- ID: {}\n  Título: {}\n  Autor: {}\n  Ano: {}\n  Status: {:?}\n",
                            livro.id, livro.titulo, livro.autor, livro.ano, livro.status
                        );
                    } else {
                        println!("\nLivro não encontrado.");
                    }
                }
                Err(_) => println!("\nID inválido!"),
            }
        }
        "2" => {
            let titulo = ler_entrada("Digite o título (ou parte dele): ")
                .trim()
                .to_string();
            let livros = biblioteca.buscar_livros_por_titulo(&titulo);
            if livros.is_empty() {
                println!("\nNenhum livro encontrado.");
            } else {
                println!("\nLivros encontrados:");
                for livro in livros {
                    println!(
                        "- ID: {}\n  Título: {}\n  Autor: {}\n  Ano: {}\n  Status: {:?}\n",
                        livro.id, livro.titulo, livro.autor, livro.ano, livro.status
                    );
                }
            }
        }
        "3" => {
            let autor = ler_entrada("Digite o autor (ou parte dele): ")
                .trim()
                .to_string();
            let livros = biblioteca.buscar_livros_por_autor(&autor);
            if livros.is_empty() {
                println!("\nNenhum livro encontrado.");
            } else {
                println!("\nLivros encontrados:");
                for livro in livros {
                    println!(
                        "- ID: {}\n  Título: {}\n  Autor: {}\n  Ano: {}\n  Status: {:?}\n",
                        livro.id, livro.titulo, livro.autor, livro.ano, livro.status
                    );
                }
            }
        }
        _ => println!("\nOpção inválida!"),
    }
    println!();
}

fn remover_livro(biblioteca: &mut Biblioteca) {
    println!("\n--- Remover Livro ---");
    let id_str = ler_entrada("Digite o ID do livro: ").trim().to_string();

    match Uuid::parse_str(&id_str) {
        Ok(id) => match biblioteca.remover_livro(id) {
            Ok(_) => {
                println!("\nLivro removido com sucesso!");
                salvar_biblioteca(biblioteca);
            }
            Err(e) => println!("\nErro ao remover livro: {}", e),
        },
        Err(_) => println!("\nID inválido!"),
    }
    println!();
}

fn adicionar_usuario(biblioteca: &mut Biblioteca) {
    println!("\n--- Adicionar Usuário ---");
    let nome = ler_entrada("Nome: ").trim().to_string();

    match biblioteca.adicionar_usuario(nome) {
        Ok(id) => {
            println!("\nUsuário adicionado com sucesso! ID: {}", id);
            salvar_biblioteca(biblioteca);
        }
        Err(e) => println!("\nErro ao adicionar usuário: {}", e),
    }
    println!();
}

fn listar_usuarios(biblioteca: &Biblioteca) {
    println!("\n--- Listar Usuários ---");
    let usuarios = biblioteca.listar_usuarios();

    if usuarios.is_empty() {
        println!("\nNenhum usuário cadastrado.");
    } else {
        println!("\nUsuários cadastrados:");
        for usuario in usuarios {
            println!("- ID: {}\n  Nome: {}\n", usuario.id, usuario.nome);
        }
    }
    println!();
}

fn emprestar_livro(biblioteca: &mut Biblioteca) {
    println!("\n--- Emprestar Livro ---");
    let id_usuario_str = ler_entrada("ID do usuário: ").trim().to_string();
    let id_livro_str = ler_entrada("ID do livro: ").trim().to_string();

    let id_usuario = match Uuid::parse_str(&id_usuario_str) {
        Ok(id) => id,
        Err(_) => {
            println!("\nID de usuário inválido!");
            return;
        }
    };

    let id_livro = match Uuid::parse_str(&id_livro_str) {
        Ok(id) => id,
        Err(_) => {
            println!("\nID de livro inválido!");
            return;
        }
    };

    match biblioteca.emprestar_livro(id_usuario, id_livro) {
        Ok(id_emprestimo) => {
            println!(
                "\nLivro emprestado com sucesso! ID do empréstimo: {}",
                id_emprestimo
            );
            salvar_biblioteca(biblioteca);
        }
        Err(e) => println!("\nErro ao emprestar livro: {}", e),
    }
    println!();
}

fn devolver_livro(biblioteca: &mut Biblioteca) {
    println!("\n--- Devolver Livro ---");
    let id_livro_str = ler_entrada("ID do livro: ").trim().to_string();

    match Uuid::parse_str(&id_livro_str) {
        Ok(id_livro) => match biblioteca.devolver_livro(id_livro) {
            Ok(_) => {
                println!("\nLivro devolvido com sucesso!");
                salvar_biblioteca(biblioteca);
            }
            Err(e) => println!("\nErro ao devolver livro: {}", e),
        },
        Err(_) => println!("\nID inválido!"),
    }
    println!();
}

fn listar_emprestimos(biblioteca: &Biblioteca) {
    println!("\n--- Listar Empréstimos Ativos ---");
    let emprestimos = biblioteca.listar_emprestimos_ativos();

    if emprestimos.is_empty() {
        println!("\nNenhum empréstimo ativo.");
    } else {
        println!("\nEmpréstimos ativos:");
        for emprestimo in emprestimos {
            println!(
                "- ID Empréstimo: {}\n  ID Livro: {}\n  ID Usuário: {}\n  Data Empréstimo: {}\n  Data Devolução Prevista: {}\n",
                emprestimo.id_emprestimo,
                emprestimo.id_livro,
                emprestimo.id_usuario,
                emprestimo.data_emprestimo,
                emprestimo.data_devolucao_prevista
            );
        }
    }
    println!();
}

fn salvar_biblioteca(biblioteca: &Biblioteca) {
    if let Err(e) = biblioteca.salvar() {
        println!("Erro ao salvar dados: {}", e);
    }
}
