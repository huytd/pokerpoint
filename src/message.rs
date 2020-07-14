use actix::prelude::*;

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct PokerMessage(pub String);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct JoinRoom(pub String, pub Option<String>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);
