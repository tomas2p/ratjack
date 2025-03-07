use crate::game::{
    deck::Carta,
    logic::{determinar_ganador, jugar_turno, repartir_cartas},
    player::Jugador,
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use std::io;

pub enum GameState {
    Inicio,
    TurnoJugador,
    TurnoBanca,
    FinJuego,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Inicio
    }
}

#[derive(Default)]
pub struct AppState {
    pub estado: GameState,
    pub mensaje: String,
    pub opciones: Vec<String>,
    pub seleccion: usize,
    pub mostrar_todas_cartas_banca: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            estado: GameState::Inicio,
            mensaje: String::from("¡Bienvenido a RatJack!"),
            opciones: vec![String::from("Pedir carta"), String::from("Plantarse")],
            seleccion: 0,
            mostrar_todas_cartas_banca: false,
        }
    }

    // Método para actualizar las opciones según el estado del juego
    pub fn actualizar_opciones(&mut self) {
        match self.estado {
            GameState::TurnoJugador => {
                self.opciones = vec![String::from("Pedir carta"), String::from("Plantarse")];
                self.mostrar_todas_cartas_banca = false;
            }
            GameState::FinJuego => {
                self.opciones = vec![String::from("Nueva partida"), String::from("Salir")];
                self.mostrar_todas_cartas_banca = true;
            }
            _ => {}
        }
        self.seleccion = 0;
    }
}

pub fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    jugador: &mut Jugador,
    banca: &mut Jugador,
    baraja: &mut Vec<Carta>,
) -> io::Result<()> {
    let mut app = AppState::new();

    loop {
        terminal.draw(|frame| render_ui(frame, jugador, banca, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.estado {
                    GameState::Inicio => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            app.estado = GameState::TurnoJugador;
                            app.actualizar_opciones();
                        }
                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                    }
                    GameState::TurnoJugador => {
                        match key.code {
                            KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char('p') => {
                                // Pedir carta - atajo de teclado
                                jugar_turno(jugador, baraja, true);
                                jugador.puntos = jugador.puntaje();
                                if jugador.puntos > 21 {
                                    app.mensaje = determinar_ganador(jugador, banca);
                                    app.estado = GameState::FinJuego;
                                    app.actualizar_opciones();
                                }
                            }
                            KeyCode::Char('2') | KeyCode::Char('s') => {
                                // Plantarse - atajo de teclado
                                app.estado = GameState::TurnoBanca;
                                app.mensaje = "Turno de la banca".to_string();
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                    GameState::TurnoBanca => {
                        // La banca juega automáticamente
                        while banca.puntaje() < 17 {
                            jugar_turno(banca, baraja, true);
                            banca.puntos = banca.puntaje();
                        }

                        // Determinar ganador
                        let resultado = determinar_ganador(jugador, banca);
                        app.mensaje = resultado;
                        app.estado = GameState::FinJuego;
                        app.actualizar_opciones();

                        // Simplemente mostrar el resultado y esperar entrada del usuario
                        terminal.draw(|frame| render_ui(frame, jugador, banca, &app))?;

                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                    }
                    GameState::FinJuego => {
                        match key.code {
                            KeyCode::Enter | KeyCode::Char('n') => {
                                // Nueva partida - atajo de teclado
                                reiniciar_partida(jugador, banca, baraja);
                                app.mensaje = "¡Nueva partida!".to_string();
                                app.estado = GameState::TurnoJugador;
                                app.actualizar_opciones();
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// Función para reiniciar la partida
fn reiniciar_partida(jugador: &mut Jugador, banca: &mut Jugador, baraja: &mut Vec<Carta>) {
    jugador.mano.clear();
    banca.mano.clear();
    jugador.puntos = 0;
    banca.puntos = 0;

    *baraja = crate::game::deck::crear_baraja();
    repartir_cartas(jugador, banca, baraja);
}

fn render_ui(frame: &mut ratatui::Frame, jugador: &Jugador, banca: &Jugador, app: &AppState) {
    // Function to render a player
    fn render_player(
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        nombre: &str,
        jugador: &Jugador,
        mostrar_todas_cartas: bool,
        color: Color,
    ) {
        // Create a string representation of cards
        let mut mano = String::new();

        if jugador.mano.is_empty() {
            mano = "[Sin cartas]".to_string();
        } else {
            // Define how many cards to show
            let cards_to_show = if nombre == "Banca" && !mostrar_todas_cartas {
                1 // Only show first card for the bank when hidden
            } else {
                jugador.mano.len() // Show all cards otherwise
            };

            // Add visible cards
            for i in 0..cards_to_show {
                let carta = &jugador.mano[i];
                let carta_str = format!(
                    "\n╭─────╮\n│{:^5}│\n│{:^5}│\n│{:^5}│\n╰─────╯\n",
                    carta.valor_str(),
                    carta.simbolo(),
                    carta.valor_str()
                );
                mano.push_str(&carta_str);
                mano.push(' ');
            }

            // Add hidden cards for the bank
            if nombre == "Banca" && !mostrar_todas_cartas && jugador.mano.len() > 1 {
                mano.push_str(&format!("\n+ ocultas"));
            }
        }

        let puntos = if mostrar_todas_cartas {
            jugador.puntos.to_string()
        } else {
            "?".to_string()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title_top(Span::styled(nombre, Style::default().fg(Color::White)))
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(color))
            .title_top(
                Span::styled(
                    format!("PTS: {}", puntos),
                    Style::default().fg(Color::White),
                )
                .into_right_aligned_line(),
            )
            .title_bottom(
                Span::styled(
                    format!("Ganadas: {}", jugador.partidas_ganadas),
                    Style::default().fg(Color::White),
                )
                .into_centered_line(),
            );

        let widget = Paragraph::new(mano)
            .style(Style::default().fg(Color::White))
            .block(block)
            .centered();
        frame.render_widget(widget, area);
    }

    // Main vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Título
            Constraint::Length(1), // Mensaje
            Constraint::Min(3),    // Mesa (Banca y Jugador horizontalmente)
            Constraint::Length(1), // Footer
        ])
        .split(frame.size());

    // Horizontal layout for Banca y Jugador
    let mesa_chunks = Layout::horizontal([Constraint::Min(10), Constraint::Min(10)])
        .vertical_margin(2)
        .spacing(3)
        .split(main_chunks[2]);

    // Título
    let titulo = Paragraph::new("♤ ♡ RATJACK ♢ ♧")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    frame.render_widget(titulo, main_chunks[0]);

    // Mensaje
    let mensaje = Paragraph::new(app.mensaje.clone())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    frame.render_widget(mensaje, main_chunks[1]);

    render_player(
        frame,
        mesa_chunks[0],
        "Banca",
        &banca,
        app.mostrar_todas_cartas_banca,
        Color::Red,
    );
    render_player(
        frame,
        mesa_chunks[1],
        &jugador.nombre,
        &jugador,
        true,
        Color::Blue,
    );

    // Footer con todos los comandos disponibles
    let footer_text = match app.estado {
        GameState::Inicio => "↵:Comenzar | q:Salir",
        GameState::TurnoJugador => "↵/1/p:Pedir | 2/s:Plantarse | q:Salir",
        GameState::FinJuego => "↵/n:Nueva Partida | q:Salir",
        _ => "q:Salir",
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(footer, main_chunks[3]);
}
