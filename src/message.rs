use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct PokerMessage(pub usize, pub String, pub String);

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinRoom(pub String, pub Recipient<PokerMessage>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);
