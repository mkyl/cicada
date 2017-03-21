# cicada
Chess engine compatabile with clients that use the universal chess interface (UCI).

- [x] Alpha-Beta Pruning
- [x] Most Valuable Victim, Least Valuable Attacker (MVV-LVA) ordering
- [x] Transposition Tables using Zobhrist Hashing

The sanity of the chess engine can be tested by running `cargo test`. This will run a series of unit and integration tests, including a pretty thorough Perft test.
