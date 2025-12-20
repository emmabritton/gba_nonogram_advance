# Nonogram Advance

Nonogram game with 104 puzzles for the GBA

## Screenshots



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
