use crate::{ai::EvaluationScore, rules::moves::{Move, MoveList}};

pub struct SearchFrame {
    /// Tracks the index of the move currently being evaluated within the legal moves list.
    pub move_idx: usize,
    /// Stores the pre-allocated list of legal moves generated for this specific position.
    pub legal_moves: MoveList,
    /// The current lower bound score for this node.
    pub alpha: EvaluationScore,
    /// The current upper bound score for this node.
    pub beta: EvaluationScore,
    /// Tracks the highest score discovered among explored child nodes.
    pub max_score: EvaluationScore,
    /// Keeps track of the original alpha value to determine exact, lower, or upper bounds for the transposition table.
    pub original_alpha: EvaluationScore,
    /// keeps track of best line of moves 
    pub pv_line: Vec<Move>,
}

#[derive(Debug)]
pub enum StepResult {
    /// The state machine needs to evaluate a child node, requiring a step deeper into the tree.
    Deepen,
    /// The state machine completed a node evaluation and is backing up to the parent frame.
    Backtrack { 
        score: EvaluationScore,
        pv: Vec<Move>,
    },
    /// The search space has been completely traversed.
    Done {
        best_score: EvaluationScore,
        pv: Vec<Move>,
    },
}

