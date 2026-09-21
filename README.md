<h1> <img src="https://cdn.simpleicons.org/ratatui/FFDC00" height="25"> RatJack - Blackjack en Rust con Ratatui </h1>

---

**RatJack** es un juego de **Blackjack** interactivo y simulador escrito en **Rust**, que utiliza **Ratatui** para crear una experiencia envolvente en la terminal.

---

## 🎲 Características y Mejoras Recientes

- ✅ **Interfaz Visual de Cartas:** Renderizado ordenado con colores según el palo, optimizado para evitar recortes.
- ✅ **Conteo de la Banca:** Seguimiento en tiempo real con marcadores visuales para cartas ocultas.
- ✅ **Resolución Automática:** Victoria automática al alcanzar 21 y continuación instantánea tras plantarse.
- ✅ **Estrategias Personalizables:** Configuración dinámica de algoritmos para jugador y banca.

---

## 📊 Comparativa de Estrategias

RatJack incluye un motor de estrategias automatizadas que puedes configurar mediante `--ui-strategies`.

| Estrategia | Ejemplo | Descripción | Comportamiento |
| :--- | :--- | :--- | :--- |
| **Threshold** | `threshold:17` | Se planta si puntuación $\ge$ valor. | Estándar de la banca. |
| **Random** | `random:0.5` | Decisión basada en probabilidad $P$. | Caótico/Errático. |
| **Prob** | `prob:0.4` | Cálculo de probabilidad de *bust*. | Conservador/Calculador. |
| **Count** | `count:17` | Conteo de cartas Hi-Lo. | Avanzado/Estadístico. |

---

## 🚀 Instalación y Ejecución

```bash
git clone https://github.com/tomas2p/ratjack.git
cd ratjack
cargo run -- --ui --ui-strategies "threshold:17,threshold:16"
```

---

## 🎮 Controles

| Tecla | Acción |
| :--- | :--- |
| <kbd>↵</kbd> / <kbd>1</kbd> / <kbd>p</kbd> | Pedir (Hit) |
| <kbd>2</kbd> / <kbd>s</kbd> | Plantarse (Stand) |
| <kbd>n</kbd> | Nueva partida |
| <kbd>q</kbd> | Salir |

---

## 📜 Licencia

Este proyecto está bajo la licencia [MIT](LICENSE).
