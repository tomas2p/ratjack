mod auto;
mod cli;
mod game;
mod strategies;
mod ui;

use crate::auto::simulate;
use crate::cli::{parse_args, Mode};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::deck::crear_baraja;
use game::logic::Mesa;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};
use strategies::parse_strategies;

/// Asegura que el terminal se restaure al salir del alcance.
struct TerminalGuard;

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

fn main() -> io::Result<()> {
    let cfg = parse_args();

    if cfg.show_help {
        crate::cli::print_help();
        return Ok(());
    }

    match cfg.mode {
        Mode::AutoUi => {
            let _guard = TerminalGuard::new()?;
            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;
            terminal.clear()?;

            let strategies = parse_strategies(&cfg.strategies_raw);

            if std::env::var("RATJACK_HEADLESS").is_ok() {
                let summary = simulate(cfg.reps, cfg.num_players, strategies);
                crate::auto::print_summary(&summary);
                return Ok(());
            }

            ui::run_auto_ui(&mut terminal, cfg.reps, cfg.num_players, strategies)
        }
        Mode::Ui => {
            let _guard = TerminalGuard::new()?;
            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;
            terminal.clear()?;

            let mut mesa = Mesa::nueva(crear_baraja());
            mesa.repartir_inicial();

            let ui_strategies = parse_strategies(&cfg.ui_str_raw);
            let label_b = ui_strategies.get(0).map(|s| strategies::strategy_label(s));
            let label_j = ui_strategies.get(1).map(|s| strategies::strategy_label(s));

            ui::run_game(
                &mut terminal,
                &mut mesa.jugador,
                &mut mesa.banca,
                &mut mesa.baraja,
                label_j.as_deref(),
                label_b.as_deref(),
            )
        }
    }
}
