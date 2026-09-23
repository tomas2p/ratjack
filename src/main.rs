mod auto;
mod cli;
mod game;
mod strategies;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::deck::crear_baraja;
use game::game::Game;
use strategies::parse_strategies;
// rand used inside strategies module; no direct usage here
use crate::auto::simulate;
use crate::cli::{parse_args, Mode};
use game::player::Jugador;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let cfg = parse_args();

    if cfg.show_help {
        crate::cli::print_help();
        return Ok(());
    }
    match cfg.mode {
        Mode::AutoUi => {
            let strategies = parse_strategies(&cfg.strategies_raw);

            // If RATJACK_HEADLESS env var is set, allow running simulation without terminal UI (used by tests)
            if std::env::var("RATJACK_HEADLESS").is_ok() {
                let summary = simulate(cfg.reps, cfg.num_players, strategies);
                crate::auto::print_summary(&summary);
                return Ok(());
            }

            let mut terminal = setup_terminal()?;
            let res = ui::run_auto_ui(&mut terminal, cfg.reps, cfg.num_players, strategies);
            restore_terminal()?;
            return res;
        }
        Mode::Ui => {
            let mut terminal = setup_terminal()?;

            // Inicialización del juego
            let baraja = crear_baraja();
            let mut game = Game::nuevo(baraja);

            // Repartir cartas iniciales
            game.repartir_cartas_iniciales();

            // Antes de ejecutar la UI, usar la configuración parseada por cli
            let ui_strategies = parse_strategies(&cfg.ui_str_raw);
            let label_b = ui_strategies
                .get(0)
                .map(|s| strategies::strategy_label(s))
                .unwrap_or_else(|| String::from("Threshold(17)"));
            let label_j = ui_strategies
                .get(1)
                .map(|s| strategies::strategy_label(s))
                .unwrap_or_default();

            // Ejecutar juego con la UI de ratatui (pasar etiquetas si existen)
            let lj_opt = if label_j.is_empty() {
                None
            } else {
                Some(label_j)
            };
            let lb_opt = if label_b.is_empty() {
                None
            } else {
                Some(label_b)
            };

            let lj_ref = lj_opt.as_ref().map(|s| s.as_str());
            let lb_ref = lb_opt.as_ref().map(|s| s.as_str());

            let result = ui::run_game(
                &mut terminal,
                &mut game.jugador,
                &mut game.banca,
                &mut game.baraja,
                lj_ref,
                lb_ref,
            );

            restore_terminal()?;
            result
        }
    }
}

// run_auto was moved to `auto::simulate`; keep main.rs focused on wiring and UI.
