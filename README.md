# GameOoy

Rust GameBoy emulator using SDL2. 

<img src="https://github.com/ViolInKK/GameOoy/blob/main/preview%20images/tetris_title.png" width="300" height="300" />


This project was made for the sake of learning emulation and all the concepts that come with it, and rust. It wasn't made with idea of deliviring it to end users but rather to get working prototype as fast as possible, so its pretty basic in it's implementation and doesnt have much. But if you stumbled upon this repo trying to achieve somthing similar - feel free to poke around and check out some of the resources I left at the end.

# Installation

## Requirements

* **git**
* **cargo** `>= 1.81.0`
* **SDL2** `>= 2.0.26`

# Usage

# TODO

* Add audio functionality
* Saving states
* Proper code documentation

# Refs. Great GameBoy emulation resources

Here are main resources I used whilst developing this emulator:

[Pan Docs](https://gbdev.io/pandocs/About.html)

[gbdev optable](https://gbdev.io/gb-opcodes/optables/)

[RGBDS CPU instructions](https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7)

[GBEDG](https://hacktix.github.io/GBEDG/ppu/#oam-scan-mode-2)

[Codeslinger Gameboy](http://www.codeslinger.co.uk/pages/projects/gameboy/beginning.html)

[Lazy Stripes](https://blog.tigris.fr/2019/09/15/writing-an-emulator-the-first-pixel/)

[The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&ab_channel=media.ccc.de)

[Copetti](https://www.copetti.org/writings/consoles/game-boy)

[A Look At Game Boy Bootstrap](https://realboyemulator.wordpress.com/2013/01/03/a-look-at-the-game-boy-bootstrap-let-the-fun-begin/)

Tools I used for debugging:

[Emulicious](https://emulicious.net/)

[Blargg's test roms](https://github.com/retrio/gb-test-roms)
