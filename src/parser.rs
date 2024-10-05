use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::sequence::{delimited, pair, Tuple};
use nom::IResult;

use crate::tree::{Node, NodeKind, SyntaxTree};

fn node_content(input: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n[]")(input)
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    let (remaining, _) = (multispace0, tag("["), multispace0).parse(input)?;
    let (remaining, category) = delimited(multispace0, node_content, multispace0)(remaining)?;

    let (remaining, kind) = if let Ok((r, label)) = node_content(remaining) {
        (
            r,
            NodeKind::Leaf {
                label: label.to_string(),
            },
        )
    } else {
        let (r, (left, right)) = pair(parse_node, opt(parse_node))(remaining)?;
        (r, NodeKind::Subtree { left, right })
    };

    let (remaining, _) = (multispace0, tag("]")).parse(remaining)?;
    Ok((
        remaining,
        Node {
            category: category.to_string(),
            kind: Box::new(kind),
            x: 0.0,
            y: 0.0,
        },
    ))
}

pub fn parse_syntax_tree(input: &str) -> IResult<&str, SyntaxTree> {
    let (remaining, root) = delimited(multispace0, parse_node, multispace0)(input)?;
    Ok((remaining, SyntaxTree::new(root)))
}
