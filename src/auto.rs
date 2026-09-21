use crate::game::deck::crear_baraja;
use crate::game::logic::jugar_turno;
use crate::game::player::Jugador;
use crate::strategies::{should_draw, strategy_label, Strategy};

#[derive(Debug, Clone)]
pub struct Summary {
    pub reps: u32,
    pub num_players: usize,
    pub wins_per_game: Vec<u32>,
    pub per_game_ties: u32,
    pub busts: Vec<u32>,
    pub total_points: Vec<u64>,
    pub strat_labels: Vec<String>,
}

impl Summary {
    pub fn new(num_players: usize, reps: u32, labels: Vec<String>) -> Self {
        Self {
            reps,
            num_players,
            wins_per_game: vec![0; num_players],
            per_game_ties: 0,
            busts: vec![0; num_players],
            total_points: vec![0; num_players],
            strat_labels: labels,
        }
    }
}

pub struct Simulator {
    pub reps: u32,
    pub num_players: usize,
    pub strategies: Vec<Strategy>,
}

impl Simulator {
    pub fn new(reps: u32, num_players: usize, strategies: Vec<Strategy>) -> Self {
        Self {
            reps,
            num_players,
            strategies,
        }
    }

    pub fn run<F>(&self, mut on_progress: F) -> Summary
    where
        F: FnMut(u32, &Summary),
    {
        let num_players = self.num_players.clamp(2, 8);
        let mut strat_vec = Vec::with_capacity(num_players);
        let mut labels = Vec::with_capacity(num_players);

        for i in 0..num_players {
            let s = if let Some(st) = self.strategies.get(i) {
                st.clone()
            } else if !self.strategies.is_empty() {
                self.strategies.last().unwrap().clone()
            } else {
                Strategy::Threshold(12 + (i % 8) as u8)
            };
            labels.push(strategy_label(&s));
            strat_vec.push(s);
        }

        let mut summary = Summary::new(num_players, self.reps, labels);
        let update_every = (self.reps / 100).max(1);

        for iter in 0..self.reps {
            let mut baraja = crear_baraja();
            let mut jugadores: Vec<Jugador> = (0..num_players)
                .map(|i| {
                    let mut j = Jugador::nuevo();
                    j.nombre = format!("P{}", i + 1);
                    j
                })
                .collect();

            // Reparto inicial
            for _ in 0..2 {
                for p in jugadores.iter_mut() {
                    p.tomar_carta(&mut baraja);
                }
            }

            // Jugadores actúan
            for idx in 0..num_players {
                while should_draw(&strat_vec[idx], jugadores[idx].puntos(), &baraja) {
                    jugar_turno(&mut jugadores[idx], &mut baraja, true);
                    if jugadores[idx].puntos() > 21 {
                        break;
                    }
                }
            }

            // Evaluación
            let mut best_pts: i32 = -1;
            for i in 0..num_players {
                let pts = jugadores[i].puntos();
                summary.total_points[i] += pts as u64;
                if pts > 21 {
                    summary.busts[i] += 1;
                } else if (pts as i32) > best_pts {
                    best_pts = pts as i32;
                }
            }

            if best_pts >= 0 {
                let mut winners = Vec::new();
                for i in 0..num_players {
                    if jugadores[i].puntos() as i32 == best_pts {
                        winners.push(i);
                    }
                }
                if winners.len() == 1 {
                    summary.wins_per_game[winners[0]] += 1;
                } else {
                    summary.per_game_ties += 1;
                }
            } else {
                summary.per_game_ties += 1;
            }

            if (iter + 1) % update_every == 0 || iter + 1 == self.reps {
                on_progress(iter + 1, &summary);
            }
        }

        summary
    }
}

/// Ejecuta la simulación completa con un callback opcional para el progreso.
pub fn run_simulation<F>(
    reps: u32,
    num_players: usize,
    strategies: Vec<Strategy>,
    on_progress: F,
) -> Summary
where
    F: FnMut(u32, &Summary),
{
    let simulator = Simulator::new(reps, num_players, strategies);
    simulator.run(on_progress)
}

// Función para compatibilidad con main.rs (legacy simulate)
pub(crate) fn simulate(reps: u32, num_players: usize, strategies: Vec<Strategy>) -> Summary {
    run_simulation(reps, num_players, strategies, |_, _| {})
}

pub fn print_summary(summary: &Summary) {
    for line in summary_lines(summary) {
        println!("{}", line);
    }
}

pub fn summary_lines(summary: &Summary) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Resumen final después de {} partidas:",
        summary.reps
    ));
    lines.push(String::new());

    lines.push(format!(
        "{:<3} {:<30} {:>10} {:>8} {:>10}",
        "ID", "Estrategia", "Vict(part)", "%P", "PtsAvg"
    ));
    lines.push("-".repeat(70));

    for i in 0..summary.num_players {
        let vict = summary.wins_per_game[i];
        let pct = if summary.reps > 0 {
            (vict as f64) / (summary.reps as f64) * 100.0
        } else {
            0.0
        };
        let avg = if summary.reps > 0 {
            (summary.total_points[i] as f64) / (summary.reps as f64)
        } else {
            0.0
        };
        lines.push(format!(
            "{:<3} {:<30} {:>10} {:>7.2} {:>10.2}",
            i + 1,
            summary.strat_labels[i],
            vict,
            pct,
            avg
        ));
    }

    lines.push(String::new());
    lines.push(format!(
        "Partidas empatadas (múltiples ganadores o todos bust): {}",
        summary.per_game_ties
    ));

    // Determinar mejor estrategia
    let mut best_idx = 0;
    let mut best_vict = 0;
    for (i, &v) in summary.wins_per_game.iter().enumerate() {
        if v > best_vict {
            best_vict = v;
            best_idx = i;
        }
    }

    lines.push(String::new());
    lines.push(format!(
        "Estrategia con más victorias: {} ({:.2}%)",
        summary.strat_labels[best_idx],
        (best_vict as f64) / (summary.reps as f64) * 100.0
    ));

    lines
}
