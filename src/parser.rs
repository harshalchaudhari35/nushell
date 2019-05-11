use nom::branch::alt;
use nom::bytes::complete::{escaped, is_not, tag};
use nom::{ws, named, separated_list, complete};
use nom::character::complete::one_of;
use nom::multi::separated_list;
use nom::sequence::{preceded, terminated};
use nom::IResult;

#[derive(Debug)]
pub enum Item {
    Quoted(String),
    Bare(String),
}

impl Item {
  crate fn name(&self) -> &str {
    match self {
      Item::Quoted(s) => s,
      Item::Bare(s) => s
    }
  }
}

fn esc(s: &str) -> IResult<&str, &str> {
    escaped(is_not("\\\""), '\\', one_of("\"n\\"))(s)
}

fn quoted(s: &str) -> IResult<&str, Item> {
    terminated(preceded(tag("\""), esc), tag("\""))(s)
        .map(|(a, b)| (a, Item::Quoted(b.to_string())))
}

fn unquoted(s: &str) -> IResult<&str, Item> {
    is_not(" |")(s).map(|(a, b)| (a, Item::Bare(b.to_string())))
}

fn command_token(s: &str) -> IResult<&str, Item> {
    alt((quoted, unquoted))(s)
}

fn command_args(s: &str) -> IResult<&str, Vec<Item>> {
    separated_list(tag(" "), command_token)(s)
}

named!(
  pub shell_parser(&str) -> Vec<Vec<Item>>,
  complete!(
    ws!(
      separated_list!(tag("|"), command_args)
    )
  )
);
