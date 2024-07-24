//! This module implements all variations of the alpha-beta algorithm.

use super::*;
use std::cmp::*;

mod helpers;
use helpers::*;

/// Loop of the alpha-beta algorithm.
fn alpha_beta_aux<G: ScoredGame>(game: &G, player: &G::Player, max_depth: u16, mut alpha: G::Score, mut beta: G::Score, moves: Vec<G::Move>) -> (Option<G::Move>, G::Score) {
    
    // End of the search.
    if max_depth == 0 || moves.is_empty() {return (None, game.score(player))}

    // Initiate the moves
    let mut current_move = None;
    let mut score;
    
    if player == &game.current_player() {
        
        score = - G::Score::VICTORY;
        for other_move in moves {
            game.play(&other_move);
            let next_moves = game.get_moves();
            let (_, other_score) = alpha_beta_aux(game, player, max_depth-1, alpha, beta, next_moves);
            game.undo();

            update_with_scores(&mut score, &mut current_move, other_score, Some(other_move), false);
            alpha = max(alpha, score);
            if score >= beta {break}
        }
    
    } else {

        score = G::Score::VICTORY;
        for other_move in moves {
            game.play(&other_move);
            let next_moves = game.get_moves();
            let (_, other_score) = alpha_beta_aux(game, player, max_depth-1, alpha, beta, next_moves);
            game.undo();

            update_with_scores(&mut score, &mut current_move, other_score, Some(other_move), true);
            beta = min(beta, score);
            if score <= alpha {break}
        }
    }

    return (current_move, score);
}