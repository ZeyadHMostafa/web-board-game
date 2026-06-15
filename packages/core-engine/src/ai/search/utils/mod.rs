mod transposition_table;

pub(crate) use transposition_table::{ HashEntryBounds, TranspositionTable};


use crate::ai::EvaluationScore;
/// Helper function to handle perspective inversion for customized EvaluationScore
pub fn invert_score(score: EvaluationScore) -> EvaluationScore {
    match score {
        // Use checked_neg to safely catch and handle i32::MIN overflow
        EvaluationScore::Value(v) => {
            let inverted = v.checked_neg().unwrap_or(i32::MAX);
            EvaluationScore::Value(inverted)
        }
        EvaluationScore::Mating(d) => {
            EvaluationScore::Mated(d + 1)
        },
        EvaluationScore::Mated(d) => {
            EvaluationScore::Mating(d + 1)
        },
    }
}