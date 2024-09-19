use std::fmt::Display;
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Copy, Debug)]
pub enum LexicalCategory {
    N,
    NP,
    V,
    VP,
    Det,
    A,
    P,
    PP,
    C,
    CP,
    S,
}

impl Display for LexicalCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LexicalCategory::N => "N".to_string(),
            LexicalCategory::NP => "NP".to_string(),
            LexicalCategory::V => "V".to_string(),
            LexicalCategory::VP => "VP".to_string(),
            LexicalCategory::Det => "Det".to_string(),
            LexicalCategory::A => "A".to_string(),
            LexicalCategory::P => "P".to_string(),
            LexicalCategory::PP => "PP".to_string(),
            LexicalCategory::C => "C".to_string(),
            LexicalCategory::CP => "CP".to_string(),
            LexicalCategory::S => "S".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl FromStr for LexicalCategory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(LexicalCategory::N),
            "NP" => Ok(LexicalCategory::NP),
            "V" => Ok(LexicalCategory::V),
            "VP" => Ok(LexicalCategory::VP),
            "Det" => Ok(LexicalCategory::Det),
            "A" => Ok(LexicalCategory::A),
            "P" => Ok(LexicalCategory::P),
            "PP" => Ok(LexicalCategory::PP),
            "C" => Ok(LexicalCategory::C),
            "CP" => Ok(LexicalCategory::CP),
            "S" => Ok(LexicalCategory::S),
            _ => Err(anyhow! {"Invalid lexical category"}),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub root: Node,
}

impl SyntaxTree {
    pub fn new(mut root: Node) -> Self {
        root._assign_leaves(0.0);
        root._compute_coordinates(0);
        Self { root }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub category: LexicalCategory,
    pub kind: Box<NodeKind>,
    pub x: f32,
    pub y: f32,
}

impl Node {
    /// Assigns x and y coordinates to this node and its children based on the structure
    /// of the tree.
    /// IMPORTANT: Assumes that leaf node x-coordinates have been assigned already.
    fn _compute_coordinates(&mut self, depth: usize) {
        if let NodeKind::Subtree { left, right } = &mut *self.kind {
            left._compute_coordinates(depth + 1);
            if let Some(right) = right {
                right._compute_coordinates(depth + 1);
                self.x = (left.x + right.x) / 2.0
            } else {
                self.x = left.x
            }
        }
        self.y = depth as f32;
    }

    /// Assigns x coordinates to all leaf nodes
    fn _assign_leaves(&mut self, mut rightmost_x: f32) -> f32 {
        match &mut *self.kind {
            NodeKind::Leaf { label: _ } => {
                self.x = rightmost_x;
                rightmost_x + 1.0
            }
            NodeKind::Subtree { left, right } => {
                rightmost_x = left._assign_leaves(rightmost_x);
                if let Some(right_node) = right {
                    rightmost_x = right_node._assign_leaves(rightmost_x)
                }
                rightmost_x
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    Leaf { label: String },
    Subtree { left: Node, right: Option<Node> },
}
