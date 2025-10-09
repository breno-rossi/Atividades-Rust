# Image Downloader 🖼️

Um programa em Rust que baixa automaticamente todas as imagens de uma página web.

## ✨ Funcionalidades

- ✅ Validação de URLs
- 🔍 Extração automática de links de imagens
- 🔗 Conversão de URLs relativas para absolutas
- 📥 Download paralelo de múltiplas imagens
- 🛡️ Tratamento robusto de erros
- 📊 Feedback visual em tempo real


## 🚀 Como Usar

### 1. Clone ou crie o projeto

```bash
# Criar novo projeto
cargo new image_downloader
cd image_downloader
```

### 2. Configure as dependências

Adicione ao arquivo `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
scraper = "0.17"
url = "2.4"
thiserror = "1.0"
```

### 3. Copie o código

Substitua o conteúdo de `src/main.rs` pelo código fornecido.

### 4. Compile o projeto

```bash
# Modo debug (mais rápido para compilar)
cargo build

# Modo release (otimizado, recomendado)
cargo build --release
```

### 5. Execute o programa

```bash
# Modo debug
cargo run

# Modo release
cargo run --release

# Ou execute o binário diretamente
./target/release/image_downloader
```

### 6. Use o programa

Digite a URL da página quando solicitado:

```
Digite a URL da página: https://example.com
```

As imagens serão salvas no diretório `./downloads`.

## 📦 Dependências

O projeto utiliza as seguintes crates:

- **`reqwest`** (0.11) - Cliente HTTP robusto para fazer requisições web
  - Feature `blocking` habilitada para operações síncronas
- **`scraper`** (0.17) - Parser de HTML baseado em seletores CSS
- **`url`** (2.4) - Manipulação e validação de URLs
- **`thiserror`** (1.0) - Macros para criação de tipos de erro customizados

Todas as dependências serão baixadas automaticamente pelo Cargo durante a compilação.

## 🎯 Formatos Suportados

- `.jpg` / `.jpeg`
- `.png`
- `.gif`
- `.webp`
- `.bmp`
- `.svg`


## ⚠️ Tratamento de Erros

O programa lida com diversos cenários de erro:

- ❌ URL inválida ou malformada
- ❌ Falha na conexão de rede
- ❌ Página não encontrada (404)
- ❌ Timeout de requisição (30 segundos)
- ❌ Erro ao salvar arquivos
- ❌ Nenhuma imagem encontrada na página

## 💡 Dicas

- O programa cria automaticamente o diretório `./downloads` se ele não existir
- URLs relativas são convertidas automaticamente para absolutas
- Apenas formatos de imagem comuns são baixados
- Se uma imagem falhar, as outras continuam sendo baixadas
