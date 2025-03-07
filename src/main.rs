mod game;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::deck::crear_baraja;
use game::logic::repartir_cartas;
use game::player::Jugador;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // Configuración de terminal
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

    // Ejecutar juego con la UI de ratatui
    let result = ui::run_game(&mut terminal, &mut jugador, &mut banca, &mut baraja);

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}
