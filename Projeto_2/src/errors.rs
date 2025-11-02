use std::fmt;

#[derive(Debug)]
pub enum ErroBiblioteca {
    LivroNaoEncontrado(String),
    UsuarioNaoEncontrado(String),
    EmprestimoNaoEncontrado(String),
    EstadoInvalido(String),
    ErroPersistencia(String),
}

impl fmt::Display for ErroBiblioteca {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroBiblioteca::LivroNaoEncontrado(id) => {
                write!(f, "Livro não encontrado: {}", id)
            }
            ErroBiblioteca::UsuarioNaoEncontrado(id) => {
                write!(f, "Usuário não encontrado: {}", id)
            }
            ErroBiblioteca::EmprestimoNaoEncontrado(id) => {
                write!(f, "Empréstimo não encontrado: {}", id)
            }
            ErroBiblioteca::EstadoInvalido(msg) => {
                write!(f, "Estado inválido: {}", msg)
            }
            ErroBiblioteca::ErroPersistencia(msg) => {
                write!(f, "Erro de persistência: {}", msg)
            }
        }
    }
}

impl std::error::Error for ErroBiblioteca {}
