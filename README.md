![AXIOM Banner](./banner.png)

<div align="center">    

# AXIOM

**O Sistema está online.**
Uma entidade de RPG Social que transforma conversas em batalhas de raid dentro do Discord.

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/freitaseric/axiom/ci.yml)
![License](https://img.shields.io/badge/license-MIT-blue)

</div>

## 👁️ O Conceito

O **AXIOM** não é apenas um bot de economia. É uma camada de gamificação invisível sobre o seu servidor.
Quando a atividade social atinge níveis críticos, o Sistema abre **Dungeons** em Threads dedicadas. Para
derrotar os chefes, os membros não usam comandos de ataque, mas sim **interações sociais reais** (risadas, perguntas,
engajamento) no chat geral.

## 🚀 Tech Stack (Performance Tier)

Construído para escalar com performance nativa e zero Garbage Collection.

- **Core:** Rust (Edition 2024)
- **Discord Lib:** [Twilight](https://twilight.rs/) (Modular & Low-level)
- **Database:** MongoDB (Async via `mongodb` crate)
- **Rendering:** Tiny-Skia & Resvg (SVG to PNG render em microssegundos)
- **Pattern Matching:** Regex (Compilação estática via `OnceLock`)

## 🛠️ Instalação (Self-Hosting)

### Pré-requisitos

- Rust (Cargo) instalado.
- MongoDB rodando (Local ou Atlas).
- Uma aplicação no Discord Developer Portal com **Message Content Intent** ativado.

### 1. Clone o repositório

```bash
git clone https://github.com/freitaseric/axiom.git  
cd axiom
```

### 2. Configure o Ambiente

Crie um arquivo .env na raiz do projeto:

```dotenv
DISCORD_TOKEN=seu_token_aqui
MONGO_URI=mongodb://localhost:27017/axiom_db
RUST_LOG=info
```

### 3. Assets

Certifique-se de que a pasta `assets/` contém:

- `hp_bar.svg` (Template visual)
- `font.ttf` (Fonte utilizada no render, ex: Rajdhani)

### 4. Execute

```bash
# Modo de desenvolvimento
cargo run

# Modo de produção (Otimizado)
cargo build --release
./target/release/axiom
```

## 🎮 Comandos

> [!WARNING]
> Em desenvolvimento...

## 📂 Estrutura do Projeto

```txt
src/
├── core/           # Conexão DB e Roteador de Eventos
├── features/       # Lógica de Gameplay (Vigilância e Masmorra)
├── services/       # Motor de Renderização de Imagem
└── main.rs         # Entry point e Sharding
```

## 📄 Licença

Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.

<p align="center">Desenvolvido por <a href="https://github.com/freitaseric">Eric Freitas</a>.</p>