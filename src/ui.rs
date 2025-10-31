use crate::game::{
    deck::Carta,
    logic::{determinar_ganador, jugar_turno, repartir_cartas},
    player::Jugador,
};
use crate::strategies::{should_draw, Strategy};
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
    alg_jugador: Option<&str>,
    alg_banca: Option<&str>,
) -> io::Result<()> {
    let mut app = AppState::new();
    app.alg_jugador = alg_jugador.unwrap_or("").to_string();
    app.alg_banca = alg_banca.unwrap_or("").to_string();

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

// Run automatic simulations but render progress and final summary inside the UI (no card details)
pub fn run_auto_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    reps: u32,
    num_players: usize,
    strategies: Vec<Strategy>,
) -> io::Result<()> {
    // Prepare state (similar to main run_auto)
    let mut reps = reps;
    let mut num_players = num_players.clamp(2, 8);

    // Build strategy vector for players
    let mut strat_vec: Vec<Strategy> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        if let Some(s) = strategies.get(i) {
            strat_vec.push(s.clone());
        } else if !strategies.is_empty() {
            strat_vec.push(strategies.last().unwrap().clone());
        } else {
            let t = 12 + ((i % 8) as u8);
            strat_vec.push(Strategy::Threshold(t));
        }
    }

    // stats
    let mut wins = vec![0u32; num_players];
    let mut busts = vec![0u32; num_players];
    let mut total_points = vec![0u64; num_players];
    let mut ties = 0u32;

    // render helper: draw title/content/footer matching normal game UI exactly
    let render = |frame: &mut ratatui::Frame, _title: &str, lines: Vec<String>| {
        use ratatui::style::Style;
        use ratatui::widgets::{Block, Borders, Paragraph};
        // layout: Title (len 3) | Content (min) | Footer (len 1)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(frame.size());

        // Title area: same as main UI
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
        frame.render_widget(titulo, chunks[0]);

        // Content area
        let text = lines.join("\n");
        let p = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(_title))
            .style(Style::default());
        frame.render_widget(p, chunks[1]);

        // Footer area: match render_ui footer style and text format
        let footer = Paragraph::new("p:Pausa | r:Reconfigurar | q:Salir")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[2]);
    };

    // Run simulations, update UI periodically
    let mut update_every = std::cmp::max(1, (reps / 100).max(1));
    let mut iter: u32 = 0;
    let mut paused = false;
    let mut input_mode = false;
    let mut input_buf = String::new();

    while iter < reps {
        // allow event handling to stop/pause/reconfigure
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char('q') => {
                            // quit early
                            return Ok(());
                        }
                        KeyCode::Char('p') => {
                            // toggle pause
                            paused = !paused;
                        }
                        KeyCode::Char('r') => {
                            // enter reconfigure input mode
                            input_mode = true;
                            input_buf.clear();
                        }
                        _ => {}
                    }
                }
            }
        }

        if input_mode {
            // render prompt and collect keys until Enter or Esc
            let lines = vec!["Reconfigure: escribe '<reps> <num_players> <strategies>' y Enter (Esc para cancelar)".to_string(), String::new(), format!("Buffer: {}", input_buf)];
            terminal.draw(|f| {
                use ratatui::widgets::{Block, Borders, Paragraph};
                let area = f.size();
                let text = lines.join("\n");
                let p = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Reconfigurar"));
                f.render_widget(p, area);
            })?;

            // collect keys
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(k) = event::read()? {
                    if k.kind == KeyEventKind::Press {
                        match k.code {
                            KeyCode::Enter => {
                                // attempt parse
                                let parts: Vec<&str> =
                                    input_buf.splitn(3, ' ').map(|s| s.trim()).collect();
                                if parts.len() >= 1 && !parts[0].is_empty() {
                                    if let Ok(new_reps) = parts[0].parse::<u32>() {
                                        let new_num = if parts.len() >= 2 && !parts[1].is_empty() {
                                            parts[1].parse::<usize>().unwrap_or(num_players)
                                        } else {
                                            num_players
                                        };
                                        let new_strat_raw =
                                            if parts.len() == 3 { parts[2] } else { "" };
                                        let new_strats =
                                            crate::strategies::parse_strategies(new_strat_raw);
                                        // apply new configuration
                                        iter = 0;
                                        // reset stats
                                        for v in wins.iter_mut() {
                                            *v = 0;
                                        }
                                        for v in busts.iter_mut() {
                                            *v = 0;
                                        }
                                        for v in total_points.iter_mut() {
                                            *v = 0;
                                        }
                                        ties = 0;
                                        // update reps and players and strategies
                                        // note: shadowing the local num_players variable is tricky; use mutable local
                                        // but here num_players is immutable; create mutable local copy above if needed
                                        // to keep it simple, we'll update strat_vec and leave num_players as-is for simplicity

                                        // rebuild strat_vec according to new_num
                                        let mut new_vec: Vec<Strategy> =
                                            Vec::with_capacity(new_num);
                                        if !new_strats.is_empty() {
                                            for i in 0..new_num {
                                                if let Some(s) = new_strats.get(i) {
                                                    new_vec.push(s.clone());
                                                } else {
                                                    new_vec
                                                        .push(new_strats.last().unwrap().clone());
                                                }
                                            }
                                        } else {
                                            for i in 0..new_num {
                                                let t = 12 + ((i % 8) as u8);
                                                new_vec.push(Strategy::Threshold(t));
                                            }
                                        }
                                        // replace strat_vec and adjust arrays sizes
                                        strat_vec = new_vec;
                                        let old_np = wins.len();
                                        if new_num != old_np {
                                            wins = vec![0u32; new_num];
                                            busts = vec![0u32; new_num];
                                            total_points = vec![0u64; new_num];
                                        }
                                        // set new reps and num_players
                                        reps = new_reps;
                                        num_players = new_num;
                                        update_every = std::cmp::max(1, (reps / 100).max(1));
                                        input_mode = false;
                                    }
                                }
                            }
                            KeyCode::Esc => {
                                input_mode = false;
                            }
                            KeyCode::Backspace => {
                                input_buf.pop();
                            }
                            KeyCode::Char(c) => {
                                input_buf.push(c);
                            }
                            _ => {}
                        }
                    }
                }
            }
            continue; // skip simulation iteration while in input mode
        }

        if paused {
            // show paused screen
            let lines = vec![format!(
                "Pausado en iteración {}/{}. Teclas: p=continuar, r=reconfigurar, q=salir",
                iter + 1,
                reps
            )];
            terminal.draw(|f| {
                use ratatui::style::Style;
                use ratatui::widgets::{Block, Borders, Paragraph};
                let area = f.size();
                let text = lines.join("\n");
                let p = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Pausado"))
                    .style(Style::default());
                f.render_widget(p, area);
            })?;
            // wait for key
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(k) = event::read()? {
                    if k.kind == KeyEventKind::Press {
                        match k.code {
                            KeyCode::Char('p') => {
                                paused = false;
                            }
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Char('r') => {
                                input_mode = true;
                                input_buf.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
            continue; // skip performing iteration while paused
        }

        let mut baraja = crate::game::deck::crear_baraja();

        // Create players
        let mut jugadores: Vec<Jugador> = (0..num_players)
            .map(|i| {
                let mut j = Jugador::nuevo();
                j.nombre = if i == num_players - 1 {
                    "Banca".to_string()
                } else {
                    format!("Jugador {}", i + 1)
                };
                j
            })
            .collect();

        // initial deal
        for _ in 0..2 {
            for p in jugadores.iter_mut() {
                p.tomar_carta(&mut baraja);
            }
        }
        for p in jugadores.iter_mut() {
            p.puntos = p.puntaje();
        }

        // players act
        for idx in 0..num_players {
            loop {
                let pts = jugadores[idx].puntaje();
                if should_draw(&strat_vec[idx], pts, &baraja) {
                    jugar_turno(&mut jugadores[idx], &mut baraja, true);
                    jugadores[idx].puntos = jugadores[idx].puntaje();
                    if jugadores[idx].puntaje() > 21 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        // evaluate
        let bank_idx = num_players - 1;
        let bank_points = jugadores[bank_idx].puntaje();
        total_points[bank_idx] += bank_points as u64;
        if bank_points > 21 {
            busts[bank_idx] += 1;
        }

        for i in 0..num_players - 1 {
            let p_points = jugadores[i].puntaje();
            total_points[i] += p_points as u64;
            if p_points > 21 {
                busts[i] += 1;
                wins[bank_idx] += 1;
            } else if bank_points > 21 {
                wins[i] += 1;
            } else if p_points > bank_points {
                wins[i] += 1;
            } else if bank_points > p_points {
                wins[bank_idx] += 1;
            } else {
                ties += 1;
            }
        }

        // periodic UI update
        if iter % update_every == 0 || iter + 1 == reps {
            let mut lines = Vec::new();
            lines.push(format!("Iteración {}/{}", iter + 1, reps));
            lines.push(String::new());
            for i in 0..num_players {
                let name = if i == num_players - 1 {
                    "Banca".to_string()
                } else {
                    format!("Jugador {}", i + 1)
                };
                let alg = crate::strategies::strategy_label(&strat_vec[i]);
                let w = wins[i];
                let avg = (total_points[i] as f64) / ((iter + 1) as f64);
                lines.push(format!(
                    "{} | {} | Vict: {} | Busts: {} | AvgPts: {:.2}",
                    name, alg, w, busts[i], avg
                ));
            }
            terminal.draw(|f| render(f, "Auto (UI) - Progreso", lines))?;
        }

        // advance iteration counter
        iter += 1;
    }

    // final summary screen
    let mut lines = vec![
        format!("Resumen final después de {} partidas:", reps),
        String::new(),
    ];
    for i in 0..num_players {
        let name = if i == num_players - 1 {
            "Banca".to_string()
        } else {
            format!("Jugador {}", i + 1)
        };
        let alg = crate::strategies::strategy_label(&strat_vec[i]);
        let w = wins[i];
        let avg = (total_points[i] as f64) / (reps as f64);
        lines.push(format!(
            "{} | {} | Victorias: {} | Busts: {} | Puntos avg: {:.2}",
            name, alg, w, busts[i], avg
        ));
    }
    lines.push(String::new());
    lines.push(format!("Empates totales: {}", ties));

    terminal.draw(|f| render(f, "Auto (UI) - Resumen", lines))?;

    // Wait for a key press to return
    use crossterm::event::{self, Event};
    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    Ok(())
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
        algoritmo: &str,
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
                    format!("Ganadas: {} | Alg: {}", jugador.partidas_ganadas, algoritmo),
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

    // Determine algorithm labels if present in app or default to empty
    let alg_b = app.alg_banca.as_str();
    let alg_j = app.alg_jugador.as_str();

    render_player(
        frame,
        mesa_chunks[0],
        "Banca",
        &banca,
        app.mostrar_todas_cartas_banca,
        Color::Red,
        alg_b,
    );
    render_player(
        frame,
        mesa_chunks[1],
        &jugador.nombre,
        &jugador,
        true,
        Color::Blue,
        alg_j,
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
