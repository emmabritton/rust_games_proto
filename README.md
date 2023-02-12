## Games

Collection of tactical/strategic board games. This is a proof of concept and example implementation of the rules and gameplay, and not a releasable product. However the program should be bug free and usable.

Each game plays, looks and controls differently as each is an experiment in UI and UX.

### Controls

`up`, `down`, `left`, `right` - Move cursor

`return` - Select

`escape` - Cancel/Exit

`cmd|win+r` - Restart

### Usage
```
games [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r               Print rules instead of opening game
    -V, --version    Prints version information

OPTIONS:
OPTIONS:
    -g, --game <game>...    Open game directly [possible values: tictactoe, mancala, draughts_brazilian,
                                                        draughts_canadian, draughts_international, draughts_english, orderchaos, senet, tablut]

```

#### Example

- `games` will start the program normally, displaying a menu of the games
- `games -g go` will start the game 'Go' directly
- `games -g shogi_mini -r` will print the rules to Shogi Mini

### List of games

- Tic-Tac-Toe
- Draughts
  - English *(8x8)*
  - Brazilian *(8x8 with flying kings)*
  - International  *(10x10 with flying kings)*
  - Canadian  *(12x12 with flying kings)*
- Chess
  - Standard *(8x8)*
  - Modern *(x)*
  - Mini *(x)*
  - Grand *(x)*
  - Andernach *(x)*
  - Checkless *(x)*
  - Progressive *(x)*
  - Hostage *(x)*
  - Capablanca *(x)*
- Mancala
- Tablut
- Order and Chaos
- Senet

#### Screenshots

![draughts screenshot](https://raw.githubusercontent.com/emmabritton/rust_games_proto/master/screenshots/draughts.png)
![draughts screenshot](https://raw.githubusercontent.com/emmabritton/rust_games_proto/master/screenshots/mancala.png)
![draughts screenshot](https://raw.githubusercontent.com/emmabritton/rust_games_proto/master/screenshots/tablut.png)