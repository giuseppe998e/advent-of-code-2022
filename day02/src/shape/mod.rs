use std::cmp::Ordering;

mod kind;
pub use kind::ShapeKind;

static ROCK: Shape = Shape {
    kind: ShapeKind::Rock,
    is_rival: false,
};
static PAPER: Shape = Shape {
    kind: ShapeKind::Paper,
    is_rival: false,
};
static SCRISSORS: Shape = Shape {
    kind: ShapeKind::Scissors,
    is_rival: false,
};

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub kind: ShapeKind,
    pub is_rival: bool,
}

impl Shape {
    pub fn new(char: &str) -> Self {
        let kind = ShapeKind::from(char);
        let is_rival = matches!(char, "A" | "B" | "C");
        Self { kind, is_rival }
    }

    pub fn value(&self) -> u32 {
        match self.kind {
            ShapeKind::Rock => 1,
            ShapeKind::Paper => 2,
            ShapeKind::Scissors => 3,
        }
    }

    pub fn winning_shape(&self) -> Self {
        match self.kind {
            ShapeKind::Rock => PAPER,
            ShapeKind::Paper => SCRISSORS,
            ShapeKind::Scissors => ROCK,
        }
    }

    pub fn losing_shape(&self) -> Self {
        match self.kind {
            ShapeKind::Rock => SCRISSORS,
            ShapeKind::Paper => ROCK,
            ShapeKind::Scissors => PAPER,
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Eq for Shape {}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind.cmp(&other.kind)
    }
}
