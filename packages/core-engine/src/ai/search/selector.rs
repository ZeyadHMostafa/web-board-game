use rand::RngExt;

use crate::rules::moves::Move;
use crate::ai::search::SearchProgress;
use crate::ai::evaluator::EvaluationScore;

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Difficulty {
    pub temp: f32,
    pub b_thresh: i32
}

impl Difficulty{
    /// Plays the absolute best calculated move.
    pub const EXPERT: Difficulty = Difficulty{temp: 0.0, b_thresh: 0};
    /// Slight variations in move choice; rarely blunders heavily.
    pub const ADVANCED: Difficulty = Difficulty{temp: 0.2, b_thresh: 30};
    /// Prone to selecting secondary tactical choices in complex positions.
    pub const INTERMEDIATE: Difficulty = Difficulty{temp: 0.6, b_thresh: 50};
    /// High temperature distribution; frequently mixes up sub-optimal moves.
    pub const CASUAL: Difficulty = Difficulty{temp: 1.5, b_thresh: 80};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectorMode {
    Competitive,
    AdaptiveDifficulty(Difficulty),
    TrainingExploration { epsilon: f32 },
}

pub struct ActionSelector;

impl ActionSelector {
    /// Selects a move from candidate progress according to the requested evaluation strategy.
    pub fn select_move(result: &SearchProgress, mode: SelectorMode) -> Option<Move> {
        if result.candidates.is_empty() {
            return None;
        }

        match mode {
            SelectorMode::Competitive => {
                result.candidates
                    .iter()
                    .max_by_key(|c| self::extract_score_value(c.score))
                    .map(|c| c.current_move)
            }
            SelectorMode::TrainingExploration { epsilon } => {
                let mut rng = rand::rng();
                if rng.random::<f32>() < epsilon {
                    let random_idx = rng.random_range(0..result.candidates.len());
                    Some(result.candidates[random_idx].current_move)
                } else {
                    result.candidates
                        .iter()
                        .max_by_key(|c| self::extract_score_value(c.score))
                        .map(|c| c.current_move)
                }
            }
            SelectorMode::AdaptiveDifficulty(difficulty) => {
                if difficulty == Difficulty::EXPERT {
                    return result.candidates
                        .iter()
                        .max_by_key(|c| self::extract_score_value(c.score))
                        .map(|c| c.current_move);
                }
                self::select_tempered_move(result, difficulty)
            }
        }
    }
}

// ============================================================================
// LOW-LEVEL INLINE SELECTOR ASSISTANTS
// ============================================================================

/// Converts evaluation score variants into scalar integer space.
#[inline(always)]
fn extract_score_value(score: EvaluationScore) -> i32 {
    match score {
        EvaluationScore::Value(v) => v,
        EvaluationScore::Mating(ply) => i32::MAX - (ply as i32),
        EvaluationScore::Mated(ply) => i32::MIN + (ply as i32),
    }
}

/// Applies a softmax transformation over eligible candidate moves, weighting selection 
/// probabilities based on position evaluations and operational temperature boundaries.
#[inline(always)]
fn select_tempered_move(result: &SearchProgress, difficulty: Difficulty) -> Option<Move> {
    let scores: Vec<i32> = result.candidates
        .iter()
        .map(|c| extract_score_value(c.score))
        .collect();

    let max_score = *scores.iter().max()?;
    let threshold = difficulty.b_thresh;
    let temp = difficulty.temp;

    // Compute raw weights using an exponential scale centered around the best move
    let mut weights = Vec::with_capacity(result.candidates.len());
    let mut total_weight = 0.0;

    for &score in &scores {
        let loss = max_score - score;
        if loss <= threshold {
            // Softmax transformation scaled by temperature parameter
            let weight = (-(loss as f32) / (100.0 * temp)).exp();
            weights.push(weight);
            total_weight += weight;
        } else {
            weights.push(0.0);
        }
    }

    if total_weight <= 0.0 {
        return result.candidates
            .iter()
            .max_by_key(|c| extract_score_value(c.score))
            .map(|c| c.current_move);
    }

    let mut rng = rand::rng();
    let mut roll = rng.random_range(0.0..total_weight);

    for (idx, &weight) in weights.iter().enumerate() {
        if weight > 0.0 {
            roll -= weight;
            if roll <= 0.0 {
                return Some(result.candidates[idx].current_move);
            }
        }
    }

    result.candidates.first().map(|c| c.current_move)
}