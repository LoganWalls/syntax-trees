use leptos::{view, IntoView, View};

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub root: Node,
}

impl SyntaxTree {
    pub fn new(mut root: Node) -> Self {
        root.assign_leaves(0.0);
        root.compute_coordinates(0);
        Self { root }
    }

    pub fn iter(&self) -> SyntaxTreeIter<'_> {
        self.into_iter()
    }
}

pub struct SyntaxTreeIter<'a> {
    n1: Option<&'a Node>,
    n2: Option<&'a Node>,
}

impl<'a> Iterator for SyntaxTreeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.n1.take().or_else(|| self.n2.take())?;

        if let NodeKind::Subtree { left, right } = &*next.kind {
            self.n1 = Some(left);
            self.n2 = right.as_ref();
        }

        Some(next)
    }
}

impl<'a> IntoIterator for &'a SyntaxTree {
    type Item = &'a Node;
    type IntoIter = SyntaxTreeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SyntaxTreeIter {
            n1: None,
            n2: Some(&self.root),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub category: Token,
    pub kind: Box<NodeKind>,
    pub x: f32,
    pub y: f32,
    pub whitespace: (String, String),
}

impl Node {
    /// Assigns x and y coordinates to this node and its children based on the structure
    /// of the tree.
    /// IMPORTANT: Assumes that leaf node x-coordinates have been assigned already.
    fn compute_coordinates(&mut self, depth: usize) {
        if let NodeKind::Subtree { left, right } = &mut *self.kind {
            left.compute_coordinates(depth + 1);
            if let Some(right) = right {
                right.compute_coordinates(depth + 1);
                self.x = (left.x + right.x) / 2.0
            } else {
                self.x = left.x
            }
        }
        self.y = depth as f32;
    }

    /// Assigns x coordinates to all leaf nodes
    fn assign_leaves(&mut self, mut rightmost_x: f32) -> f32 {
        match &mut *self.kind {
            NodeKind::Leaf { label: _ } => {
                self.x = rightmost_x;
                rightmost_x + 1.0
            }
            NodeKind::Subtree { left, right } => {
                rightmost_x = left.assign_leaves(rightmost_x);
                if let Some(right_node) = right {
                    rightmost_x = right_node.assign_leaves(rightmost_x)
                }
                rightmost_x
            }
        }
    }
}

impl IntoView for Node {
    fn into_view(self) -> View {
        let category = view! {
                <span class="text-sky-500">{self.category.prefix}{self.category.value}{self.category.suffix}</span>
        };

        let nodes = match *self.kind {
            NodeKind::Leaf { label } => view! {
                <span class="text-amber-500">{label.prefix}{label.value}{label.suffix}</span>
            }
            .into_view(),
            NodeKind::Subtree { left, right } => view! {
                {left} {right}
            }
            .into_view(),
        };

        view! { {self.whitespace.0} {"["} {category} {nodes} {"]"} {self.whitespace.1} }.into_view()
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub prefix: String,
    pub suffix: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum NodeKind {
    Leaf { label: Token },
    Subtree { left: Node, right: Option<Node> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_works() {
        let root = Node {
            category: Token {
                prefix: Default::default(),
                suffix: Default::default(),
                value: String::from("i am root"),
            },
            kind: Box::new(NodeKind::Subtree {
                left: Node {
                    category: Token {
                        prefix: Default::default(),
                        suffix: Default::default(),
                        value: String::from("left 1"),
                    },
                    kind: Box::new(NodeKind::Subtree {
                        left: Node {
                            category: Token {
                                prefix: Default::default(),
                                suffix: Default::default(),
                                value: String::from("left 2"),
                            },
                            kind: Box::new(NodeKind::Leaf {
                                label: Token {
                                    prefix: Default::default(),
                                    suffix: Default::default(),
                                    value: String::from("label"),
                                },
                            }),
                            x: 0.0,
                            y: 0.0,
                            whitespace: (Default::default(), Default::default()),
                        },
                        right: Some(Node {
                            category: Token {
                                prefix: Default::default(),
                                suffix: Default::default(),
                                value: String::from("right 2"),
                            },
                            kind: Box::new(NodeKind::Leaf {
                                label: Token {
                                    prefix: Default::default(),
                                    suffix: Default::default(),
                                    value: String::from("label"),
                                },
                            }),
                            x: 0.0,
                            y: 0.0,
                            whitespace: (Default::default(), Default::default()),
                        }),
                    }),
                    x: 0.0,
                    y: 0.0,
                    whitespace: (Default::default(), Default::default()),
                },
                right: Some(Node {
                    category: Token {
                        prefix: Default::default(),
                        suffix: Default::default(),
                        value: String::from("right 1"),
                    },
                    kind: Box::new(NodeKind::Leaf {
                        label: Token {
                            prefix: Default::default(),
                            suffix: Default::default(),
                            value: String::from("label"),
                        },
                    }),
                    x: 0.0,
                    y: 0.0,
                    whitespace: (Default::default(), Default::default()),
                }),
            }),
            x: 0.0,
            y: 0.0,
            whitespace: (Default::default(), Default::default()),
        };

        let tree = SyntaxTree::new(root);
        let mut iter = tree.iter();

        assert_eq!(
            iter.next().map(|node| &*node.category.value),
            Some("i am root")
        );

        assert_eq!(
            iter.next().map(|node| &*node.category.value),
            Some("left 1")
        );

        assert_eq!(
            iter.next().map(|node| &*node.category.value),
            Some("right 1")
        );

        assert_eq!(
            iter.next().map(|node| &*node.category.value),
            Some("left 2")
        );

        assert_eq!(
            iter.next().map(|node| &*node.category.value),
            Some("right 2")
        );
    }
}
