use crate::auto::{run_simulation, summary_lines, Summary};
use crate::game::{
    deck::Carta,
    logic::{determinar_ganador, jugar_turno, repartir_cartas},
    player::Jugador,
};
use crate::strategies::Strategy;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
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
    pub alg_jugador: String,
    pub alg_banca: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            estado: GameState::Inicio,
            mensaje: String::from("¡Bienvenido a RatJack!"),
            opciones: vec![String::from("Pedir carta"), String::from("Plantarse")],
            seleccion: 0,
            mostrar_todas_cartas_banca: false,
            alg_jugador: String::new(),
            alg_banca: String::new(),
        }
    }

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
    alg_jugador: Option<&str>,
    alg_banca: Option<&str>,
) -> io::Result<()> {
    let mut app = AppState::new();
    app.alg_jugador = alg_jugador.unwrap_or("").to_string();
    app.alg_banca = alg_banca.unwrap_or("").to_string();

    if jugador.puntos() == 21 {
        app.mensaje = "¡21! Turno de la banca automático".to_string();
        app.estado = GameState::TurnoBanca;
    }

    loop {
        if matches!(app.estado, GameState::TurnoBanca) {
            terminal.draw(|frame| render_ui(frame, jugador, banca, &app))?;
            while banca.puntos() < 17 {
                jugar_turno(banca, baraja, true);
            }
            app.mensaje = determinar_ganador(jugador, banca);
            app.estado = GameState::FinJuego;
            app.actualizar_opciones();
        }

        terminal.draw(|frame| render_ui(frame, jugador, banca, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.estado {
                    GameState::Inicio => {
                        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                            app.estado = GameState::TurnoJugador;
                            app.actualizar_opciones();
                            if jugador.puntos() == 21 {
                                app.mensaje = "¡21! Turno de la banca automático".to_string();
                                app.estado = GameState::TurnoBanca;
                            }
                        }
                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                    }
                    GameState::TurnoJugador => match key.code {
                        KeyCode::Enter | KeyCode::Char('1') | KeyCode::Char('p') => {
                            jugar_turno(jugador, baraja, true);
                            if jugador.puntos() > 21 {
                                app.mensaje = determinar_ganador(jugador, banca);
                                app.estado = GameState::FinJuego;
                                app.actualizar_opciones();
                            } else if jugador.puntos() == 21 {
                                app.mensaje = "¡21! Turno de la banca automático".to_string();
                                app.estado = GameState::TurnoBanca;
                            }
                        }
                        KeyCode::Char('2') | KeyCode::Char('s') => {
                            app.estado = GameState::TurnoBanca;
                            app.mensaje = "Turno de la banca".to_string();
                        }
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    GameState::FinJuego => match key.code {
                        KeyCode::Enter | KeyCode::Char('n') => {
                            reiniciar_partida(jugador, banca, baraja);
                            app.mensaje = "¡Nueva partida!".to_string();
                            app.estado = GameState::TurnoJugador;
                            app.actualizar_opciones();
                            if jugador.puntos() == 21 {
                                app.mensaje = "¡21! Turno de la banca automático".to_string();
                                app.estado = GameState::TurnoBanca;
                            }
                        }
                        KeyCode::Char('q') => return Ok(()),
                        _ => {}
                    },
                    _ => {
                        if key.code == KeyCode::Char('q') {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}

pub fn run_auto_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    reps: u32,
    num_players: usize,
    strategies: Vec<Strategy>,
) -> io::Result<()> {
    let on_progress = |iter: u32, summary: &Summary| {
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(1),
                ])
                .split(f.size());

            render_header(f, chunks[0]);

            let mut lines = Vec::new();
            lines.push(format!("Simulación en progreso: {}/{}", iter, reps));
            lines.push(String::new());
            lines.extend(summary_lines(summary).into_iter().skip(2)); // Skip title/empty line

            let p = Paragraph::new(lines.join("\n")).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Progreso de Simulación"),
            );
            f.render_widget(p, chunks[1]);

            render_footer(f, chunks[2], "Simulando... Espere por favor.");
        });
    };

    let summary = run_simulation(reps, num_players, strategies, on_progress);

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(f.size());

        render_header(f, chunks[0]);
        let lines = summary_lines(&summary);
        let p = Paragraph::new(lines.join("\n")).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Resumen de Simulación"),
        );
        f.render_widget(p, chunks[1]);
        render_footer(f, chunks[2], "Presione cualquier tecla para salir");
    })?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    Ok(())
}

fn reiniciar_partida(jugador: &mut Jugador, banca: &mut Jugador, baraja: &mut Vec<Carta>) {
    *baraja = crate::game::deck::crear_baraja();
    repartir_cartas(jugador, banca, baraja);
}

// --- Componentes de Renderizado ---

fn render_header(frame: &mut Frame, area: Rect) {
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
    frame.render_widget(titulo, area);
}

fn render_footer(frame: &mut Frame, area: Rect, text: &str) {
    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

fn render_ui(frame: &mut Frame, jugador: &Jugador, banca: &Jugador, app: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(frame.size());

    render_header(frame, main_chunks[0]);

    let mensaje = Paragraph::new(app.mensaje.clone())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    frame.render_widget(mensaje, main_chunks[1]);

    let mesa_chunks = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .vertical_margin(1)
        .spacing(2)
        .split(main_chunks[2]);

    render_player_panel(
        frame,
        mesa_chunks[0],
        "Banca",
        banca,
        app.mostrar_todas_cartas_banca,
        Color::Yellow,
        &app.alg_banca,
    );
    render_player_panel(
        frame,
        mesa_chunks[1],
        "Jugador",
        jugador,
        true,
        Color::Blue,
        &app.alg_jugador,
    );

    let footer_text = match app.estado {
        GameState::Inicio => "↵:Comenzar | q:Salir",
        GameState::TurnoJugador => "↵/1/p:Pedir | 2/s:Plantarse | q:Salir",
        GameState::FinJuego => "↵/n:Nueva Partida | q:Salir",
        _ => "q:Salir",
    };
    render_footer(frame, main_chunks[3], footer_text);
}

fn render_player_panel(
    frame: &mut Frame,
    area: Rect,
    nombre: &str,
    jugador: &Jugador,
    mostrar_todas: bool,
    color: Color,
    alg: &str,
) {
    let mut lines = Vec::new();

    if jugador.mano.is_empty() {
        lines.push(Line::from("[Sin cartas]"));
    } else {
        // Renderizado simplificado de cartas para ahorrar espacio y mejorar legibilidad
        let mut card_row = Vec::new();
        for (i, carta) in jugador.mano.iter().enumerate() {
            if i == 0 || mostrar_todas {
                let symbol = carta.simbolo();
                let val = carta.valor_str();
                let style = if matches!(
                    carta.palo,
                    crate::game::deck::Palo::Corazones | crate::game::deck::Palo::Diamantes
                ) {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::White)
                };
                card_row.push(Span::styled(format!("[{} {}] ", val, symbol), style));
            } else {
                card_row.push(Span::styled("[? ?] ", Style::default().fg(Color::Yellow)));
            }
        }
        lines.push(Line::from(card_row));
    }

    let puntos_str = if mostrar_todas {
        jugador.puntos().to_string()
    } else {
        "?".to_string()
    };
    let info = if alg.is_empty() {
        format!(
            "Puntos: {} | Ganadas: {}",
            puntos_str, jugador.partidas_ganadas
        )
    } else {
        format!(
            "Puntos: {} | Ganadas: {} | Alg: {}",
            puntos_str, jugador.partidas_ganadas, alg
        )
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(nombre)
        .border_style(Style::default().fg(color))
        .border_type(BorderType::Rounded)
        .title_bottom(Line::from(info).alignment(Alignment::Center));

    let p = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    frame.render_widget(p, area);
}
