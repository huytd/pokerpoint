use actix::prelude::*;

/* REQUEST */

/* PokerMessage(sender-id, room_name, message) */
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct PokerMessage(pub usize, pub String, pub String);

/* JoinRoom(room_name, client) */
#[derive(Clone, Message)]
#[rtype(result = "(JoinResult)")]
pub struct JoinRoom(pub String, pub Recipient<PokerMessage>);

/*  LeaveRoom(room_name, user_id) */
#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct LeaveRoom(pub String, pub usize);

/* RESPONSE */

#[derive(MessageResponse)]
pub struct JoinResult(pub usize, pub String);
