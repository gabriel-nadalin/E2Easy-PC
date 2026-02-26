# mixnet-rust

## Requisitos
- Linux
- Rust (toolchain estável) e Cargo

## Compilar
```bash
cargo build
```
Para build otimizado:
```bash
cargo build --release
```

## Rodar exemplos
Os binários de exemplo ficam em `src/bin/`.

```bash
# Exemplo de eleição (gera arquivo de configuração)
cargo run --bin exemplo_eleicao

# Exemplo de votação (fluxo completo de votação, shuffle e totalização)
cargo run --bin exemplo_votacao

# Verificador individual
cargo run --bin verificador_individual

# Verificador universal
cargo run --bin verificador_universal
```

## Gerar headers do FFI
O projeto tem um gerador de headers em `src/bin/generate-headers.rs`.

```bash
cargo run --bin generate-headers
```

O header gerado será gravado no diretório raiz do projeto (ex.: `e2easy.h`).

## Usar a biblioteca estática (FFI)
1. Compile em **release** para gerar a biblioteca estática:
```bash
cargo build --release
```

2. A biblioteca ficará em:
`target/release/libmixnet_rust.a`

3. Inclua o header gerado (`e2easy.h`) no seu projeto C/C++ e linke com a lib estática:
```bash
gcc -o app main.c -I. -Ltarget/release -lmixnet_rust -lpthread -ldl
```

> Ajuste os flags conforme seu toolchain e dependências do sistema.

## Estrutura de dados (JSON)
- Configurações: `config/`
- Saídas: `outputs/`

## Notas
- Para executar corretamente os exemplos, garanta que os arquivos JSON esperados existam.
- Em caso de erro de link, confira se o caminho para `libmixnet_rust.a` está correto.
