use crate::errors::ErroBiblioteca;
use crate::traits::Identificavel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatusLivro {
    Disponivel,
    Emprestado,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Livro {
    pub id: Uuid,
    pub titulo: String,
    pub autor: String,
    pub ano: u16,
    pub status: StatusLivro,
}

impl Livro {
    pub fn new(titulo: String, autor: String, ano: u16) -> Self {
        Livro {
            id: Uuid::new_v4(),
            titulo,
            autor,
            ano,
            status: StatusLivro::Disponivel,
        }
    }

    pub fn emprestar(&mut self) -> Result<(), ErroBiblioteca> {
        match self.status {
            StatusLivro::Disponivel => {
                self.status = StatusLivro::Emprestado;
                Ok(())
            }
            StatusLivro::Emprestado => Err(ErroBiblioteca::EstadoInvalido(
                "Livro já está emprestado".to_string(),
            )),
        }
    }

    pub fn devolver(&mut self) -> Result<(), ErroBiblioteca> {
        match self.status {
            StatusLivro::Emprestado => {
                self.status = StatusLivro::Disponivel;
                Ok(())
            }
            StatusLivro::Disponivel => Err(ErroBiblioteca::EstadoInvalido(
                "Livro já está disponível".to_string(),
            )),
        }
    }
}

impl Identificavel for Livro {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_livro() {
        let livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        assert_eq!(livro.titulo, "1984");
        assert_eq!(livro.autor, "George Orwell");
        assert_eq!(livro.ano, 1949);
        assert_eq!(livro.status, StatusLivro::Disponivel);
    }

    #[test]
    fn test_emprestar_livro() {
        let mut livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        assert!(livro.emprestar().is_ok());
        assert_eq!(livro.status, StatusLivro::Emprestado);
    }

    #[test]
    fn test_emprestar_livro_ja_emprestado() {
        let mut livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        livro.emprestar().unwrap();
        assert!(livro.emprestar().is_err());
    }

    #[test]
    fn test_devolver_livro() {
        let mut livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        livro.emprestar().unwrap();
        assert!(livro.devolver().is_ok());
        assert_eq!(livro.status, StatusLivro::Disponivel);
    }

    #[test]
    fn test_identificavel_trait() {
        let livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        let id = livro.id();
        assert_eq!(id, livro.id);
    }
}
