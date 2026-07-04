# Metasploit TUI

Uma interface de terminal (TUI) para o Metasploit Framework, construída em Rust com [ratatui](https://github.com/ratatui-org/ratatui) e [crossterm](https://github.com/crossterm-rs/crossterm).

## Funcionalidades

| Aba         | Descrição |
|-------------|-----------|
| Dashboard   | Status da instalação, versão do msfconsole, contagem de módulos e ações rápidas |
| Explorer    | Navegação e busca de módulos (exploit, auxiliary, payload, post, encoder, nop, evasion) com detalhes |
| Payload     | Gerador de payloads via `msfvenom` — configure payload, LHOST, LPORT, formato, encoder, output |
| Sessions    | Listagem e gerenciamento de sessões ativas (meterpreter/shell) com visualização de detalhes |
| Console     | Terminal embutido com histórico, comandos locais e integração com `msfconsole` |
| Scripts     | Navegação e execução de resource scripts (`.rc`) |

## Instalação

```bash
git clone https://github.com/seu-usuario/metasploit-ui.git
cd metasploit-ui
cargo run
```

### Dependências

- [Rust](https://www.rust-lang.org/) (edition 2021)
- [Metasploit Framework](https://www.metasploit.com/) — necessário para funcionalidades completas

## Como usar

### Navegação entre abas

| Tecla          | Ação               |
|----------------|--------------------|
| `1` a `6`      | Ir para aba        |
| `Tab`          | Próxima aba        |
| `Shift+Tab`    | Aba anterior       |
| `?`            | Ajuda              |
| `q` / `Ctrl+C` | Sair               |

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
│   └── runner.rs            # Execução de processos do sistema
└── ui/
    ├── console_tab.rs       # Aba do console
    ├── dashboard.rs         # Aba do dashboard
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
