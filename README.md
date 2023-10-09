# GitHub Copilot Chat Demo

A simple project used to demonstrate how CoPilot chat can help with the software development.

## Explaining the code

Chat can help by explaining a piece of code. It does not only follows through the code techicalities, it can add some business context. In this example Chat explained that the `win` method calculates the Elo rating.

Select one of the `EloRating` structure methods.

```rust
pub fn win(&self, opponent: &EloRating) -> EloRating {
        let expected = 1.0 / (1.0 + 10.0_f64.powf((opponent.rating - self.rating) / 400.0));
        let new_rating = self.rating + self.k * (1.0 - expected);
        EloRating::new(new_rating)
    }
```

Ask Chat `Please explain the selected code`.

> The selected code is a Rust function that calculates the new Elo rating of a player after winning a game against an opponent with a given Elo rating. The function takes in two arguments: self, which is a reference to the player's own Elo rating, and opponent, which is a reference to the Elo rating of the opponent.
>
> The function first calculates the expected probability of winning the game based on the difference between the player's Elo rating and the opponent's Elo rating. This is done using the Elo rating system formula, which is a widely used method for calculating the relative skill levels of players in two-player games.
>
> The function then calculates the new Elo rating of the player based on the outcome of the game and the expected probability of winning. The k value is a constant that determines how much the player's rating will change based on the outcome of the game.
>
> Finally, the function returns a new EloRating object with the updated rating.

## Using code

Select the code and ask chat to explain how to use it.

`Can you provide an example of how to use Elo rating function in Rust?`

> Sure! Here's an example of how to use the win function in Rust:
> 
> 
> ```fn main() {
>    let player1 = EloRating::new(1500.0);
>    let player2 = EloRating::new(1600.0);
>
>    let new_rating = player1.win(&player2);
>
>    println!("Player 1's new rating: {}", new_rating.rating);
> }
> ```
> In this example, we create two EloRating objects for player1 and player2 with initial ratings of 1500 and 1600, respectively. We then call the win function on player1 with player2 as the opponent. The win function calculates the new Elo rating for player1 based on the outcome of the game and the opponent's rating.
> 
> Finally, we print out the new rating for player1 using the println! macro.
