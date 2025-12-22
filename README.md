# Nonogram Advance

Nonogram game with 104 puzzles for the GBA

> [!WARNING]
> Nonograms are only partially done:
> 6x6:   21
> 8x8:   1
> 10x10: 1
> 12x12: 0
> 20x10: 0
> 22x12: 7

## Screenshots

![Screenshot of main menu](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/main_menu.png)
![Screenshot of 6x6 puzzle](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/6x6.png)
![Screenshot of 22x12 puzzle](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/22x12.png)

## Usage

First follow instructions at https://agbrs.dev/book/setup/getting_started.html

### Run

cargo run --release

(runs in mGBA)

### Test

cargo test

(runs in mGBA)

### Make gba file

agb-gbafix target/thumbv4t-none-eabi/release/nonogram_advance -o nonogram_advance.gba

## Thanks/Tools

- agb
  - https://agbrs.dev/
  - Framework for running rust on GBA
- mGBA 
  - https://mgba.io/
  - Testing
- aseprite
  - https://www.aseprite.org/
  - Creating backgrounds and sprites
- abyssbox
  - https://choptop84.github.io/abyssbox-app/ 
  - Creating music and sound effects
- audacity
  - https://www.audacityteam.org/
  - Editing/encoding music

## Lessons learned

- Macros (`include_aseprite`, `include_wav`, etc) should be in a separate module as they seriously affect IDE syntax checking/linting speed

## Things to learn/solve

- Scene management
  - enum of scenes instead of box
  - Stack of scenes
    - Issues: vram gets used up, solution might be to create backgrounds/sprites on init and drop backgrounds when backgrounded 
- Save structure
- BGM across multiple scenes
- Less repetitive sound setting checks (global sfx enabled check?)
- Unit testing without needing mgba
