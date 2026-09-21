use crate::game::deck::Carta;
use crate::game::player::Jugador;

/// Representa el estado de una mesa de juego.
pub struct Mesa {
    pub jugador: Jugador,
    pub banca: Jugador,
    pub baraja: Vec<Carta>,
}

impl Mesa {
    pub fn nueva(baraja: Vec<Carta>) -> Self {
        Self {
            jugador: Jugador::nuevo(),
            banca: Jugador::nuevo(),
            baraja,
        }
    }

    pub fn repartir_inicial(&mut self) {
        self.jugador.reiniciar();
        self.banca.reiniciar();

        for _ in 0..2 {
            self.jugador.tomar_carta(&mut self.baraja);
            self.banca.tomar_carta(&mut self.baraja);
        }
    }
}

// Función para repartir cartas iniciales (mantener por compatibilidad o simplicidad)
pub fn repartir_cartas(jugador: &mut Jugador, banca: &mut Jugador, baraja: &mut Vec<Carta>) {
    jugador.reiniciar();
    banca.reiniciar();

    for _ in 0..2 {
        jugador.tomar_carta(baraja);
        banca.tomar_carta(baraja);
    }
}

// Función para jugar un turno
pub fn jugar_turno(jugador: &mut Jugador, baraja: &mut Vec<Carta>, tomar_carta: bool) {
    if tomar_carta {
        jugador.tomar_carta(baraja);
    }
}

// Función para determinar el ganador
pub fn determinar_ganador(jugador: &mut Jugador, banca: &mut Jugador) -> String {
    let p_j = jugador.puntos();
    let p_b = banca.puntos();

    if p_j > 21 {
        banca.partida_ganada();
        "Te has pasado. ¡La banca gana!".to_string()
    } else if p_b > 21 {
        jugador.partida_ganada();
        "La banca se ha pasado. ¡Has ganado!".to_string()
    } else if p_j > p_b {
        jugador.partida_ganada();
        "¡Has ganado!".to_string()
    } else if p_b > p_j {
        banca.partida_ganada();
        "La banca gana.".to_string()
    } else {
        "Empate.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::crear_baraja;

    #[test]
    fn test_repartir_cartas() {
        let mut baraja = crear_baraja();
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();

        repartir_cartas(&mut jugador, &mut banca, &mut baraja);

        assert_eq!(jugador.mano.len(), 2);
        assert_eq!(banca.mano.len(), 2);
        assert_eq!(baraja.len(), 48);
        assert!(jugador.puntos() > 0);
        assert!(banca.puntos() > 0);
    }

    #[test]
    fn test_mesa_repartir() {
        let baraja = crear_baraja();
        let mut mesa = Mesa::nueva(baraja);
        mesa.repartir_inicial();
        assert_eq!(mesa.jugador.mano.len(), 2);
        assert_eq!(mesa.banca.mano.len(), 2);
    }

    #[test]
    fn test_determinar_ganador() {
        let mut j = Jugador::nuevo();
        let mut b = Jugador::nuevo();

        j.set_puntos(20);
        b.set_puntos(18);
        assert_eq!(determinar_ganador(&mut j, &mut b), "¡Has ganado!");
        assert_eq!(j.partidas_ganadas, 1);

        j.set_puntos(22);
        assert_eq!(
            determinar_ganador(&mut j, &mut b),
            "Te has pasado. ¡La banca gana!"
        );
        assert_eq!(b.partidas_ganadas, 1);
    }
}
