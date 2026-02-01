# Nonogram Advance

Nonogram game with 108 puzzles for the GBA

## Screenshots

![Screenshot of main menu](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/main_menu.png)
![Screenshot of 6x6 puzzle](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/6x6.png)
![Screenshot of 22x12 puzzle](https://raw.githubusercontent.com/emmabritton/gba_nonogram_advance/refs/heads/main/.github/screenshots/22x12.png)

## Player Usage

Download gba file from [here](https://github.com/emmabritton/gba_nonogram_advance/releases/latest) and run in an emulator (mGBA recommended) or on an actual GBA

> [!TIP]
> * Use nonogram_advance_v0.9.5_sram.gba on SRAM carts or things like EverDrive
> * Use nonogram_advance_v0.9.5_flash64.gba on 512KBit/64K Flash carts 

## Dev Usage

First follow instructions at https://agbrs.dev/book/setup/getting_started.html

### Run

cargo run

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
- Nongram solver
  - https://cirociampaglia.altervista.org/UPLOADS/nonogram_solver.html
  - Used to verify correctness of puzzles

## Lessons learned

- Macros (`include_aseprite`, `include_wav`, etc) should be in a separate module as they seriously affect IDE syntax checking/linting speed
- The sum of row clues should generally match the sum of column clues 
- Small puzzles (6x6, 8x8) have their sprites doubled, but this allows for 'subpixel' detailing (see 8x8 potion)
- Don't make diagonal puzzles

## Things to learn/solve

- Scene management
  - enum of scenes instead of box
  - Stack of scenes
    - Issues: vram gets used up, solution might be to create backgrounds/sprites on init and drop backgrounds when backgrounded 
- Save structure
- BGM across multiple scenes
- Less repetitive sound setting checks (global sfx enabled check?)
- Unit testing without needing mgba
- How palettes are generated
- Buzzing on music