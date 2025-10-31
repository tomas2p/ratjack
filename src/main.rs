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
use game::logic::repartir_cartas;
use strategies::parse_strategies;
// rand used inside strategies module; no direct usage here
use crate::auto::simulate;
use crate::cli::{parse_args, Mode};
use game::player::Jugador;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let cfg = parse_args();

    if cfg.show_help {
        crate::cli::print_help();
        return Ok(());
    }

    match cfg.mode {
        Mode::AutoUi => {
            // Configuración de terminal (modo UI necesario para mostrar progreso)
            enable_raw_mode()?;
            execute!(stdout(), EnterAlternateScreen)?;

            let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend)?;
            terminal.clear()?;

            let strategies = parse_strategies(&cfg.strategies_raw);
            // If RATJACK_HEADLESS env var is set, allow running simulation without terminal UI (used by tests)
            if std::env::var("RATJACK_HEADLESS").is_ok() {
                let summary = simulate(cfg.reps, cfg.num_players, strategies);
                crate::auto::print_summary(&summary);
                // restore terminal and return OK
                disable_raw_mode()?;
                execute!(io::stdout(), LeaveAlternateScreen)?;
                return Ok(());
            }

            let res = ui::run_auto_ui(&mut terminal, cfg.reps, cfg.num_players, strategies);

            // Restaurar terminal
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;

            return res;
        }
        Mode::Ui => {
            // continue to interactive UI below
        }
    }

    // Configuración de terminal (modo interactivo)
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Inicialización del juego
    let mut baraja = crear_baraja();
    let mut jugador = Jugador::nuevo();
    let mut banca = Jugador::nuevo();

    // Repartir cartas iniciales
    repartir_cartas(&mut jugador, &mut banca, &mut baraja);
    // Antes de ejecutar la UI, usar la configuración parseada por cli
    let ui_strategies = parse_strategies(&cfg.ui_str_raw);
    let label_j = ui_strategies
        .get(0)
        .map(|s| strategies::strategy_label(s))
        .unwrap_or_default();
    let label_b = ui_strategies
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
    // Pasar referencias a strings; los Strings deben vivir hasta que run_game termine
    let lj_ref = lj_opt.as_ref().map(|s| s.as_str());
    let lb_ref = lb_opt.as_ref().map(|s| s.as_str());

    let result = ui::run_game(
        &mut terminal,
        &mut jugador,
        &mut banca,
        &mut baraja,
        lj_ref,
        lb_ref,
    );

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

// run_auto was moved to `auto::simulate`; keep main.rs focused on wiring and UI.
