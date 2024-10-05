use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::sequence::{delimited, pair, Tuple};
use nom::IResult;

use crate::tree::{Node, NodeKind, SyntaxTree, Token};

fn node_content(input: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n[]")(input)
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    let (remaining, (node_prefix, _, category_prefix)) =
        (multispace0, tag("["), multispace0).parse(input)?;
    let (remaining, (category, category_suffix)) = (node_content, multispace0).parse(remaining)?;

    let (remaining, kind) =
        if let Ok((r, (label, label_suffix))) = (node_content, multispace0).parse(remaining) {
            (
                r,
                NodeKind::Leaf {
                    label: Token {
                        prefix: String::new(),
                        suffix: label_suffix.to_owned(),
                        value: label.to_string(),
                    },
                },
            )
        } else {
            let (r, (left, right)) = pair(parse_node, opt(parse_node))(remaining)?;
            (r, NodeKind::Subtree { left, right })
        };

    let (remaining, (.., node_suffix)) = (multispace0, tag("]"), multispace0).parse(remaining)?;
    Ok((
        remaining,
        Node {
            category: Token {
                prefix: category_prefix.to_owned(),
                suffix: category_suffix.to_owned(),
                value: category.to_string(),
            },
            kind: Box::new(kind),
            x: 0.0,
            y: 0.0,
            whitespace: (node_prefix.to_owned(), node_suffix.to_owned()),
        },
    ))
}

pub fn parse_syntax_tree(input: &str) -> IResult<&str, SyntaxTree> {
    let (remaining, root) = delimited(multispace0, parse_node, multispace0)(input)?;
    Ok((remaining, SyntaxTree::new(root)))
}
