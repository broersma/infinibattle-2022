![harry_crop](https://user-images.githubusercontent.com/6991518/150874310-2175ab11-cad5-4049-9ee4-3ca6903d8c54.jpg)

# Infinibattle 2022

My bot ("Harry") for Infinibattle 2022 - the ["L Game"](https://en.wikipedia.org/wiki/L_game) edition. Infinibattle is a game-AI bot arena tournament hosted and organised by [Infi](https://www.infi.nl/).

## Post-mortem

Sadly, I didn't finish in time to participate in the actual battle. However, I learned a lot about Rust, and some things are even starting to click! :open_mouth:

I really like the bot's enum-based state machine in `main.rs`, used for parsing the input correctly (it was also quite flexible when I eventually found out where I went wrong in implementing the match protocol).

There is some dead-ish code in `board_state.rs` for evaluating a search tree: at the moment the bot just generates all possible moves for a given game state (good!) and picks the... last one in the list (boo!).
