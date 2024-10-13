# Retris

A simple implementation of Tetris, using Rust.

# Usage
To start, in a command window:

```
 cargo run cmd/retris     
```

Use the following keys on your keyboard:

* Left arrow: move to left
* Right arrow: move to right
* x: Rotate clockwise
* z: Rotate anti-clockwise.
* ESC: Exit the game.

# Known things to implement

* Have pieces fall at a consistent pace, independent of key presses.
* Implement space bar to allow pieces to quickly fall.
* Handle end of game - currently crashes.
* Speed up rate of fall as there are fewer clear roles available.
* Remove rows that are complete.
* Remove many bugs!
