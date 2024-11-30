# 2048 Nano

Welcome! 

2048 is a beloved ~~time-waster~~ puzzle game, with simple rules and a small board. I've been wondering, exactly how small can I get a program to run it functionally?

And this is the end result. `nano-2048/src/main.rs` only uses the `rand` external crate, and currently compiles down to about 490 kB, 330kB of which is the executable. This is using common compiler settings.

## To play:

enter commands by pressing any of the "wasd" directional keys, and press `enter` to submit.
Press `x` instead to immediately exit the game.
