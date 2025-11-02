use crate::traits::Identificavel;
use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatusEmprestimo {
    Ativo,
    Devolvido,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emprestimo {
    pub id_emprestimo: Uuid,
    pub id_livro: Uuid,
    pub id_usuario: Uuid,
    pub data_emprestimo: NaiveDate,
    pub data_devolucao_prevista: NaiveDate,
    pub status: StatusEmprestimo,
}

impl Emprestimo {
    pub fn new(id_livro: Uuid, id_usuario: Uuid) -> Self {
        let data_emprestimo = Utc::now().date_naive();
        let data_devolucao_prevista = data_emprestimo + Duration::days(14);

        Emprestimo {
            id_emprestimo: Uuid::new_v4(),
            id_livro,
            id_usuario,
            data_emprestimo,
            data_devolucao_prevista,
            status: StatusEmprestimo::Ativo,
        }
    }

    pub fn finalizar(&mut self) {
        self.status = StatusEmprestimo::Devolvido;
    }
}

impl Identificavel for Emprestimo {
    fn id(&self) -> Uuid {
        self.id_emprestimo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_emprestimo() {
        let id_livro = Uuid::new_v4();
        let id_usuario = Uuid::new_v4();
        let emprestimo = Emprestimo::new(id_livro, id_usuario);

        assert_eq!(emprestimo.id_livro, id_livro);
        assert_eq!(emprestimo.id_usuario, id_usuario);
        assert_eq!(emprestimo.status, StatusEmprestimo::Ativo);
    }

    #[test]
    fn test_finalizar_emprestimo() {
        let id_livro = Uuid::new_v4();
        let id_usuario = Uuid::new_v4();
        let mut emprestimo = Emprestimo::new(id_livro, id_usuario);

        emprestimo.finalizar();
        assert_eq!(emprestimo.status, StatusEmprestimo::Devolvido);
    }

    #[test]
    fn test_data_devolucao() {
        let id_livro = Uuid::new_v4();
        let id_usuario = Uuid::new_v4();
        let emprestimo = Emprestimo::new(id_livro, id_usuario);

        let duracao = emprestimo.data_devolucao_prevista - emprestimo.data_emprestimo;
        assert_eq!(duracao.num_days(), 14);
    }

    #[test]
    fn test_identificavel_trait() {
        let id_livro = Uuid::new_v4();
        let id_usuario = Uuid::new_v4();
        let emprestimo = Emprestimo::new(id_livro, id_usuario);
        let id = emprestimo.id();
        assert_eq!(id, emprestimo.id_emprestimo);
    }
}
