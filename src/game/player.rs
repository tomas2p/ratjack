use crate::game::deck::Carta;

pub struct Jugador {
    pub mano: Vec<Carta>,
    pub nombre: String,
    puntos: u8,
    pub partidas_ganadas: u32,
}

impl Jugador {
    pub fn nuevo() -> Self {
        Jugador {
            mano: Vec::new(),
            nombre: "Jugador".to_string(),
            puntos: 0,
            partidas_ganadas: 0,
        }
    }

    pub fn puntos(&self) -> u8 {
        self.puntos
    }

    pub fn tomar_carta(&mut self, baraja: &mut Vec<Carta>) {
        if let Some(carta) = baraja.pop() {
            self.mano.push(carta);
            self.puntos = self.calcular_puntos();
        }
    }

    pub fn reiniciar(&mut self) {
        self.mano.clear();
        self.puntos = 0;
    }

    fn calcular_puntos(&self) -> u8 {
        let mut total = 0;
        let mut ases = 0;

        for carta in &self.mano {
            total += carta.puntos();
            if carta.valor == 1 {
                ases += 1;
            }
        }

        while total > 21 && ases > 0 {
            total -= 10;
            ases -= 1;
        }

        total
    }

    pub fn partida_ganada(&mut self) {
        self.partidas_ganadas += 1;
    }
}

#[cfg(test)]
impl Jugador {
    pub fn set_puntos(&mut self, puntos: u8) {
        self.puntos = puntos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::deck::{Carta, Palo};

    #[test]
    fn test_jugador_nuevo() {
        let jugador = Jugador::nuevo();
        assert_eq!(jugador.nombre, "Jugador");
        assert_eq!(jugador.puntos(), 0);
        assert_eq!(jugador.partidas_ganadas, 0);
        assert!(jugador.mano.is_empty());
    }

    #[test]
    fn test_tomar_carta() {
        let mut jugador = Jugador::nuevo();
        let mut baraja = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            },
        ];

        jugador.tomar_carta(&mut baraja);
        assert_eq!(jugador.mano.len(), 1);
        assert_eq!(baraja.len(), 1);
        assert_eq!(jugador.mano[0].valor, 5);
        assert_eq!(jugador.puntos(), 5);

        jugador.tomar_carta(&mut baraja);
        assert_eq!(jugador.mano.len(), 2);
        assert!(baraja.is_empty());
        assert_eq!(jugador.puntos(), 15);
    }

    #[test]
    fn test_reiniciar() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![Carta {
            palo: Palo::Corazones,
            valor: 10,
        }];
        // Forzamos el valor ya que puntos es privado y se actualiza al tomar carta
        jugador.puntos = 10;

        jugador.reiniciar();
        assert!(jugador.mano.is_empty());
        assert_eq!(jugador.puntos(), 0);
    }

    #[test]
    fn test_puntaje_simple() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            },
        ];
        // Calculamos puntos manualmente para el test
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 15);
    }

    #[test]
    fn test_puntaje_con_as() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 10,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 21);
    }

    #[test]
    fn test_puntaje_con_multiples_ases() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 1,
            },
            Carta {
                palo: Palo::Tréboles,
                valor: 1,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 13);
    }

    #[test]
    fn test_puntaje_con_blackjack() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 13,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 21);
    }

    #[test]
    fn test_partida_ganada() {
        let mut jugador = Jugador::nuevo();
        assert_eq!(jugador.partidas_ganadas, 0);

        jugador.partida_ganada();
        assert_eq!(jugador.partidas_ganadas, 1);

        jugador.partida_ganada();
        assert_eq!(jugador.partidas_ganadas, 2);
    }

    #[test]
    fn test_puntaje_as_que_cambia() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 1,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 5,
            },
            Carta {
                palo: Palo::Tréboles,
                valor: 10,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 16);
    }

    #[test]
    fn test_puntaje_con_figuras() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 11,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 12,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 20);
    }

    #[test]
    fn test_puntaje_con_mas_de_21() {
        let mut jugador = Jugador::nuevo();
        jugador.mano = vec![
            Carta {
                palo: Palo::Corazones,
                valor: 10,
            },
            Carta {
                palo: Palo::Diamantes,
                valor: 10,
            },
            Carta {
                palo: Palo::Tréboles,
                valor: 5,
            },
        ];
        jugador.puntos = jugador.calcular_puntos();

        assert_eq!(jugador.puntos(), 25);
    }
}
