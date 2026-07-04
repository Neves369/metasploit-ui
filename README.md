# Metasploit TUI

Uma interface de terminal (TUI) para o Metasploit Framework, construída em Rust com [ratatui](https://github.com/ratatui-org/ratatui) e [crossterm](https://github.com/crossterm-rs/crossterm).

## Funcionalidades

| Aba         | Descrição |
|-------------|-----------|
| Dashboard   | Banner MSF estilizado, health check completo (msfconsole, msfvenom, Ruby, DB, módulos) |
| Explorer    | Navegação e busca de módulos (exploit, auxiliary, payload, post, encoder, nop, evasion) com detalhes |
| Payload     | Gerador de payloads via `msfvenom` — configure payload, LHOST, LPORT, formato, encoder, output |
| Sessions    | Listagem e gerenciamento de sessões ativas (meterpreter/shell) com visualização de detalhes |
| Console     | Terminal embutido com histórico, comandos locais e integração com `msfconsole` |
| Scripts     | Navegação e execução de resource scripts (`.rc`) |

## Instalação

### 1. Instalar Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Instalar Metasploit Framework

**Kali Linux:**
```bash
sudo apt update && sudo apt install -y metasploit-framework
```

**Debian/Ubuntu:**
```bash
curl https://raw.githubusercontent.com/rapid7/metasploit-omnibus/master/config/templates/metasploit-framework-wrappers/msfupdate.erb > msfinstall
chmod +x msfinstall
sudo ./msfinstall
```

**Arch Linux:**
```bash
yay -S metasploit
```

**Fedora/RHEL:**
```bash
sudo dnf install -y ruby rubygem-bundler
git clone https://github.com/rapid7/metasploit-framework.git
cd metasploit-framework
sudo bundle install
```

### 3. Clonar e compilar

```bash
git clone https://github.com/seu-usuario/metasploit-ui.git
cd metasploit-ui
cargo build --release
./target/release/metasploit-tui
```

Para rodar direto sem compilar separado:
```bash
cargo run --release
```

### Verificar instalação

Na aba **Dashboard** (aba 1), pressione `h` para executar o health check completo. O sistema verificará:

- ✅ `msfconsole` — instalado e versão
- ✅ `msfvenom` — instalado e versão
- ✅ `ruby` — versão do interpretador
- ✅ `database` — status da conexão com banco
- ✅ módulos — contagem real por categoria

## Como usar

### Navegação entre abas

| Tecla          | Ação               |
|----------------|--------------------|
| `1` a `6`      | Ir para aba        |
| `Tab`          | Próxima aba        |
| `Shift+Tab`    | Aba anterior       |
| `?`            | Ajuda              |
| `q` / `Ctrl+C` | Sair               |

### Teclas da Dashboard

| Tecla | Ação                    |
|-------|-------------------------|
| `h`   | Executar health check   |

### Explorer

| Tecla       | Ação                    |
|-------------|-------------------------|
| `↑/↓`       | Navegar categorias      |
| `←/→`       | Navegar módulos         |
| `Enter`     | Ver detalhes do módulo  |
| `/`         | Buscar módulos          |

### Payload Generator

| Tecla          | Ação               |
|----------------|--------------------|
| `Tab` / `↓`    | Próximo campo      |
| `Shift+Tab` / `↑` | Campo anterior  |
| `/`            | Editar campo       |
| `Enter`        | Mostrar preview    |
| `c`            | Limpar output      |

### Sessions

| Tecla   | Ação                |
|---------|---------------------|
| `↑/↓`   | Navegar sessões     |
| `Enter` | Ver detalhes        |
| `k`     | Kill sessão         |

### Console

| Tecla              | Ação                     |
|--------------------|--------------------------|
| `Ctrl+D`           | Alternar msfconsole      |
| `↑/↓`              | Histórico de comandos    |
| `PageUp`/`PageDown`| Rolar output             |
| `help`             | Ver comandos disponíveis |

## Estrutura do projeto

```
src/
├── main.rs                  # Entry point, setup do terminal raw mode
├── app.rs                   # Loop principal, teclas globais, renderização
├── components/
│   ├── input.rs             # Input field reutilizável
│   ├── list.rs              # Lista scrollável reutilizável
│   ├── status_bar.rs        # Barra de status inferior
│   └── tab_bar.rs           # Barra de abas superior
├── msf/
│   ├── msfconsole.rs        # Wrapper para comandos do msfconsole
│   ├── msfvenom.rs          # Wrapper para geração de payloads
│   ├── parser.rs            # Parsers de output do Metasploit
│   └── runner.rs            # Execução de processos + health check
└── ui/
    ├── console_tab.rs       # Aba do console
    ├── dashboard.rs         # Aba do dashboard (banner, health, módulos)
    ├── explorer.rs          # Aba do explorador de módulos
    ├── payload_gen.rs       # Aba do gerador de payloads
    ├── resources.rs         # Aba de resource scripts
    └── sessions.rs          # Aba de sessões
```

## Tecnologias

- [Rust](https://www.rust-lang.org/) — linguagem
- [ratatui](https://github.com/ratatui-org/ratatui) — framework TUI
- [crossterm](https://github.com/crossterm-rs/crossterm) — manipulação de terminal
- [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) — serialização
- [chrono](https://github.com/chronotope/chrono) — data e hora

## Licença

MIT
