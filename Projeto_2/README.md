# Sistema de Gerenciamento de Biblioteca Virtual

Sistema modular desenvolvido em Rust para gerenciar uma biblioteca virtual com funcionalidades completas de cadastro, empréstimo e persistência de dados.

## Características

- **Identificadores Únicos**: Uso de UUID v4 para todos os registros (livros, usuários, empréstimos)
- **Persistência**: Dados salvos em formato JSON
- **Segurança de Tipos**: Uso extensivo de enums e tipos seguros do Rust
- **Controle de Erros**: Sistema robusto com `Result<T, ErroBiblioteca>`
- **Traits e Generics**: Implementação de conceitos avançados de Rust

## Estrutura do Projeto

```
src/
├── main.rs           # Interface CLI
├── lib.rs            # Declaração dos módulos
├── biblioteca.rs     # Módulo principal com lógica de negócio
├── livros.rs         # Estruturas e lógica de livros
├── usuarios.rs       # Estruturas e lógica de usuários
├── emprestimos.rs    # Estruturas e lógica de empréstimos
├── errors.rs         # Definição de erros customizados
├── traits.rs         # Trait Identificavel
└── generics.rs       # Função genérica customizada
```

## Dependências

- `uuid` (1.16.0) - Geração de identificadores únicos
- `serde` (1.0) - Serialização/deserialização
- `serde_json` (1.0) - Formato JSON
- `chrono` (0.4.40) - Manipulação de datas
- `tempfile` (3.19.1) - Arquivos temporários para testes (dev-dependency)

## Como Executar

### Compilar o projeto
```bash
cargo build --release
```

### Executar a aplicação
```bash
cargo run
```

### Executar os testes
```bash
cargo test
```

## Funcionalidades

### Gerenciamento de Livros
- Adicionar livro (com geração automática de UUID)
- Remover livro (com validação de empréstimos ativos)
- Buscar por ID, título ou autor
- Listar todos os livros, disponíveis ou emprestados

### Gerenciamento de Usuários
- Cadastrar usuário (com geração automática de UUID)
- Listar usuários cadastrados

### Gerenciamento de Empréstimos
- Emprestar livro para usuário
- Devolver livro
- Listar empréstimos ativos
- Listar empréstimos de um usuário específico
- Datas: empréstimo (hoje) e devolução prevista (14 dias)

### Persistência
- Salvamento automático após cada operação
- Carregamento automático ao iniciar
- Arquivo: `biblioteca_dados.json`

## Conceitos Rust Implementados

### Enums
- `StatusLivro`: Disponivel | Emprestado
- `StatusEmprestimo`: Ativo | Devolvido
- `ErroBiblioteca`: Variantes de erro customizadas

### Trait Customizado
```rust
pub trait Identificavel {
    fn id(&self) -> Uuid;
}
```
Implementado para `Livro`, `Usuario` e `Emprestimo`.

### Função Genérica
```rust
pub fn buscar_item_por_id<'a, T>(
    colecao: &'a HashMap<Uuid, T>,
    id: &Uuid
) -> Option<&'a T>
where
    T: Identificavel
```

### HashMap<Uuid, T>
Usado para armazenamento eficiente de livros, usuários e empréstimos.

## Exemplo de Uso

1. Execute o programa: `cargo run`
2. Escolha a opção "5" para adicionar um usuário
3. Escolha a opção "1" para adicionar um livro
4. Use os IDs gerados para realizar empréstimos
5. Todas as operações são automaticamente salvas

## Testes

O projeto inclui 21 testes unitários cobrindo:
- Criação de entidades
- Validações de estado
- Operações de empréstimo/devolução
- Persistência de dados (usando `tempfile`)
- Funções genéricas

Execute com:
```bash
cargo test
```

## Autor

Desenvolvido como projeto acadêmico para demonstrar conceitos avançados de Rust.
