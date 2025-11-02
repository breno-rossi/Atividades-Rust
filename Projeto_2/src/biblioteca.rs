use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use uuid::Uuid;

use crate::emprestimos::{Emprestimo, StatusEmprestimo};
use crate::errors::ErroBiblioteca;
use crate::livros::{Livro, StatusLivro};
use crate::usuarios::Usuario;

#[derive(Debug, Serialize, Deserialize)]
struct DadosPersistencia {
    livros: HashMap<Uuid, Livro>,
    usuarios: HashMap<Uuid, Usuario>,
    emprestimos: HashMap<Uuid, Emprestimo>,
}

pub struct Biblioteca {
    caminho_arquivo: PathBuf,
    livros: HashMap<Uuid, Livro>,
    usuarios: HashMap<Uuid, Usuario>,
    emprestimos: HashMap<Uuid, Emprestimo>,
}

impl Biblioteca {
    pub fn new(caminho_arquivo: PathBuf) -> Self {
        Biblioteca {
            caminho_arquivo,
            livros: HashMap::new(),
            usuarios: HashMap::new(),
            emprestimos: HashMap::new(),
        }
    }

    // Persistência
    pub fn carregar(caminho: &PathBuf) -> Result<Self, ErroBiblioteca> {
        if !caminho.exists() {
            return Ok(Biblioteca::new(caminho.clone()));
        }

        let file = File::open(caminho).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao abrir arquivo: {}", e))
        })?;

        let reader = BufReader::new(file);
        let dados: DadosPersistencia = serde_json::from_reader(reader).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao deserializar JSON: {}", e))
        })?;

        Ok(Biblioteca {
            caminho_arquivo: caminho.clone(),
            livros: dados.livros,
            usuarios: dados.usuarios,
            emprestimos: dados.emprestimos,
        })
    }

    pub fn salvar(&self) -> Result<(), ErroBiblioteca> {
        let dados = DadosPersistencia {
            livros: self.livros.clone(),
            usuarios: self.usuarios.clone(),
            emprestimos: self.emprestimos.clone(),
        };

        let file = File::create(&self.caminho_arquivo).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao criar arquivo: {}", e))
        })?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &dados).map_err(|e| {
            ErroBiblioteca::ErroPersistencia(format!("Erro ao serializar JSON: {}", e))
        })?;

        Ok(())
    }

    // Gerenciamento de Livros
    pub fn adicionar_livro(
        &mut self,
        titulo: String,
        autor: String,
        ano: u16,
    ) -> Result<Uuid, ErroBiblioteca> {
        let livro = Livro::new(titulo, autor, ano);
        let id = livro.id;
        self.livros.insert(id, livro);
        Ok(id)
    }

    pub fn remover_livro(&mut self, id_livro: Uuid) -> Result<(), ErroBiblioteca> {
        if !self.livros.contains_key(&id_livro) {
            return Err(ErroBiblioteca::LivroNaoEncontrado(format!("{}", id_livro)));
        }

        // Verificar se existe empréstimo ativo para este livro
        let tem_emprestimo_ativo = self
            .emprestimos
            .values()
            .any(|e| e.id_livro == id_livro && e.status == StatusEmprestimo::Ativo);

        if tem_emprestimo_ativo {
            return Err(ErroBiblioteca::EstadoInvalido(
                "Não é possível remover um livro com empréstimo ativo".to_string(),
            ));
        }

        self.livros.remove(&id_livro);
        Ok(())
    }

    pub fn buscar_livro_por_id(&self, id: Uuid) -> Option<&Livro> {
        self.livros.get(&id)
    }

    pub fn buscar_livros_por_titulo(&self, titulo: &str) -> Vec<&Livro> {
        self.livros
            .values()
            .filter(|l| l.titulo.to_lowercase().contains(&titulo.to_lowercase()))
            .collect()
    }

    pub fn buscar_livros_por_autor(&self, autor: &str) -> Vec<&Livro> {
        self.livros
            .values()
            .filter(|l| l.autor.to_lowercase().contains(&autor.to_lowercase()))
            .collect()
    }

    pub fn listar_todos_livros(&self) -> Vec<&Livro> {
        self.livros.values().collect()
    }

    pub fn listar_livros_disponiveis(&self) -> Vec<&Livro> {
        self.livros
            .values()
            .filter(|l| l.status == StatusLivro::Disponivel)
            .collect()
    }

    pub fn listar_livros_emprestados(&self) -> Vec<&Livro> {
        self.livros
            .values()
            .filter(|l| l.status == StatusLivro::Emprestado)
            .collect()
    }

    // Gerenciamento de Usuários
    pub fn adicionar_usuario(&mut self, nome: String) -> Result<Uuid, ErroBiblioteca> {
        let usuario = Usuario::new(nome);
        let id = usuario.id;
        self.usuarios.insert(id, usuario);
        Ok(id)
    }

    pub fn buscar_usuario_por_id(&self, id: Uuid) -> Option<&Usuario> {
        self.usuarios.get(&id)
    }

    pub fn listar_usuarios(&self) -> Vec<&Usuario> {
        self.usuarios.values().collect()
    }

    // Gerenciamento de Empréstimos
    pub fn emprestar_livro(
        &mut self,
        id_usuario: Uuid,
        id_livro: Uuid,
    ) -> Result<Uuid, ErroBiblioteca> {
        // Validar existência do usuário
        if !self.usuarios.contains_key(&id_usuario) {
            return Err(ErroBiblioteca::UsuarioNaoEncontrado(format!(
                "{}",
                id_usuario
            )));
        }

        // Validar existência do livro
        let livro = self
            .livros
            .get_mut(&id_livro)
            .ok_or_else(|| ErroBiblioteca::LivroNaoEncontrado(format!("{}", id_livro)))?;

        // Tentar emprestar o livro
        livro.emprestar()?;

        // Criar empréstimo
        let emprestimo = Emprestimo::new(id_livro, id_usuario);
        let id_emprestimo = emprestimo.id_emprestimo;
        self.emprestimos.insert(id_emprestimo, emprestimo);

        Ok(id_emprestimo)
    }

    pub fn devolver_livro(&mut self, id_livro: Uuid) -> Result<(), ErroBiblioteca> {
        // Encontrar o empréstimo ativo do livro
        let emprestimo_id = self
            .emprestimos
            .iter()
            .find(|(_, e)| e.id_livro == id_livro && e.status == StatusEmprestimo::Ativo)
            .map(|(id, _)| *id)
            .ok_or_else(|| {
                ErroBiblioteca::EmprestimoNaoEncontrado(format!(
                    "Empréstimo ativo não encontrado para o livro {}",
                    id_livro
                ))
            })?;

        // Finalizar empréstimo
        if let Some(emprestimo) = self.emprestimos.get_mut(&emprestimo_id) {
            emprestimo.finalizar();
        }

        // Devolver livro
        let livro = self
            .livros
            .get_mut(&id_livro)
            .ok_or_else(|| ErroBiblioteca::LivroNaoEncontrado(format!("{}", id_livro)))?;

        livro.devolver()?;

        Ok(())
    }

    pub fn listar_emprestimos_ativos(&self) -> Vec<&Emprestimo> {
        self.emprestimos
            .values()
            .filter(|e| e.status == StatusEmprestimo::Ativo)
            .collect()
    }

    pub fn listar_emprestimos_usuario(&self, id_usuario: Uuid) -> Vec<&Emprestimo> {
        self.emprestimos
            .values()
            .filter(|e| e.id_usuario == id_usuario && e.status == StatusEmprestimo::Ativo)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_adicionar_livro() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        let id = biblioteca
            .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
            .unwrap();

        assert!(biblioteca.buscar_livro_por_id(id).is_some());
    }

    #[test]
    fn test_adicionar_usuario() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        let id = biblioteca
            .adicionar_usuario("João Silva".to_string())
            .unwrap();

        assert!(biblioteca.buscar_usuario_por_id(id).is_some());
    }

    #[test]
    fn test_emprestar_livro() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        let id_livro = biblioteca
            .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
            .unwrap();
        let id_usuario = biblioteca
            .adicionar_usuario("João Silva".to_string())
            .unwrap();

        let resultado = biblioteca.emprestar_livro(id_usuario, id_livro);
        assert!(resultado.is_ok());

        let livro = biblioteca.buscar_livro_por_id(id_livro).unwrap();
        assert_eq!(livro.status, StatusLivro::Emprestado);
    }

    #[test]
    fn test_devolver_livro() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        let id_livro = biblioteca
            .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
            .unwrap();
        let id_usuario = biblioteca
            .adicionar_usuario("João Silva".to_string())
            .unwrap();

        biblioteca.emprestar_livro(id_usuario, id_livro).unwrap();
        let resultado = biblioteca.devolver_livro(id_livro);
        assert!(resultado.is_ok());

        let livro = biblioteca.buscar_livro_por_id(id_livro).unwrap();
        assert_eq!(livro.status, StatusLivro::Disponivel);
    }

    #[test]
    fn test_remover_livro_com_emprestimo_ativo() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        let id_livro = biblioteca
            .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
            .unwrap();
        let id_usuario = biblioteca
            .adicionar_usuario("João Silva".to_string())
            .unwrap();

        biblioteca.emprestar_livro(id_usuario, id_livro).unwrap();
        let resultado = biblioteca.remover_livro(id_livro);
        assert!(resultado.is_err());
    }

    #[test]
    fn test_persistencia() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");

        // Criar e salvar biblioteca
        {
            let mut biblioteca = Biblioteca::new(caminho.clone());
            biblioteca
                .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
                .unwrap();
            biblioteca
                .adicionar_usuario("João Silva".to_string())
                .unwrap();
            biblioteca.salvar().unwrap();
        }

        // Carregar biblioteca
        {
            let biblioteca = Biblioteca::carregar(&caminho).unwrap();
            assert_eq!(biblioteca.listar_todos_livros().len(), 1);
            assert_eq!(biblioteca.listar_usuarios().len(), 1);
        }
    }

    #[test]
    fn test_buscar_livros_por_titulo() {
        let dir = tempdir().unwrap();
        let caminho = dir.path().join("test.json");
        let mut biblioteca = Biblioteca::new(caminho);

        biblioteca
            .adicionar_livro("1984".to_string(), "George Orwell".to_string(), 1949)
            .unwrap();
        biblioteca
            .adicionar_livro("Animal Farm".to_string(), "George Orwell".to_string(), 1945)
            .unwrap();

        let resultados = biblioteca.buscar_livros_por_titulo("1984");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].titulo, "1984");
    }
}
