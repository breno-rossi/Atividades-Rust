use crate::traits::Identificavel;
use std::collections::HashMap;
use uuid::Uuid;

// Função genérica auxiliar para buscar um item por ID (Uuid) num HashMap
// Nota: O compilador Rust infere os lifetimes em muitos casos (elision),
// mas neste caso específico com múltiplas referências de entrada,
// precisamos declarar explicitamente para deixar claro que o retorno
// está vinculado ao lifetime de 'colecao'.
pub fn buscar_item_por_id<'a, T>(colecao: &'a HashMap<Uuid, T>, id: &Uuid) -> Option<&'a T>
where
    T: Identificavel,
{
    colecao.get(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::livros::Livro;
    use crate::usuarios::Usuario;

    #[test]
    fn test_buscar_livro_por_id() {
        let mut colecao: HashMap<Uuid, Livro> = HashMap::new();
        let livro = Livro::new("1984".to_string(), "George Orwell".to_string(), 1949);
        let id = livro.id;
        colecao.insert(id, livro);

        let resultado = buscar_item_por_id(&colecao, &id);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().titulo, "1984");
    }

    #[test]
    fn test_buscar_usuario_por_id() {
        let mut colecao: HashMap<Uuid, Usuario> = HashMap::new();
        let usuario = Usuario::new("João Silva".to_string());
        let id = usuario.id;
        colecao.insert(id, usuario);

        let resultado = buscar_item_por_id(&colecao, &id);
        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().nome, "João Silva");
    }

    #[test]
    fn test_buscar_id_inexistente() {
        let colecao: HashMap<Uuid, Livro> = HashMap::new();
        let id_inexistente = Uuid::new_v4();

        let resultado = buscar_item_por_id(&colecao, &id_inexistente);
        assert!(resultado.is_none());
    }
}
