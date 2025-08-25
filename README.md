# Proyecto 1 - Taylor Swift Labyrinth

**Proyecto de juego de laberinto en Rust utilizando SDL2**

---

## 📚 Descripción

Este proyecto es un juego de laberinto en 3D inspirado en un laberinto con temática de Taylor Swift. El jugador debe moverse por los niveles, recoger objetos y alcanzar la meta. Se implementa:

- Renderizado 3D básico con raycasting.
- Sprites para elementos del juego (álbum, obstáculos, etc.).
- Mini-mapa para orientación.
- Música y efectos de sonido.

---

## 🗂️ Estructura de carpetas
Proyecto1/
├─ assets/
│ ├─ music/ # Archivos de música (.mp3)
│ ├─ pages/ # Pantallas del juego (inicio, instrucciones, victoria)
│ ├─ sfx/ # Efectos de sonido
│ ├─ sprites/ # Sprites de personajes y objetos
│ └─ tx/ # Texturas de paredes
├─ maze/
│ ├─ maze1.txt # Laberinto nivel 1
│ ├─ maze2.txt # Laberinto nivel 2
│ └─ maze3.txt # Laberinto nivel 3
├─ src/
│ ├─ caster.rs # Raycasting y render 3D
│ ├─ framebuffer.rs# Funciones de dibujo, mini-mapa y FPS
│ ├─ line.rs # Cálculos de líneas y colisiones
│ ├─ main.rs # Bucle principal y lógica del juego
│ ├─ maze.rs # Carga y estructura de los laberintos
│ ├─ player.rs # Lógica del jugador y movimiento
│ └─ sprite.rs # Manejo de sprites
├─ Cargo.toml # Configuración del proyecto Rust
├─ Cargo.lock # Dependencias bloqueadas
└─ README.md # Este archivo


---

## 💻 Requisitos

- Rust 1.70 o superior
- SDL2 y sus bindings para Rust
- SDL2_image, SDL2_mixer y SDL2_ttf

**Instalación de dependencias en Linux (ejemplo):**

```bash
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
```

## ✨ Ejecución
```
git clone <url-del-repositorio>
cd Proyecto1
cd src
cargo run
```

## 🎮 Controles del juego
- Teclas de movimiento: W, A, S, D
- Rotar cámara: Movimiento del mouse
- Escape: Salir del juego
- Enter: Pasar pantallas (inicio, instrucciones, victoria)

## 📝 Características
- 3 niveles de laberinto, cada uno definido en maze/maze1.txt, maze2.txt, maze3.txt.
- Mini-mapa a la derecha que muestra posición del jugador y objetos importantes.
- Sprites animados para elementos del juego.
- Música y efectos de sonido reproducidos en bucle.

## 👤 Autor
Marinés García
