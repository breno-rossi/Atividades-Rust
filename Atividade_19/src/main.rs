use dialoguer::{Select, Input, Confirm};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::fs;


#[derive(Clone)]
struct Atividade {
    nome: String,
    descricao: String,
    responsavel: Option<Rc<String>>,
}

// Estrutura auxiliar para serialização JSON
#[derive(Serialize, Deserialize)]
struct AtividadeJson {
    nome: String,
    descricao: String,
    responsavel: Option<String>,
}

// Gerenciador principal do sistema
struct GerenciadorCronograma {
    atividades: Vec<Atividade>,
    cache_responsaveis: HashMap<String, Rc<String>>,
}


impl GerenciadorCronograma {
    fn novo() -> Self {
        GerenciadorCronograma {
            atividades: Vec::new(),
            cache_responsaveis: HashMap::new(),
        }
    }

    // Carrega dados do arquivo JSON
    fn carregar_do_arquivo() -> Self {
        const ARQUIVO: &str = "cronograma_dados.json";
        
        let mut gerenciador = Self::novo();
        
        // Tenta ler o arquivo
        match fs::read_to_string(ARQUIVO) {
            Ok(conteudo) => {
                if conteudo.trim().is_empty() {
                    println!(" Arquivo vazio. Iniciando com cronograma limpo.");
                    return gerenciador;
                }
                
                match serde_json::from_str::<Vec<AtividadeJson>>(&conteudo) {
                    Ok(atividades_json) => {
                        println!(" Dados carregados com sucesso!");
                        
                        // Reconstrói as atividades com Rc compartilhados
                        for aj in atividades_json {
                            let responsavel = aj.responsavel.map(|nome| {
                                gerenciador.cache_responsaveis
                                    .entry(nome.clone())
                                    .or_insert_with(|| Rc::new(nome))
                                    .clone()
                            });
                            
                            gerenciador.atividades.push(Atividade {
                                nome: aj.nome,
                                descricao: aj.descricao,
                                responsavel,
                            });
                        }
                    }
                    Err(e) => {
                        println!(" Erro ao analisar JSON: {}", e);
                        println!("Iniciando com cronograma limpo.");
                    }
                }
            }
            Err(_) => {
                println!(" Arquivo não encontrado. Criando novo cronograma.");
            }
        }
        
        gerenciador
    }

    // Salva dados no arquivo JSON
    fn salvar_no_arquivo(&self) {
        const ARQUIVO: &str = "cronograma_dados.json";
        
        let atividades_json: Vec<AtividadeJson> = self.atividades
            .iter()
            .map(|a| AtividadeJson {
                nome: a.nome.clone(),
                descricao: a.descricao.clone(),
                responsavel: a.responsavel.as_ref().map(|rc| (**rc).clone()),
            })
            .collect();
        
        match serde_json::to_string_pretty(&atividades_json) {
            Ok(json) => {
                if let Err(e) = fs::write(ARQUIVO, json) {
                    println!(" Erro ao salvar arquivo: {}", e);
                } else {
                    println!(" Dados salvos com sucesso!");
                }
            }
            Err(e) => {
                println!(" Erro ao serializar dados: {}", e);
            }
        }
    }

    // Adiciona uma nova atividade
    fn adicionar_atividade(&mut self) {
        println!("\n===  NOVA ATIVIDADE ===\n");
        
        let nome: String = Input::new()
            .with_prompt("Nome da atividade")
            .interact_text()
            .unwrap();
        
        let descricao: String = Input::new()
            .with_prompt("Descrição")
            .interact_text()
            .unwrap();
        
        let atividade = Atividade {
            nome,
            descricao,
            responsavel: None,
        };
        
        self.atividades.push(atividade);
        self.salvar_no_arquivo();
        
        println!("\n Atividade criada com sucesso!");
        self.pausar();
    }

    // Edita uma atividade existente
    fn editar_atividade(&mut self, indice: usize) {
        println!("\n=== ✏️  EDITAR ATIVIDADE ===\n");
        
        let atividade = &mut self.atividades[indice];
        
        println!("Nome atual: {}", atividade.nome);
        let novo_nome: String = Input::new()
            .with_prompt("Novo nome (Enter para manter)")
            .allow_empty(true)
            .interact_text()
            .unwrap();
        
        if !novo_nome.is_empty() {
            atividade.nome = novo_nome;
        }
        
        println!("\nDescrição atual: {}", atividade.descricao);
        let nova_descricao: String = Input::new()
            .with_prompt("Nova descrição (Enter para manter)")
            .allow_empty(true)
            .interact_text()
            .unwrap();
        
        if !nova_descricao.is_empty() {
            atividade.descricao = nova_descricao;
        }
        
        self.salvar_no_arquivo();
        println!("\n Atividade atualizada!");
        self.pausar();
    }

    // Exclui uma atividade
    fn excluir_atividade(&mut self, indice: usize) {
        let atividade = &self.atividades[indice];
        
        let confirma = Confirm::new()
            .with_prompt(format!("Tem certeza que deseja excluir '{}'?", atividade.nome))
            .default(false)
            .interact()
            .unwrap();
        
        if confirma {
            self.atividades.remove(indice);
            self.limpar_cache_responsaveis();
            self.salvar_no_arquivo();
            println!("\n Atividade excluída!");
        } else {
            println!("\n Exclusão cancelada.");
        }
        
        self.pausar();
    }

    // Edita o responsável de uma atividade
    fn editar_responsavel(&mut self, indice: usize) {
        println!("\n===  EDITAR RESPONSÁVEL ===\n");
    
        let responsaveis_existentes = self.coletar_responsaveis_unicos();
        
        let mut opcoes = vec![];
        opcoes.push(" Associar Novo Indivíduo".to_string());
        
        for rc in &responsaveis_existentes {
            opcoes.push(format!(" {}", **rc));
        }
        
        opcoes.push("  Limpar Responsável".to_string());
        opcoes.push("⬅  Voltar ao Menu da Atividade".to_string());
        
        let selecao = Select::new()
            .with_prompt("Escolha uma opção")
            .items(&opcoes)
            .default(0)
            .interact()
            .unwrap();
        
        match selecao {
            0 => {
                // Novo indivíduo
                let nome: String = Input::new()
                    .with_prompt("Nome do responsável")
                    .interact_text()
                    .unwrap();
                
                let rc = self.cache_responsaveis
                    .entry(nome.clone())
                    .or_insert_with(|| Rc::new(nome))
                    .clone();
                
                self.atividades[indice].responsavel = Some(rc);
                self.salvar_no_arquivo();
                println!("\n Responsável associado!");
                self.pausar();
            }
            i if i <= responsaveis_existentes.len() => {
                // Responsável existente
                let rc = responsaveis_existentes[i - 1].clone();
                self.atividades[indice].responsavel = Some(rc);
                self.salvar_no_arquivo();
                println!("\n Responsável associado!");
                self.pausar();
            }
            i if i == opcoes.len() - 2 => {
                // Limpar responsável
                self.atividades[indice].responsavel = None;
                self.limpar_cache_responsaveis();
                self.salvar_no_arquivo();
                println!("\n Responsável removido!");
                self.pausar();
            }
            _ => {
            }
        }
    }

    // Coleta responsáveis únicos (sem duplicatas de Rc)
    fn coletar_responsaveis_unicos(&self) -> Vec<Rc<String>> {
        let mut responsaveis: Vec<Rc<String>> = self.atividades
            .iter()
            .filter_map(|a| a.responsavel.as_ref())
            .map(|rc| Rc::clone(rc))
            .collect();
        
        // Ordena por ponteiro para agrupar iguais
        responsaveis.sort_by(|a, b| Rc::as_ptr(a).cmp(&Rc::as_ptr(b)));
        
        // Remove duplicatas comparando ponteiros
        responsaveis.dedup_by(|a, b| Rc::ptr_eq(a, b));
        
        // Ordena alfabeticamente para exibição
        responsaveis.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
        
        responsaveis
    }

    // Remove Rc's não utilizados do cache
    fn limpar_cache_responsaveis(&mut self) {
        let mut nomes_em_uso = std::collections::HashSet::new();
        
        for atividade in &self.atividades {
            if let Some(ref rc) = atividade.responsavel {
                nomes_em_uso.insert((**rc).clone());
            }
        }
        
        self.cache_responsaveis.retain(|nome, _| nomes_em_uso.contains(nome));
    }

    // Menu de gerenciamento de uma atividade específica
    fn menu_atividade(&mut self, indice: usize) -> bool {
        loop {
            self.limpar_tela();
            
            let atividade = &self.atividades[indice];
            
            println!("╔════════════════════════════════════════════════════════════════╗");
            println!("║               📋 GERENCIAMENTO DE ATIVIDADE                    ║");
            println!("╚════════════════════════════════════════════════════════════════╝\n");
            
            println!(" Nome: {}", atividade.nome);
            println!(" Descrição: {}", atividade.descricao);
            
            match &atividade.responsavel {
                Some(rc) => {
                    println!(" Responsável: {} (Refs: {})", **rc, Rc::strong_count(rc));
                }
                None => {
                    println!(" Responsável: (nenhum)");
                }
            }
            
            println!();
            
            let opcoes = vec![
                "  Editar Atividade",
                "  Excluir Atividade",
                "  Editar Responsável",
                "⬅Voltar ao Menu Principal",
            ];
            
            let selecao = Select::new()
                .with_prompt("Escolha uma opção")
                .items(&opcoes)
                .default(0)
                .interact()
                .unwrap();
            
            match selecao {
                0 => self.editar_atividade(indice),
                1 => {
                    self.excluir_atividade(indice);
                    return true; // Sinaliza que a atividade foi excluída
                }
                2 => self.editar_responsavel(indice),
                3 => return false, // Volta ao menu principal
                _ => unreachable!(),
            }
        }
    }

    // Menu principal
    fn menu_principal(&mut self) {
        loop {
            self.limpar_tela();
            
            println!("╔════════════════════════════════════════════════════════════════╗");
            println!("║           SISTEMA DE GERENCIAMENTO DE CRONOGRAMA            ║");
            println!("╚════════════════════════════════════════════════════════════════╝\n");
            
            if self.atividades.is_empty() {
                println!(" Nenhuma atividade cadastrada.\n");
            } else {
                println!(" ATIVIDADES:\n");
                for (i, atividade) in self.atividades.iter().enumerate() {
                    let responsavel_str = match &atividade.responsavel {
                        Some(rc) => format!(" {}", **rc),
                        None => " (sem responsável)".to_string(),
                    };
                    
                    println!("{}. {} - {}", i + 1, atividade.nome, responsavel_str);
                    println!("    {}", atividade.descricao);
                    println!();
                }
            }
            
            let mut opcoes: Vec<String> = self.atividades
                .iter()
                .map(|a| {
                    let resp = match &a.responsavel {
                        Some(rc) => format!("👤 {}", **rc),
                        None => "".to_string(),
                    };
                    format!(" {} {}", a.nome, resp)
                })
                .collect();
            
            opcoes.push(" Nova Atividade".to_string());
            opcoes.push(" Sair".to_string());
            
            let selecao = Select::new()
                .with_prompt("Selecione uma atividade ou ação")
                .items(&opcoes)
                .default(0)
                .interact()
                .unwrap();
            
            if selecao < self.atividades.len() {
                // Selecionou uma atividade
                let excluida = self.menu_atividade(selecao);
                if !excluida {
                    // Se não foi excluída, continua normalmente
                }
            } else if selecao == self.atividades.len() {
                // Nova atividade
                self.adicionar_atividade();
            } else {
                // Sair
                println!("\n Encerrando o sistema...");
                self.salvar_no_arquivo();
                break;
            }
        }
    }

    // Funções auxiliares
    fn limpar_tela(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn pausar(&self) {
        println!("\nPressione Enter para continuar...");
        let _: String = Input::new()
            .allow_empty(true)
            .interact_text()
            .unwrap();
    }

    // Executa o sistema
    fn executar(&mut self) {
        self.menu_principal();
    }
}

// ============================================================================
// FUNÇÃO MAIN
// ============================================================================

fn main() {
    println!(" Iniciando Sistema de Gerenciamento de Cronograma...\n");
    
    let mut gerenciador = GerenciadorCronograma::carregar_do_arquivo();
    gerenciador.executar();
    
    println!("\n Sistema encerrado. Até logo!");
}