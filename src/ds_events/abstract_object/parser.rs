use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag};
use nom::character::complete::{alpha1, alphanumeric1, char, i64, space0};
use nom::combinator::{map, peek, recognize};
use nom::IResult;
use nom::multi::{many0_count, many1, separated_list0};
use nom::sequence::{delimited, pair, tuple};
use crate::ds_events::abstract_object::{AbstractObject, AbstractValue};

pub fn parse_abstract_object(input: &str) -> IResult<&str, AbstractObject> {
    let (remainder, name) = parse_abstract_object_name(input)?;
    let (remainder, props) = delimited(char('('), parse_prop_list, char(')'))(remainder)?;
    let obj = AbstractObject::new_complete(name, props);
    Ok((remainder, obj))
}

fn parse_abstract_object_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_"), tag("."))))
    ))(input)
}

fn parse_prop_list(input: &str) -> IResult<&str, HashMap<String, AbstractValue>> {

    let separator = tuple((char(','), space0));

    let (remaining, props) = separated_list0(separator, parse_prop)(input)?;
    let props = props.into_iter()
        .collect::<HashMap<_, _>>();

    Ok((remaining, props))
}

fn parse_prop(input: &str) -> IResult<&str, (String, AbstractValue)> {
    let (cont, (name, _, value)) = tuple((parse_prop_name, char('='), parse_prop_value))(input)?;
    Ok((cont, (String::from(name), value)))
}


fn parse_prop_name(input: &str) -> IResult<&str, &str> {
    alt((
        parse_abstract_object_name,
        alphanumeric1,
        ))(input)
}

fn parse_prop_value(input: &str) -> IResult<&str, AbstractValue> {
    let symbol_parser = recognize(many1(is_not(",)]")));
    let complete_number_prop = map(pair(i64, peek(is_a(",)]"))), |(left, _)| left);
    alt((
        map(parse_abstract_object, AbstractValue::from),
        map(parse_map, AbstractValue::from),
        map(parse_collection, AbstractValue::from),
        map(complete_number_prop, AbstractValue::from),
        map(symbol_parser, AbstractValue::from),
        ))(input)
}

fn parse_map(input: &str) -> IResult<&str, HashMap<String, AbstractValue>> {
    delimited(char('{'), parse_prop_list, char('}'))(input)
}

fn parse_collection(input: &str) -> IResult<&str, Vec<AbstractValue>> {
    let separator = tuple((char(','), space0));
    let list_parser = separated_list0(separator, parse_prop_value);
    delimited(char('['), list_parser, char(']'))(input)
}

#[cfg(test)]
mod tests {
    use nom::bytes::complete::is_not;
    use nom::combinator::recognize;
    use nom::error::Error;
    use nom::multi::many1;
    use crate::ds_events::abstract_object::AbstractValue;
    use crate::ds_events::abstract_object::parser::{parse_abstract_object, parse_collection, parse_prop_value};

    #[test]
    fn parse_obj_succ_1() {
        let (_, obj) = parse_abstract_object("ViewReply(view=View(viewNum=2, primary=server1, backup=server2))").expect("Should parse successfully");
        assert_eq!(obj.name, "ViewReply");
        assert_eq!(obj.props.len(), 1);
        let AbstractValue::Object(view_obj) = obj.props.get("view").unwrap() else {
            panic!("view should be object");
        };

        assert_eq!(view_obj.name, "View");
        assert_eq!(view_obj.props.len(), 3);

        assert_eq!(view_obj.props.get("viewNum").unwrap(), &AbstractValue::Number(2));
        assert_eq!(view_obj.props.get("primary").unwrap(), &AbstractValue::Symbol(String::from("server1")));
        assert_eq!(view_obj.props.get("backup").unwrap(), &AbstractValue::Symbol(String::from("server2")));
    }
    
    #[test]
    fn parse_event_with_collection() {
        let (_, obj) = parse_abstract_object("PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Put(key=client3-1, value=W9U3z7KQ), address=client3, sequenceNum=0), slotStatus=CHOSEN, isExecuted=true, acceptedBallot=Ballot(serverAddress=server1, roundNum=1), acceptors=[server1, server4, server5, server2])").expect("Should parse successfully");
        assert_eq!(obj.name, "PaxosSlotEntry");
        assert_eq!(obj.props.len(), 5);
        let acceptors_prop = obj.props().get("acceptors").expect("acceptors should be a property");
        let AbstractValue::Collection(collection) = acceptors_prop else {
            panic!("acceptors is not a collection");
        };
        
        assert_eq!(collection.len(), 4)
    }
    
    #[test]
    fn parse_strange_value() {
        let text = "uocFqRu),";
        let mut symbol_parser = recognize(many1(is_not::<&str, &str, Error<&str>>(",)]")));
        let (_, text) = symbol_parser(text).expect("Underlying symbol parser should parse");
        assert_eq!(text, "uocFqRu");
        
        let (_, text) = parse_prop_value(text).expect("should parse as prop value");
        let AbstractValue::Symbol(internal_value) = text else {
            panic!("Expected text to be symbol")
        };
        assert_eq!(internal_value, "uocFqRu");
    }
    
    #[test]
    fn parse_event() {
        let text = "PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Put(key=client5-5, value=7uocFqRu), address=client5, sequenceNum=72), slotStatus=CHOSEN, isExecuted=true, acceptedBallot=Ballot(serverAddress=server1, roundNum=1), acceptors=[server2, server1])";
        let (_, _obj) = parse_abstract_object(text).expect("Should parse correctly");
    }
    
    #[test]
    fn parse_num_collection() {
        let text = "[1, 2, 3]";
        let (_, items) = parse_collection(text).expect("should parse successfully");
        assert_eq!(items.len(), 3);
        let AbstractValue::Number(first_item) = items.get(0).expect("first element should be something") else {
            panic!("element was not a number")
        };
        assert_eq!(first_item, &1)
    }
    
    #[test]
    fn parses_abstract_obj_name() {
        let text = "EventLogger.InsertedIntoLog(slotNum=2662, entry=PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Get(key=client2-3), address=client2, sequenceNum=635), slotStatus=ACCEPTED, isExecuted=false, acceptedBallot=Ballot(serverAddress=server5, roundNum=5), sentToClient=false, acceptors=[server1, server5]))";
        let (_, item) = parse_abstract_object(text).expect("Parsing should not fail");
        assert_eq!(item.name(), "EventLogger.InsertedIntoLog");
        let slot_num = item.props().get("slotNum").expect("should have slotNum property");
        let AbstractValue::Number(num_) = slot_num else {
            panic!("expected slot num to be a number, but was not");
        };
        
        assert_eq!(*num_, 2662);
    }
}
