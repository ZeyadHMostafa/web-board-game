use std::sync::atomic::Ordering;
use crate::ai::search::SearchContext;
use crate::ai::search::utils::{HashEntryBounds, TranspositionTable, invert_score};
use crate::ai::heuristics::evaluators;
use crate::rules::state::GameState;
use crate::ai::evaluator::EvaluationScore;

/// Core recursive Negamax algorithm with Alpha-Beta pruning.
pub fn negamax(
    ctx: &SearchContext,
    tt: &mut TranspositionTable,
    state: &GameState,
    depth: usize,
    mut alpha: EvaluationScore,
    beta: EvaluationScore,
) -> EvaluationScore {
    ctx.nodes_explored.fetch_add(1, Ordering::Relaxed);
    // 1. Check for timeout/abort signals early
    if ctx.cancelled.load(Ordering::Relaxed) {
        return EvaluationScore::Value(0); // Value doesn't matter; it will be discarded
    }

    let original_alpha = alpha;
    if let Some(entry) = tt.lookup(state) {
        if entry.depth >= depth {
            match entry.bounds {
                HashEntryBounds::Exact => return entry.score,
                HashEntryBounds::AlphaLower => {
                    if entry.score <= alpha { return entry.score; }
                }
                HashEntryBounds::BetaUpper => {
                    if entry.score >= beta { return entry.score; }
                }
            }
        }
    }
    
    // 2. Base case: Leaf node
    if depth == 0 {
        return ctx.evaluator.evaluate(state);
    }

    let mut legal_moves = state.generate_legal_moves(ctx.luts);
    if legal_moves.is_empty() {
        return EvaluationScore::Mated(0)
    }

    // 3. Move Ordering Optimization
    // Pre-sorting moves dramatically increases the chances of early alpha-beta cutoffs.
    // Here we use a simple heuristic: evaluate the immediate state resulting from the move.
    // Identify who we are fighting to match target capture bits
    if depth != 1 {
        let allied_pieces = state.get_player_pieces(state.active_player);
        let enemy_pieces = state.get_player_pieces(state.active_player.opponent());
        
        // Sort directly on the stack allocation using the lightweight heuristic hierarchy
        legal_moves.sort_unstable_by_key(
            |&m|evaluators::evaluate_move(&m, allied_pieces, enemy_pieces)
        );
    }

    let mut max_score = EvaluationScore::Value(i32::MIN);

    // 4. Recursive Search Loop
    for current_move in legal_moves {
        let mut next_state = state.clone();
        next_state.make_move(current_move);

        // Negamax step: Evaluate opponent's position and invert it
        let raw_score = negamax(ctx, tt, &next_state, depth - 1, invert_score(beta), invert_score(alpha));
        let relative_score = invert_score(raw_score);

        if relative_score > max_score {
            max_score = relative_score;
        }

        if max_score > alpha {
            alpha = max_score;
        }

        // Alpha-Beta Pruning Cutoff
        if alpha >= beta {
            break; 
        }

    }
    // Categorize what kind of score this was based on alpha/beta limits
    let bounds = if max_score <= original_alpha {
        HashEntryBounds::AlphaLower
    } else if max_score >= beta {
        HashEntryBounds::BetaUpper
    } else {
        HashEntryBounds::Exact
    };

    tt.store(state, max_score, depth, bounds);

    max_score
}