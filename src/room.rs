use actix::prelude::*;

/* RoomMessage(sender_id, message) */
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage(pub usize, pub String);

/* RoomResponse(receiver_id, message) */
#[derive(MessageResponse)]
pub struct RoomResponse(pub usize, pub String);

/* RoomDestroy */
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomDestroy;

#[derive(Default)]
pub struct RoomLogic;

impl RoomLogic {

}

impl Actor for RoomLogic {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("I'm a new logic actor started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("This room has been destroyed");
    }
}

impl Handler<RoomMessage> for RoomLogic {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, _ctx: &mut Self::Context) {
        println!("Alriht, this is inside a worker: {}", msg.1);
    }
}

impl Handler<RoomDestroy> for RoomLogic {
    type Result = ();

    fn handle(&mut self, _msg: RoomDestroy, ctx: &mut Self::Context) {
        ctx.stop();
    }
}
