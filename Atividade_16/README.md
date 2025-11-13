# Calculador de Números Primos Concorrente

## Descrição

Este programa em Rust utiliza múltiplas threads para encontrar números primos em um grande intervalo de forma eficiente. O projeto demonstra o uso de comunicação entre threads usando canais (mpsc) e análise de performance de estratégias de paralelismo.

## Funcionalidades

- Busca concorrente de números primos usando múltiplas threads workers
- Comunicação entre threads via canais mpsc
- Monitoramento de progresso em tempo real
- Divisão linear de trabalho entre threads
- Agregação e ordenação de resultados
- Reflexão sobre balanceamento de carga

## Requisitos

- Rust 1.70 ou superior
- Cargo (gerenciador de pacotes do Rust)

## Dependências

O projeto utiliza a crate `primal` (versão 0.3) para verificação de primalidade.

## Como Compilar

1. Navegue até o diretório do projeto:
```bash
cd Atividade_16
```

2. Compile o projeto:
```bash
cargo build --release
```

## Como Executar

Execute o programa compilado:
```bash
cargo run --release
```

Ou, após compilar, execute o binário diretamente:
```bash
./target/release/calculador_primos
```

## Como Usar

1. O programa solicitará o **limite superior** para a busca de primos (ex: 2000000)
2. Em seguida, solicitará o **número de threads workers** a serem utilizadas (recomendado: 8 para um processador de 8 núcleos)
3. O programa iniciará a busca e exibirá o progresso
4. Ao finalizar, exibirá:
   - Total de números primos encontrados
   - Os primeiros e últimos primos (se houver muitos)
   - Ou todos os primos (se forem 100 ou menos)

## Exemplo de Uso

```
=== Calculador de Números Primos Concorrente ===

Digite o limite superior para busca de primos: 2000000
Digite o número de threads workers: 8

Iniciando busca de primos até 2000000 usando 8 threads...

Worker 1 iniciado: verificando intervalo [2, 250000]
Worker 2 iniciado: verificando intervalo [250001, 500000]
Worker 3 iniciado: verificando intervalo [500001, 750000]
Worker 4 iniciado: verificando intervalo [750001, 1000000]
Worker 5 iniciado: verificando intervalo [1000001, 1250000]
Worker 6 iniciado: verificando intervalo [1250001, 1500000]
Worker 7 iniciado: verificando intervalo [1500001, 1750000]
Worker 8 iniciado: verificando intervalo [1750001, 2000000]

Trabalhando... 3 de 8 workers finalizados
Trabalhando... 5 de 8 workers finalizados

Worker 1 encontrou 22166 números primos
Worker 2 encontrou 19676 números primos
...

=== Resultados ===
Total de números primos encontrados: 148933

Primeiros 20 primos:
       2       3       5       7      11      13      17      19      23      29
      31      37      41      43      47      53      59      61      67      71

Últimos 20 primos:
 1999643 1999657 1999691 1999711 1999757 1999763 1999769 1999787 1999811 1999843
 1999853 1999873 1999879 1999897 1999921 1999931 1999933 1999957 1999969 1999993

Busca concluída com sucesso!
```

## Configurações Recomendadas para PC de 8 Núcleos

Para aproveitar ao máximo seu processador de 8 núcleos:

- **Uso otimizado**: Use 8 threads (uma por núcleo)
- **Testes pequenos**: Para limites até 100.000, use 4-8 threads
- **Testes grandes**: Para limites acima de 1.000.000, use 8 threads
- **Performance máxima**: Compile sempre com `--release` para otimizações

## Estrutura do Código

- **WorkerMessage**: Enum que define os tipos de mensagens trocadas entre threads
  - `Progress(f32)`: Progresso percentual do worker
  - `FoundPrimes(Vec<u64>)`: Vetor com primos encontrados
  - `Finished`: Sinaliza término do worker

- **worker_thread()**: Função executada por cada thread worker
  - Verifica primalidade usando `primal::is_prime()`
  - Envia progresso periodicamente
  - Retorna resultados via canal

- **main()**: Thread principal
  - Coleta entrada do usuário
  - Cria canal mpsc
  - Divide o trabalho linearmente
  - Spawna threads workers
  - Coleta e agrega resultados
  - Ordena e exibe primos encontrados

## Reflexão sobre Balanceamento de Carga

O código inclui uma reflexão detalhada sobre:
1. Por que a divisão linear causa desbalanceamento
2. Estratégia alternativa (Fila de Trabalho Dinâmica) para melhorar o balanceamento

Veja o comentário no início do arquivo `src/main.rs`.
