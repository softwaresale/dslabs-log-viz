use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};
use nom::combinator::{map, recognize};
use nom::error::ParseError;
use nom::{IResult, Parser};
use nom::multi::{many0, many0_count, separated_list1};
use nom::sequence::{delimited, pair, separated_pair};
use crate::dsl::query_ast::{EventNameFilter, EventQuery, FindEventNode, Operator, PropPath};

pub fn parse_event_query(input: &str) -> IResult<&str, EventQuery> {
    map(
        pair(ws(tag("find")), parse_find_events_query),
        |(_, find_events)| EventQuery::Find { queries: find_events }
    )(input)
}

fn parse_find_events_query(input: &str) -> IResult<&str, Vec<FindEventNode>> {
    map(parse_find_event_query, |ev| vec![ev])(input)
}

fn parse_find_event_query(input: &str) -> IResult<&str, FindEventNode> {
    let (remaining, filter_name) = parse_event_name_filter(input)?;
    let (remaining, op) = parse_operator(remaining)?;
    Ok((remaining, FindEventNode {
        event_type: filter_name,
        operator: op
    }))
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((
        parse_eq_op,
        parse_has_op,
        parse_not_op,
        parse_and_ops,
        parse_or_ops,
        parse_server_op
        ))(input)
}

fn parse_or_ops(input: &str) -> IResult<&str, Operator> {
    let (remaining, _) = tag("or")(input)?;
    let body_parser = separated_list1(ws(char(';')), parse_operator);
    let (remaining, ops) = delimited(ws(char('{')), body_parser, ws(char('}')))(remaining)?;

    Ok((remaining, Operator::Or(ops)))
}

fn parse_and_ops(input: &str) -> IResult<&str, Operator> {

    let (remaining, _) = tag("and")(input)?;
    let body_parser = separated_list1(ws(char(';')), parse_operator);
    let (remaining, ops) = delimited(ws(char('{')), body_parser, ws(char('}')))(remaining)?;

    Ok((remaining, Operator::And(ops)))
}

fn parse_not_op(input: &str) -> IResult<&str, Operator> {
    let (remaining, (_, op)) = pair(
        ws(tag("not")),
        delimited(char('('), parse_operator, char(')'))
    )(input)?;

    Ok((remaining, Operator::Not(Box::new(op))))
}

fn parse_eq_op(input: &str) -> IResult<&str, Operator> {
    // TODO this needs to accept numbers vs strings etc.
    let (remaining, _) = tag("eq")(input)?;
    let params = separated_pair(parse_path, ws(char(',')), parse_eq_value);
    let (remaining, (prop_name, value)) = delimited(char('('), params, char(')'))(remaining)?;

    Ok((remaining, Operator::Eq { prop_name, comparison: value.to_string() }))
}

fn parse_server_op(input: &str) -> IResult<&str, Operator> {
    let (remaining, _) = tag("server")(input)?;
    let (remaining, server_id) = delimited(char('('), parse_eq_value, char(')'))(remaining)?;

    Ok((remaining, Operator::Server(server_id.to_string())))
}

fn parse_eq_value(input: &str) -> IResult<&str, &str> {
    alt((
        parse_str_value,
        recognize(nom::character::complete::i64)
        ))(input)
}

fn parse_str_value(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), recognize(many0(is_not("\""))), char('"'))(input)
}

fn parse_has_op(input: &str) -> IResult<&str, Operator> {

    let (remaining, (_, ident)) = pair(
        ws(tag("has")),
        delimited(char('('), parse_path, char(')'))
    )(input)?;

    Ok((remaining, Operator::Has(ident)))
}

fn parse_event_name_filter(input: &str) -> IResult<&str, EventNameFilter> {
    alt((
        map(ws(tag("any")), |_| EventNameFilter::Any),
        map(ws(parse_ident), |name| EventNameFilter::Named(name.to_string()))
        ))(input)
}

fn parse_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"), tag("."))))
    ))(input)
}

fn parse_path_segment(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"))))
    ))(input)
}

fn parse_path(input: &str) -> IResult<&str, PropPath> {
    let (remaining, segments) = separated_list1(char('.'), parse_path_segment)(input)?;
    let segments = segments.into_iter()
        .map(|seg| seg.to_string())
        .collect::<Vec<_>>();
    
    Ok((remaining, PropPath { segments }))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}

#[cfg(test)]
mod tests {
    use crate::dsl::parser::{parse_eq_op, parse_event_query};
    use crate::dsl::query_ast::{EventNameFilter, EventQuery, Operator};

    #[test]
    fn find_event_parse() {
        let ev_text = "find any has(slotNum)";
        let (_, query) = parse_event_query(ev_text).expect("Parsing should succeed");
        let EventQuery::Find { queries } = query;

        assert_eq!(queries.len(), 1);
        let first = queries.get(0).expect("should have a first element");
        let EventNameFilter::Any = first.event_type else { panic!("Expected query filter to be any") };
        match &first.operator {
            Operator::Has(prop) => assert_eq!(prop.segments[0], "slotNum"),
            op => panic!("Unexpected operator type: {:?}", op),
        }
    }

    #[test]
    fn find_event_parse_and_body() {
        let ev_text = "find any and { has(slotNum); eq(slotNum, 1824) }";
        let (_, _query) = parse_event_query(ev_text).expect("Parsing should succeed");
    }

    #[test]
    fn find_event_parse_and_and_or() {
        let ev_text = "
        find EventLogger.InsertIntoLog and {
            has(slotNum);
            or {
                eq(slotNum, 1928);
                eq(slotNum, 33)
            }
        }
        ";

        let (_, _query) = parse_event_query(ev_text).expect("Parsing should succeed");
    }
    
    #[test]
    fn parse_path() {
        let text = "eq(leader.area, 10)";
        let (_, path) = parse_eq_op(text).expect("Should parse successfully");
        let Operator::Eq { prop_name, comparison } = path else {
            panic!("Expected eq, but got something else");
        };
        
        assert_eq!(prop_name.segments.len(), 2);
        assert_eq!(prop_name.segments.get(0).unwrap(), "leader");
        assert_eq!(prop_name.segments.get(1).unwrap(), "area");
    }
}
