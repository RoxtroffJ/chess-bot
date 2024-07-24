//! This module contains the implementations for various games.

/// This trait is implemented by all games
pub trait Game {
    /// Type for a game move.
    ///
    /// There is no guarentee at compile time that the move is valid.
    type Move;

    /// Type for a player.
    type Player: Eq;

    /// Returns the player whose turn it is.
    fn current_player(&self) -> Self::Player;

    /// Plays the given [move](Self::Move). 
    /// 
    /// It does not check whether the move is legal or not.
    fn play(&self, m: &Self::Move);

    /// Undoes the previous move.
    fn undo(&self);

    /// Returns the list of all the next legal moves.
    fn get_moves(&self) -> Vec<Self::Move>;
}