# go-crabbing
Implementing the card game Go Fish in Rust

## Go Fish rules
### Setup
- 5 cards are dealt from a standard 52-card deck to each player, or 7 cards if there are only two players.
- The remaining cards are shared between the players, usually spread out in a disorderly pile referred to as the "ocean" or "pool". We will call it the `ocean`.

### Game loop
- The player whose turn it is to play asks any other player for their cards of a particular face value (i.e 8s). The player must have at least one card of the rank requested before asking for it.
- The other player must hand over all cards of that rank if possible.
- If they have none, they tell the 1st player to "go crabbing", and the 1st player draws a card from the pool and places it in their own hand.
- Then it is the next player's turn – unless the card drawn is the card he asked for, in which case they show the card to the other players, and gets another turn.

### Other rules
- When any player at any time has four cards of one face value, it forms a "book", and the cards must be placed face up in front of that player.
- If the player's hand is empty, they have to draw a card from the pool.
- Play proceeds to the left.

### Ending the game
- When all sets of cards have been laid down in "books", the game ends.
- The player with the most "books" wins.

## Implementation notes
- 52 card `deck` => model as face values as keys (A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K) * (heart, diamond, spade, club)
  - could this be an iterator?
- `hand` => subset of the `deck`, hash map with face value => 0 (initial) | 1 | 2 | 3 | 4
- `ocean` => inital state: `deck`; after deal: `deck` - (`hand` * players), end of turn: `ocean` - random `card`
- `player` has a `hand`, `books` = set of 4 cards

### Game states
**Before Deal: Initial (0) state**
- assign dealer = player 1
- `deck` = deck
- `ocean` = empty

**After Deal: (1) state**
- player 1 `hand` = 7 cards ("take" 7 from deck)
- player 2 `hand` = 7 cards ("take" 7 from deck)
- `ocean` =  52 - 14 = 38 cards (deck after the taking has happened)
  
--- game loop ---
**Player 2**
if receiving a card during transfer
- check for `book` = any values in hash map = 4
  - move any `books` to `book` structure
- check if in end state

- random choice from `hand` and get face value
- ask player 1 for face value of `card`
- transfer `turn` to player 2

**Player 1**
- check for `book` = any values in hash map = 4
  - move any `books` to `book` structure

- check for face value in `hand`
  - if face value in `hand` = transfer `turn` to player 1 (with card)
  - else 'go crabbing' -> transfer `turn` to player 1 (with no card)

- check if in end state (13 books total across all players)

**End state**
- Find largest number of `books`, largest number of `books` win

### a turn
a `turn` involves:
- either receiving 0 or more cards or being asked for cards
  - special case are first round:
    - first round skips input and goes straight to request
    - player 1 = in: none, request: cards, response: cards | go crabbing
    - player 2 = in: cards | go crabbing, request: cards, response: cards | go crabbing
    - player 1 = in: cards | go crabbing, request: cards, response: cards | go crabbing
    - player 2 = in: cards | go crabbing, request: cards, response: cards | go crabbing
    - ...

- checking for a `book`
- checking if in the end state
- 