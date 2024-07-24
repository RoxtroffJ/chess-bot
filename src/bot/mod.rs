//! This module implements all the generic bot algorithms for non random games played with two players in turns.

use crate::games::Game;
use std::time::Duration;
use std::ops::Neg;

mod alpha_beta;
use alpha_beta::*;

/// Trait that game scores shall implement in order to be used in the bots.
/// 
/// The higher the score, the most likely it is for the the player to win. 
/// Furthermore, if a player has a ```score```, then the score of the opponent is```-score```
pub trait Score : Ord + Copy + Neg<Output = Self> {
    /// Maximum value for a score.
    const VICTORY: Self;
}

/// [Result] type returned by all the bots.
pub type ResultBot<G> = Result<(<G as Game>::Move, <G as ScoredGame>::Score), String>;


/// Trait that all games needs to implement in order to work with this module.
pub trait ScoredGame : Game {
    /// The type of the score.
    type Score: Score;

    /// Returns the current score of the given player.
    fn score(&self, player: &Self::Player) -> Self::Score;

    /// Computes a next move using the iterative deepening alpha-beta pruning algorithm.
    /// 
    /// The depth of the search is given by the [compute_time](Duration) parameter.
    /// 
    /// If no moves can be found, then [Err] is returned with an error message.
    fn bot_move(&self, compute_time: Duration) -> ResultBot<Self>;
}

/// Trait that all games shall implement for the sake of simplicity. 
/// 
/// Depending on which traits are implemented, 
/// this is usually a matter of indicating which trait's ```bot_move``` with which parameters to use.
pub trait BestMove : ScoredGame {
    /// Returns the best valid move according to the best bot available.
    /// The default implementation uses the [bot_move](ScoredGame::bot_move) function,
    /// but it up to the user to override this implementation if a better bot exists.
    /// 
    /// The computation is done in at most the given [compute_time](Duration).
    /// 
    /// If no moves can be found, then [Err] is returned with an error message.
    fn best_move(&self, compute_time: Duration) -> ResultBot<Self> {
        self.bot_move(compute_time)
    }
}