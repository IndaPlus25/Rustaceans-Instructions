# DD1337 Week 3-4

_Author_: Viola Söderlund 
_Modified by_: Isak Larsson

## Chess Project

With that quick introduction out of the way, we are going to dive deeper into Rust and write a game-engine! The task for this and next week is to implement the logic for chess.

**In short:**
* Use Rust 🦀
* Implement the required features, as far as you can muster.
* Write tests for each of your functions that proves their functionality, to the best of your ability.
* Document your code.
  * Write an additional documentation in a README.md if your API differs from the one detailed below, documenting each of your *public* functions and structs/enums/etc.
* Write issues for your engine.
  * Design thoughts, bugs, unimplemented features, etc.

## Chess

The complete rules for chess can be found [here](https://en.wikipedia.org/wiki/Chess#Rules).

More resources:
* [Chess wiki](https://www.chessprogramming.org/Getting_Started)
  * [Board](https://www.chessprogramming.org/Board_Representation)
  * [Moves](https://www.chessprogramming.org/Move_Generation)
* [Array/vec](https://medium.com/@bellerb/building-a-chess-engine-part1-9758da877be7)
* [Fen](https://www.chessprogramming.org/Forsyth-Edwards_Notation)-strings ([example](https://www.youtube.com/watch?v=fVxvY-d28FE))
* [Bitboards](https://www.chessprogramming.org/Bitboards)

**Required features** _(view this as the goal)_
* Complete movesets for all pieces (except castling & en passant)
* Check
* Turn indicator (whose turn it is)
* Promotion

**Optional features**
* Castling
* En passant
* Checkmate
* Stalemate

You may implement it however you want, but it is recommended that you create a struct called `Game` with the following public functions: 

| **Function**                                                                  | **Description**                                                                                                                                                                               |
| ----------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `pub fn new() -> Game`                                                        | Initialises a new board with pieces.                                                                                                                                                          |
| `pub fn make_move(&mut self, from: &Position, to: &Position) -> Option<GameState>`  | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game.                                                                     |
| `pub fn set_promotion(&mut self, piece: &Position) -> ()`                        | Set the piece type that a peasant becames following a promotion.                                                                                                                              |
| `pub fn get_game_state(&self) -> GameState`                                   | Get the current game state.                                                                                                                                                                   |
| `pub fn get_possible_moves(&self, position: &Position) -> Optional<Vec<Position>>` | If a piece is standing on the given tile, return all possible new positions of that piece. Don't forget to the rules for check. _(optional)_ Don't forget to include en passent and castling. |

Positions are given as strings with the format `<file><rank>`. Ex. B4. Decide a suitable structure/enum/type `Position`.

Your program should also export an enumerator `GameState` with the values:
- `InProgress`, 
- `Check`,
- `GameOver`, 
- _(optional)_ `Checkmate` and
- _(optional)_ `DeadPosition`.

#### Expansion

A GUI application could also make use of enumerables such as `Colour` and `PieceType`. You may also like to make changes to the above API depeding on your board representation. If your library API do not reflect the documentation above, write your own complementary documentation in your repository's `README.md` file.

### Prepare assignment

1) Create a repository named `task-3-<KTH_ID>`.
2) Clone your regular assignment repository.
```sh
git clone git@gits-15.sys.kth.se:inda-25/<KTH_ID>-task-3.git
```
3) Add the upstream for `task-3-<KTH_ID>` to your local repository.
```sh
git remote add plus git@github.com:IndaPlus25/task-3-<KTH_ID>.git
```
4) The repository is your workspace. You may organise it however you want. I recommend to seperate the regular assignment from the plus assignment in two seperate directories. 
5) Create one library crate.
```sh
cargo init --lib chess-engine
```

See the template crate for help with code setup.

### Testing

Since your crate is of type library, we cannot simply test it by running it. Instead, test your application with Rust unit tests. 

The grading on this assignment is based on how well the tests demonstrates the full functionality and game mechanics of your chess engine. The tests are expected to not fail, and print a representation of the board in the case of move demonstrations. _Test at least all of your implemented functionality from the lists above to prove their functionality._

Run your unit tests with comments by entering the following command in your terminal:
```
cargo test -- --nocapture --test-threads=1
```

### Documentation

In addition to unit tests, all your public structures, functions, constants, and enumerables must have well written documentation comments.  
This can be done in multiple ways, one of which is the famous `doc`-comment.

You can read more at https://doc.rust-lang.org/reference/comments.html

> Note: I don't expect proffesional looking documentation, but atleast show some effort to try to document your code

### Progression

#### Week 3

Try to be able to present:
* Finished board representation.
* Complete movesets for all pieces.

#### Week 4

* As far as you're able. 