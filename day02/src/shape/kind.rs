use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum ShapeKind {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for ShapeKind {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for ShapeKind {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (ShapeKind::Rock, ShapeKind::Rock)
                | (ShapeKind::Paper, ShapeKind::Paper)
                | (ShapeKind::Scissors, ShapeKind::Scissors)
        )
    }
}

impl Eq for ShapeKind {}

impl Ord for ShapeKind {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // Win
            (ShapeKind::Rock, ShapeKind::Scissors)
            | (ShapeKind::Paper, ShapeKind::Rock)
            | (ShapeKind::Scissors, ShapeKind::Paper) => Ordering::Greater,

            // Draw
            (ShapeKind::Rock, ShapeKind::Rock)
            | (ShapeKind::Paper, ShapeKind::Paper)
            | (ShapeKind::Scissors, ShapeKind::Scissors) => Ordering::Equal,

            // Lose
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for ShapeKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
