use nom::bytes::complete::{is_not, tag};
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::ParseError;
use nom::sequence::{delimited, pair, Tuple};
use nom::IResult;

use crate::tree::{LexicalCategory, Node, NodeKind, SyntaxTree};

fn non_space(input: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(input)
}

fn node_label(input: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n[]")(input)
}

pub fn parse_category(input: &str) -> IResult<&str, LexicalCategory> {
    let (remaining, result) = non_space(input)?;
    let category = result.parse().map_err(|_| {
        nom::Err::Failure(ParseError::from_error_kind(
            result,
            nom::error::ErrorKind::Fail,
        ))
    })?;
    Ok((remaining, category))
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    let (remaining, _) = (multispace0, tag("["), multispace0).parse(input)?;
    let (mut remaining, category) = delimited(multispace0, parse_category, multispace0)(remaining)?;

    let kind;
    if let Ok((r, label)) = node_label(remaining) {
        remaining = r;
        kind = Box::new(NodeKind::Leaf {
            label: label.to_string(),
        });
    } else {
        let (r, (left, right)) = pair(parse_node, opt(parse_node))(remaining)?;
        remaining = r;
        kind = Box::new(NodeKind::Subtree { left, right });
    }

    let (remaining, _) = (multispace0, tag("]")).parse(remaining)?;
    Ok((
        remaining,
        Node {
            category,
            kind,
            x: 0.0,
            y: 0.0,
        },
    ))
}

pub fn parse_syntax_tree(input: &str) -> IResult<&str, SyntaxTree> {
    let (remaining, root) = delimited(multispace0, parse_node, multispace0)(input)?;
    Ok((remaining, SyntaxTree::new(root)))
}
