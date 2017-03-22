# cicada
Chess engine compatible with clients that use the universal chess interface (UCI). Ranks around 1450 - 1650 ELO.

Cicada was created as a hobby project. Nonetheless, it implements a substantial set of features:
- [x] Alpha-Beta pruning
- [x] Most Valuable Victim, Least Valuable Attacker (MVV-LVA) ordering
- [x] Transposition tables using Zobrist hashing
- [x] Quiescence Search

### Download
Download a copy of Cicada from [here](https://github.com/mkyl/cicada/releases). You'll need your own GUI; [Arena](http://www.playwitharena.com/) is a good option.

### Compile
The program can be built from source by cloning the repository and then running `cargo build --release`.

The sanity of the chess engine can be tested by running `cargo test`. This will run a series of unit and integration tests, including a pretty thorough Perft test.
