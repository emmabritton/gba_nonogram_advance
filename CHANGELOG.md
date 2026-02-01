# 0.9.5

- Attempt to make all puzzles have a single solution by altering art (and hints)
  - Thanks to https://cirociampaglia.altervista.org/UPLOADS/nonogram_solver.html
  - 8x8
    - potion reduced from 249 solutions to 1
    - sword reduced from 1384 solutions to 4
    - staff reduced from 39997 solutions to 1
    - mirror redrawn as telephone
  - 10x10
    - added pineapple to pizza
    - hotdog reduced from 31790 solutions to 1
  - 12x12
    - fern reduced from 31579 solutions to 1
  - 20x10
    - paper place reduced from 10000 solutions to 1
    - reduce train carriage
    - reduce nessie
    - reduce books
  - 22x12
    - reduce castle
    - reduce moonscape
    - reduce waterfall
    - replaced forest with nono fox
- Add missing puzzle (12x12 lightbulb)
- Fix graphical issue with 20x10 grid

# 0.9.4

- Update code and actions to support SRAM and Flash64

# 0.9.3

- Fix bug where deleting save duplicated bgm
- Issue #1: 10x10#1 is ambiguous
  - Solution was to go through and validate every puzzle or change the validation to use clues (instead of puzzles)
  - I chose to change the validation to use clues

# 0.9.2

- Add reset option to settings

# 0.9.1

- Faster win animation
- Increase delay between inputs on puzzle screen to try to improve input feel
- Change to 512Kbit flash save for cheaper cartridges
- Fix some 12x12 puzzles
- Add cartridge sticker

# 0.9.0

- Add 9 10x10
- Add 10 12x12
- Add 9 20x10

# 0.2.3

- Add 3 22x12
- Add 11 10x10

# 0.2.2

- Change puzzle image on win screen to scale instead of fade in
- Move some of the 22x12 to 20x10
- Add 5 22x12

# 0.2.1

- Add 8 12x12

# 0.2.0

- Add 20 8x8
- Change cursor to go to last uncompleted puzzle in menu

# 0.1.1

- Add 21 6x6, 7 22x12, 1 8x8, 1 10x10 puzzles

# 0.1.0

- Initial version
  - Game is basically done but missing puzzles