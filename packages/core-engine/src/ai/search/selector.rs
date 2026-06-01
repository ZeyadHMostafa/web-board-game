use rand::RngExt;
use crate::rules::moves::Move;
use crate::ai::search::SearchProgress;
use crate::ai::EvaluationScore;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectorMode {
    Competitive,
    TrainingExploration { epsilon: f32 },
}

pub struct ActionSelector;

impl ActionSelector {
    /// Dispatches move selection decisions according to the specified evaluation strategy.
    pub fn select_move(result: &SearchProgress, mode: SelectorMode) -> Option<Move> {
        if result.candidates.is_empty() {
            return None;
        }

        // Helper closure to extract the underlying scalar integer value from variant wrappers
        let extract_score_value = |score: EvaluationScore| match score {
            EvaluationScore::Value(v) => v,
            EvaluationScore::Mating(ply) => i32::MAX - (ply as i32),
            EvaluationScore::Mated(ply) => i32::MIN + (ply as i32),
        };

        match mode {
            SelectorMode::Competitive => {
                result.candidates
                    .iter()
                    .max_by_key(|c| extract_score_value(c.score))
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
                        .max_by_key(|c| extract_score_value(c.score))
                        .map(|c| c.current_move)
                }
            }
        }
    }
}