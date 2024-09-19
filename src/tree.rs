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
    pub category: String,
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
