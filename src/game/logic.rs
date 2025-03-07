use crate::game::deck::Carta;
use crate::game::player::Jugador;

// Función para repartir cartas iniciales
pub fn repartir_cartas(jugador: &mut Jugador, banca: &mut Jugador, baraja: &mut Vec<Carta>) {
    // Se reparten dos cartas a cada jugador
    jugador.tomar_carta(baraja);
    banca.tomar_carta(baraja);
    jugador.tomar_carta(baraja);
    banca.tomar_carta(baraja);

    // Actualizar puntos
    jugador.puntos = jugador.puntaje();
    banca.puntos = banca.puntaje();
}

// Función para jugar un turno
pub fn jugar_turno(jugador: &mut Jugador, baraja: &mut Vec<Carta>, tomar_carta: bool) {
    if tomar_carta {
        jugador.tomar_carta(baraja);
        jugador.puntos = jugador.puntaje();
    }
}

// Función para determinar el ganador
pub fn determinar_ganador(jugador: &mut Jugador, banca: &mut Jugador) -> String {
    let puntos_jugador = jugador.puntos;
    let puntos_banca = banca.puntos;

    let mensaje = if puntos_jugador > 21 {
        banca.partida_ganada();
        "Te has pasado. ¡La banca gana!"
    } else if puntos_banca > 21 {
        jugador.partida_ganada();
        "La banca se ha pasado. ¡Has ganado!"
    } else if puntos_jugador > puntos_banca {
        jugador.partida_ganada();
        "¡Has ganado!"
    } else if puntos_banca > puntos_jugador {
        banca.partida_ganada();
        "La banca gana."
    } else {
        "Empate."
    };
    mensaje.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::crear_baraja;
    use crate::game::player::Jugador;

    #[test]
    fn test_repartir_cartas() {
        let mut baraja = crear_baraja();
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();

        repartir_cartas(&mut jugador, &mut banca, &mut baraja);

        assert_eq!(jugador.mano.len(), 2);
        assert_eq!(banca.mano.len(), 2);
        assert_eq!(baraja.len(), 48); // 52 - 4 cartas repartidas
    }

    #[test]
    fn test_jugar_turno() {
        let mut baraja = crear_baraja();
        let mut jugador = Jugador::nuevo();

        jugar_turno(&mut jugador, &mut baraja, true);
        assert_eq!(jugador.mano.len(), 1);

        jugar_turno(&mut jugador, &mut baraja, false);
        assert_eq!(jugador.mano.len(), 1); // No debe cambiar si no toma carta
    }

    #[test]
    fn test_determinar_ganador_jugador_pasa() {
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();
        jugador.puntos = 22;
        banca.puntos = 18;

        assert_eq!(
            determinar_ganador(&mut jugador, &mut banca),
            "Te has pasado. ¡La banca gana!"
        );
    }

    #[test]
    fn test_determinar_ganador_banca_pasa() {
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();
        jugador.puntos = 18;
        banca.puntos = 22;

        assert_eq!(
            determinar_ganador(&mut jugador, &mut banca),
            "La banca se ha pasado. ¡Has ganado!"
        );
    }

    #[test]
    fn test_determinar_ganador_jugador_gana() {
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();
        jugador.puntos = 20;
        banca.puntos = 18;

        assert_eq!(determinar_ganador(&mut jugador, &mut banca), "¡Has ganado!");
    }

    #[test]
    fn test_determinar_ganador_banca_gana() {
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();
        jugador.puntos = 17;
        banca.puntos = 20;

        assert_eq!(
            determinar_ganador(&mut jugador, &mut banca),
            "La banca gana."
        );
    }

    #[test]
    fn test_determinar_ganador_empate() {
        let mut jugador = Jugador::nuevo();
        let mut banca = Jugador::nuevo();
        jugador.puntos = 19;
        banca.puntos = 19;

        assert_eq!(determinar_ganador(&mut jugador, &mut banca), "Empate.");
    }
}
