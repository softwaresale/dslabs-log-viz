use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, char, space0};
use nom::combinator::{map, not, opt};

use nom::{IResult};
use nom::sequence::{delimited, separated_pair, tuple};
use crate::abstract_object::AbstractValue;
use crate::abstract_object::parser::parse_abstract_object;
use crate::event::node_ev::{CustomEventNode, MessageReceiveEvent, MessageSendEvent, NodeEvent, TimerDeliverEvent, TimerSetEvent};

pub fn parse_node_event(input: &str) -> IResult<&str, NodeEvent> {
    alt((
        parse_message_send,
        parse_message_recv,
        parse_timer_set,
        parse_timer_recv,
        parse_custom_event,
        ))(input)
}

fn parse_message_send(input: &str) -> IResult<&str, NodeEvent> {
    let (remaining, _) = tag("MessageSend")(input)?;
    let sep = tuple((char(','), space0));
    let body = separated_pair(parse_dispatch, sep, parse_abstract_object);
    let (remaining, ((sender, receiver), body)) = delimited(char('('), body, char(')'))(remaining)?;
    let event = MessageSendEvent {
        sender: sender.unwrap(),
        dest: receiver,
        payload: body,
    };

    Ok((remaining, NodeEvent::MsgSend(event)))
}

fn parse_message_recv(input: &str) -> IResult<&str, NodeEvent> {
    let (remaining, _) = tag("MessageReceive")(input)?;
    let sep = tuple((char(','), space0));
    let body = separated_pair(parse_dispatch, sep, parse_abstract_object);
    let (remaining, ((sender, receiver), body)) = delimited(char('('), body, char(')'))(remaining)?;
    let event = MessageReceiveEvent {
        sender: sender.unwrap(),
        dest: receiver,
        payload: body,
    };

    Ok((remaining, NodeEvent::MsgRecv(event)))
}

fn parse_timer_set(input: &str) -> IResult<&str, NodeEvent> {
    let (remaining, _) = tag("TimerSet")(input)?;
    let sep = tuple((char(','), space0));
    let body = separated_pair(parse_dispatch, sep, parse_abstract_object);
    let (remaining, ((_, receiver), body)) = delimited(char('('), body, char(')'))(remaining)?;
    let event = TimerSetEvent {
        dest: receiver,
        payload: body,
    };

    Ok((remaining, NodeEvent::TimerSet(event)))
}

fn parse_timer_recv(input: &str) -> IResult<&str, NodeEvent> {
    let (remaining, _) = tag("TimerReceive")(input)?;
    let sep = tuple((char(','), space0));
    let body = separated_pair(parse_dispatch, sep, parse_abstract_object);
    let (remaining, ((_, receiver), body)) = delimited(char('('), body, char(')'))(remaining)?;
    let event = TimerDeliverEvent {
        dest: receiver,
        payload: body,
    };

    Ok((remaining, NodeEvent::TimerRecv(event)))
}

fn parse_custom_event(input: &str) -> IResult<&str, NodeEvent> {
    let (remaining, event_name) = alphanumeric1(input)?;
    let sep = tuple((char(','), space0));
    let body = separated_pair(parse_dispatch, sep, parse_custom_event_body);
    let (remaining, ((sender, receiver), body)) = delimited(char('('), body, char(')'))(remaining)?;
    let node = CustomEventNode {
        name: event_name.to_string(),
        sender,
        dest: receiver,
        payload: body,
    };
    
    Ok((remaining, NodeEvent::Custom(node)))
}

fn parse_custom_event_body(input: &str) -> IResult<&str, AbstractValue> {

    let not_quote_slash = is_not("\"\\");
    
    alt((
            map(parse_abstract_object, |abstract_obj| AbstractValue::Object(abstract_obj)),
            map(not_quote_slash, |msg: &str| AbstractValue::Symbol(msg.to_string()))
    ))(input)
}

fn parse_dispatch(input: &str) -> IResult<&str, (Option<String>, String)> {
    let (remaining, (send, _, _, _, recv)) = tuple((
        opt(alphanumeric1),
        space0,
        tag("->"),
        space0,
        alphanumeric1
        ))(input)?;

    let val = (send.map(|val| val.to_string()), recv.to_string());
    Ok((remaining, val))
}

#[cfg(test)]
mod tests {
    use crate::event::node_ev::NodeEvent;
    use crate::event::parser::parse_node_event;

    #[test]
    fn parse_custom_event() {
        let event_line = "CommandExecuted(-> server5, PaxosSlotEntry(amoCommand=AMOCommand(command=KVStore.Put(key=client3-1, value=W9U3z7KQ), address=client3, sequenceNum=0), slotStatus=CHOSEN, isExecuted=true, acceptedBallot=Ballot(serverAddress=server1, roundNum=1), acceptors=[server5, server1]))";
        let (_, event) = parse_node_event(event_line).expect("Custom event should parse successfully");
        
        let NodeEvent::Custom(custom_event) = event else {
            panic!("event should have been a custom event")
        };
        
        assert_eq!(custom_event.name, "CommandExecuted");
        assert_eq!(custom_event.dest, "server5");
    }
}
