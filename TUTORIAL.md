# Tutorial: Metasploit TUI

Guia prático de uso da interface de terminal para o Metasploit Framework.

## Índice

1. [Introdução](#1-introdução)
2. [Pré-requisitos](#2-pré-requisitos)
3. [Instalação](#3-instalação)
4. [Navegação Geral](#4-navegação-geral)
5. [Dashboard (Aba 1)](#5-dashboard-aba-1)
6. [Explorer (Aba 2)](#6-explorer-aba-2)
7. [Payload Generator (Aba 3)](#7-payload-generator-aba-3)
8. [Sessions (Aba 4)](#8-sessions-aba-4)
9. [Console (Aba 5)](#9-console-aba-5)
10. [Scripts (Aba 6)](#10-scripts-aba-6)
11. [Referência Rápida de Teclas](#11-referência-rápida-de-teclas)
12. [Solução de Problemas](#12-solução-de-problemas)

---

## 1. Introdução

O **Metasploit TUI** é uma aplicação de terminal (TUI) que permite interagir com o Metasploit Framework sem sair do teclado. Diferente de interfaces web ou RPC, ele executa comandos diretamente no terminal chamando os binários `msfconsole` e `msfvenom` como subprocessos.

### O que você pode fazer:

- Navegar e buscar módulos (exploits, auxiliares, payloads, etc.)
- Gerar payloads com `msfvenom` por meio de um formulário interativo
- Listar e gerenciar sessões ativas (meterpreter/shell)
- Executar comandos no `msfconsole` diretamente de dentro da TUI
- Rodar resource scripts (`.rc`) com um pressionar de tecla
- Verificar a saúde do sistema (versões do msfconsole, msfvenom, ruby, banco de dados)

---

## 2. Pré-requisitos

Antes de começar, você precisa ter instalado:

| Componente | Como verificar |
|---|---|
| **Metasploit Framework** | `msfconsole --version` |
| **msfvenom** | `msfvenom --version` |
| **Ruby** | `ruby --version` |

Instalação rápida no **Kali Linux**:

```bash
sudo apt update && sudo apt install -y metasploit-framework
```

---

## 3. Instalação

### 3.1 Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 3.2 Clonar e compilar

```bash
git clone https://github.com/seu-usuario/metasploit-ui.git
cd metasploit-ui
cargo build --release
```

### 3.3 Executar

```bash
./target/release/metasploit-tui
```

Ou diretamente com `cargo run --release`.

---

## 4. Navegação Geral

Ao iniciar, a tela é dividida em três áreas:

```
┌─────────────────────────────────────────────────┐
│  [1] Dashboard  [2] Explorer  [3] Payload ...   │  ← Barra de abas
├─────────────────────────────────────────────────┤
│                                                   │
│              Área de conteúdo                     │  ← Aba ativa
│                                                   │
├─────────────────────────────────────────────────┤
│  [1] Dash [2] Explore ... [?] Help [q] Quit     │  ← Barra de status
└─────────────────────────────────────────────────┘
```

### Teclas de navegação global

| Tecla | Ação |
|---|---|
| `1` a `6` | Ir diretamente para a aba |
| `Tab` | Próxima aba |
| `Shift+Tab` | Aba anterior |
| `?` | Abrir/fechar tela de ajuda |
| `q` ou `Ctrl+C` | Sair do programa |

---

## 5. Dashboard (Aba 1)

A tela inicial exibe um banner ASCII "MSF" e o painel de saúde do sistema.

### Como usar

Assim que o programa inicia, um health check rápido é executado automaticamente. Para uma verificação completa:

1. Pressione `h`
2. O sistema testará:
   - **msfconsole** — se está instalado e a versão
   - **msfvenom** — se está instalado e a versão
   - **ruby** — versão do interpretador
   - **database** — status da conexão com o banco de dados do Metasploit

Cada item aparece com um ícone:
- `✔` (verde/negrito) — componente OK
- `✘` (vermelho/negrito) — componente com problema

### Teclas

| Tecla | Ação |
|---|---|
| `h` | Executar health check completo |

---

## 6. Explorer (Aba 2)

O explorador de módulos permite navegar por todas as categorias de módulos do Metasploit, buscar por nome e visualizar informações detalhadas.

### Estrutura da tela

```
┌─ Categories ────┐ ┌─ Modules ────────────────────┐
│  All             │ │  exploit/multi/handler [excellent]
│  exploit         │ │  exploit/multi/script       [manual]
│  auxiliary       │ │  ...                         │
│  payload         │ │                              │
│  post            │ │                              │
│  encoder         │ │                              │
│  nop             │ │                              │
│  evasion         │ │                              │
└─────────────────┘ └──────────────────────────────┘
```

### Como usar

**Passo 1 — Navegar entre categorias:**
- Use `↑` e `↓` para selecionar uma categoria à esquerda
- Ao mudar de categoria, os módulos correspondentes carregam à direita

**Passo 2 — Escolher um módulo:**
- Com uma categoria selecionada, use `→` para mover o foco para a lista de módulos
- Use `↑` e `↓` para navegar entre os módulos
- Use `←` para voltar à lista de categorias

**Passo 3 — Ver detalhes:**
- Pressione `Enter` sobre um módulo para abrir a tela de detalhes
- Na tela de detalhes você verá: nome, rank e descrição
- Pressione `i` para buscar informações completas (via `msfconsole info <modulo>`)
- Pressione `r` para executar o módulo diretamente
- Pressione `Enter` ou `Esc` para voltar à lista

**Busca:**
- Pressione `/` para abrir o campo de busca
- Digite parte do nome do módulo (a lista filtra em tempo real)
- Pressione `Esc` ou `Enter` para fechar a busca

### Teclas

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar entre categorias ou módulos |
| `←/→` | Alternar foco entre categorias e módulos |
| `Enter` | Ver detalhes do módulo |
| `/` | Abrir busca |
| `i` | Buscar informações completas do módulo |
| `r` | Executar módulo |
| `Esc` | Voltar / fechar busca |

---

## 7. Payload Generator (Aba 3)

O gerador de payloads oferece um formulário para configurar e gerar payloads com `msfvenom`.

### Estrutura da tela

```
┌─ Payload ──────────┐ ┌─ LHOST ─────────────┐
│ linux/x64/meter...  │ │ 192.168.1.10        │
├────────────────────┤ ├─────────────────────┤
│ LPORT              │ │ Format              │
│ 4444               │ │ elf                 │
├────────────────────┤ ├─────────────────────┤
│ Encoder            │ │ Iterations          │
│                    │ │ 1                   │
├────────────────────┤ ├─────────────────────┤
│ Platform           │ │ Arch                │
│ linux              │ │ x64                 │
├────────────────────┤ ├─────────────────────┤
│ Output             │ │ Extra options       │
│ ./payload.elf      │ │                     │
└────────────────────┘ └─────────────────────┘
┌─ Preview ────────────────────────────────────┐
│ [Enter] Preview  [Tab] Next  [/] Edit  [g] G…│
└──────────────────────────────────────────────┘
```

### Como usar

**Passo 1 — Preencher o formulário:**
- Navegue entre os campos com `Tab` (próximo) ou `Shift+Tab` (anterior)
- Pressione `/` para editar o campo atualmente focado
- Os campos disponíveis são:
  - **Payload** — nome do payload (ex: `linux/x64/meterpreter/reverse_tcp`)
  - **LHOST** — IP do listener
  - **LPORT** — porta (padrão: `4444`)
  - **Format** — formato de saída (ex: `elf`, `exe`, `python`, `raw`)
  - **Encoder** — encoder opcional
  - **Iterations** — número de iterações do encoder
  - **Platform** — plataforma alvo
  - **Arch** — arquitetura (x86, x64, etc.)
  - **Output** — caminho do arquivo de saída (padrão: `./payload.elf`)
  - **Extra options** — opções extras para passar ao msfvenom

**Passo 2 — Visualizar o preview:**
- Pressione `Enter` para ver o comando `msfvenom` que será executado

**Passo 3 — Gerar o payload:**
- Pressione `g` para executar o `msfvenom` com as opções configuradas
- O resultado (ou mensagem de erro) aparece no painel **Output** na parte inferior

**Limpar output:**
- Pressione `c` para limpar o histórico de saída

### Teclas

| Tecla | Ação |
|---|---|
| `Tab` / `↓` | Próximo campo |
| `Shift+Tab` / `↑` | Campo anterior |
| `/` | Editar campo focado |
| `Enter` | Mostrar preview do comando |
| `g` | Gerar payload |
| `c` | Limpar output |

---

## 8. Sessions (Aba 4)

A aba de sessões lista todas as sessões ativas (meterpreter/shell) e permite gerenciá-las.

### Como usar

As sessões são carregadas automaticamente sempre que você alterna para esta aba.

**Passo 1 — Navegar:**
- Use `↑` e `↓` para selecionar uma sessão na lista
- A lista mostra: ID, Tipo, Alvo e Status

**Passo 2 — Ver detalhes:**
- Pressione `Enter` para ver informações detalhadas da sessão selecionada
- Pressione `Enter` ou `Esc` para voltar

**Passo 3 — Gerenciar:**
- `k` — **Kill**: encerra a sessão selecionada (executa `sessions -k <id>`)
- `u` — **Upgrade**: tenta elevar a sessão para meterpreter (executa `sessions -u <id>`)

### Teclas

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar sessões |
| `Enter` | Ver detalhes da sessão |
| `k` | Encerrar (kill) sessão |
| `u` | Upgrade para meterpreter |

---

## 9. Console (Aba 5)

O console embutido permite executar comandos diretamente, com suporte a comandos locais e modo msfconsole.

### Estrutura da tela

```
┌─ Console [local] ─────────────────────────────┐
│ Metasploit TUI Console v0.1                    │
│ Type commands below. Press Ctrl+D to start     │
│ msfconsole.                                    │
│ ---                                            │
│ > help                                         │
│ Available commands:                            │
│   help        - Show this help                 │
│   clear/cls   - Clear console                  │
│   version     - Show version                   │
│   exit/quit   - Close console                  │
│ ---                                            │
└────────────────────────────────────────────────┘
┌─ Input ────────────────────────────────────────┐
│                                                │
└────────────────────────────────────────────────┘
```

### Como usar

**Modo local** (padrão):
- Digite comandos e pressione `Enter` para executar
- Comandos disponíveis: `help`, `clear`/`cls`, `version`, `exit`/`quit`
- Use `↑`/`↓` para navegar pelo histórico de comandos
- Use `PageUp`/`PageDown` para rolar o output

**Modo msfconsole:**
- Pressione `Ctrl+D` para ativar/desativar o modo msfconsole
- Quando ativo, o título muda para `Console [msfconsole]`
- Neste modo, qualquer comando digitado é enviado ao `msfconsole` via `-x "<cmd>; exit"`
- Pressione `Ctrl+C` para interromper um comando em execução
- Para sair do modo, pressione `Ctrl+D` novamente

### Teclas

| Tecla | Ação |
|---|---|
| `Enter` | Executar comando |
| `↑/↓` | Navegar histórico |
| `Ctrl+D` | Alternar modo msfconsole |
| `Ctrl+C` | Interromper comando |
| `PageUp` | Rolar output para cima |
| `PageDown` | Rolar output para baixo |
| `Backspace` | Apagar caractere anterior |
| `Delete` | Apagar caractere sob o cursor |
| `←/→`/`Home`/`End` | Navegar no input |

---

## 10. Scripts (Aba 6)

A aba de scripts permite navegar e executar resource scripts (arquivos `.rc`) do Metasploit.

### Estrutura da tela

```
┌─ Resource Scripts ─┐ ┌─ Actions ──────────────┐
│ Name              S│ │ [Enter] View content   │
│                   iz│ │ [r] Run script         │
│                   e │ │ [↑/↓] Navigate        │
│ autoexploit.rc  128│ │                        │
│ persist.rc       56│ │ Resource scripts (.rc) │
│                    │ │ contain Metasploit      │
│                    │ │ commands executed       │
│                    │ │ sequentially.           │
└────────────────────┘ └────────────────────────┘
```

### Como usar

O sistema varre automaticamente os diretórios `./` e `./scripts/` em busca de arquivos `.rc`.

**Passo 1 — Navegar:**
- Use `↑` e `↓` para selecionar um script

**Passo 2 — Ver conteúdo:**
- Pressione `Enter` para ler o conteúdo do script
- Pressione `Enter` ou `Esc` para voltar à lista

**Passo 3 — Executar:**
- Pressione `r` para executar o script via `msfconsole -q -r <caminho>`

### Teclas

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar scripts |
| `Enter` | Ver conteúdo do script |
| `r` | Executar script |

---

## 11. Referência Rápida de Teclas

### Globais

| Tecla | Ação |
|---|---|
| `1` a `6` | Ir para aba correspondente |
| `Tab` | Próxima aba |
| `Shift+Tab` | Aba anterior |
| `?` | Abrir/fechar ajuda |
| `q` ou `Ctrl+C` | Sair |

### Dashboard

| Tecla | Ação |
|---|---|
| `h` | Executar health check |

### Explorer

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar categorias/módulos |
| `←/→` | Alternar foco categorias/módulos |
| `Enter` | Detalhes do módulo |
| `/` | Buscar módulo |
| `i` | Informações completas do módulo |
| `r` | Executar módulo |

### Payload Generator

| Tecla | Ação |
|---|---|
| `Tab`/`↓` | Próximo campo |
| `Shift+Tab`/`↑` | Campo anterior |
| `/` | Editar campo |
| `Enter` | Preview do comando |
| `g` | Gerar payload |
| `c` | Limpar output |

### Sessions

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar sessões |
| `Enter` | Detalhes da sessão |
| `k` | Kill sessão |
| `u` | Upgrade sessão |

### Console

| Tecla | Ação |
|---|---|
| `Enter` | Executar comando |
| `↑/↓` | Histórico |
| `Ctrl+D` | Alternar msfconsole |
| `Ctrl+C` | Interromper |
| `PageUp`/`PageDown` | Rolar output |

### Scripts

| Tecla | Ação |
|---|---|
| `↑/↓` | Navegar scripts |
| `Enter` | Ver conteúdo |
| `r` | Executar script |

---

## 12. Solução de Problemas

### "msfconsole not found" no health check

O Metasploit Framework não está instalado ou não está no `PATH`. Instale-o conforme a seção [Pré-requisitos](#2-pré-requisitos).

### "Database connection failed"

Execute `msfdb init` no terminal para inicializar o banco de dados.

### Módulos não carregam no Explorer

O carregamento dos módulos pode levar alguns segundos, especialmente na primeira execução. Cada categoria é carregada em uma thread separada. O progresso é mostrado na barra de status.

### O console travou em um comando

Comandos no msfconsole têm um **timeout de 30 segundos**. Se o comando demorar mais que isso, o processo é encerrado automaticamente. Pressione `Ctrl+C` para tentar interromper manualmente.

### Terminal quebrou após erro

Se o programa fechar inesperadamente e o terminal ficar em estado estranho, o Metasploit TUI registra um panic hook que restaura o terminal automaticamente. Caso ainda assim ocorra, digite:

```bash
reset
```

### Onde colocar meus scripts `.rc`?

O programa procura por arquivos `.rc` no diretório atual (`./`) e em `./scripts/`. Basta colocar seus scripts em qualquer um desses locais.

---


Dúvidas ou sugestões? Abra uma issue em https://github.com/seu-usuario/metasploit-ui/issues
