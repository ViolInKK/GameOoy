# GameOoy

Rust GameBoy emulator using SDL2. 

This project was made for the sake of learning emulation and all the concepts that come with it, and rust. It wasn't made with idea of deliviring it to end users but rather to get working prototype as fast as possible, so its pretty basic in it's implementation and doesnt have much. But if you stumbled upon this repo trying to achieve somthing similar - feel free to poke around and check out some of the resources I left at the end.
<div display="flex"> 
  <img src="https://github.com/ViolInKK/GameOoy/blob/main/preview%20images/tetris_title.png" width="250" height="250" />
  <img src="https://github.com/ViolInKK/GameOoy/blob/main/preview%20images/tetris.png" width="250" height="250" />
  <img src="https://github.com/ViolInKK/GameOoy/blob/main/preview%20images/tetris_end.png" width="250" height="250" />
</div>

# Installation

# Build from sources

## Requirements

* **git**
* **cargo** `>= 1.81.0`
* **SDL2** `>= 2.0.26`

# Usage

# TODO

* Add audio functionality.
* Saving states.
* More accurate PPU implementation.
* Proper code documentation.

# Refs. Great GameBoy emulation resources

**Here are main resources I used whilst developing this emulator:**

* [Pan Docs](https://gbdev.io/pandocs/About.html) - Main GB emulation resource, has everything you need to know.

* [gbdev optable](https://gbdev.io/gb-opcodes/optables/) - Table of CPU instructions.

* [gbdev](https://gbdev.io/) - Havent really used it but looks promising.

* [RGBDS CPU instructions](https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7) - CPU instructions explanations.

* [GBEDG](https://hacktix.github.io/GBEDG/ppu/) - Great blog about how GB PPU works.

* [Codeslinger Gameboy](http://www.codeslinger.co.uk/pages/projects/gameboy/beginning.html) - GB emulator blog with code examples. Pretty outdated but still good to understand how some parts work.

* [Lazy Stripes](https://blog.tigris.fr/2019/09/15/writing-an-emulator-the-first-pixel/) - GB emulator blog with code examples. This one is up-to-date.

* [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&ab_channel=media.ccc.de) - Great GB talk.

* [A Look At Game Boy Bootstrap](https://realboyemulator.wordpress.com/2013/01/03/a-look-at-the-game-boy-bootstrap-let-the-fun-begin/) - GB boot process explained in all details.

**Tools I used for debugging:**

* [Emulicious](https://emulicious.net/) - Emulator with great debugging tools.

* [Blargg's test roms](https://github.com/retrio/gb-test-roms) - CPU test roms.
