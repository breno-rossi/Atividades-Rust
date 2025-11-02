use crate::traits::Identificavel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usuario {
    pub id: Uuid,
    pub nome: String,
}

impl Usuario {
    pub fn new(nome: String) -> Self {
        Usuario {
            id: Uuid::new_v4(),
            nome,
        }
    }
}

impl Identificavel for Usuario {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criar_usuario() {
        let usuario = Usuario::new("João Silva".to_string());
        assert_eq!(usuario.nome, "João Silva");
    }

    #[test]
    fn test_identificavel_trait() {
        let usuario = Usuario::new("João Silva".to_string());
        let id = usuario.id();
        assert_eq!(id, usuario.id);
    }
}
