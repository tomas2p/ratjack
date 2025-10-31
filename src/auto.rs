use crate::game::deck::crear_baraja;
use crate::game::logic::jugar_turno;
use crate::game::player::Jugador;
use crate::strategies::{should_draw, strategy_label, Strategy};

#[derive(Debug)]
pub struct Summary {
    pub reps: u32,
    pub num_players: usize,
    pub wins: Vec<u32>,
    pub busts: Vec<u32>,
    pub total_points: Vec<u64>,
    pub ties: u32,
    pub strat_labels: Vec<String>,
}

// Simulate `reps` games with `num_players` using provided strategies vector.
// This function is crate-visible (pub(crate)) so it can be called from the UI, but we avoid exposing
// it as a public CLI helper to encourage running simulations through the UI.
pub(crate) fn simulate(reps: u32, num_players: usize, strategies: Vec<Strategy>) -> Summary {
    let num_players = num_players.clamp(2, 8);

    // Build strategy vector
    let mut strat_vec: Vec<Strategy> = Vec::with_capacity(num_players);
    let mut strat_labels: Vec<String> = Vec::with_capacity(num_players);
    for i in 0..num_players {
        if let Some(s) = strategies.get(i) {
            strat_vec.push(s.clone());
        } else if !strategies.is_empty() {
            strat_vec.push(strategies.last().unwrap().clone());
        } else {
            let t = 12 + ((i % 8) as u8);
            strat_vec.push(Strategy::Threshold(t));
        }
        strat_labels.push(strategy_label(&strat_vec[i]));
    }

    let mut wins = vec![0u32; num_players];
    let mut busts = vec![0u32; num_players];
    let mut total_points = vec![0u64; num_players];
    let mut ties = 0u32;

    for _ in 0..reps {
        let mut baraja = crear_baraja();

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
    }

    Summary {
        reps,
        num_players,
        wins,
        busts,
        total_points,
        ties,
        strat_labels,
    }
}

pub fn print_summary(summary: &Summary) {
    println!("Resumen final después de {} partidas:", summary.reps);
    for i in 0..summary.num_players {
        println!(
            "{} | {} | Victorias: {} | Busts: {} | Puntos avg: {:.2}",
            if i == summary.num_players - 1 {
                "Banca".to_string()
            } else {
                format!("Jugador {}", i + 1)
            },
            summary.strat_labels.get(i).cloned().unwrap_or_default(),
            summary.wins[i],
            summary.busts[i],
            (summary.total_points[i] as f64) / (summary.reps as f64)
        );
    }
    println!("Empates totales: {}", summary.ties);
}
