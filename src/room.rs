use actix::prelude::*;

// RoomBroadcast(sender_id, message)
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomBroadcast(pub usize, pub String);

// RoomDestroy
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomDestroy;

#[derive(Default)]
pub struct RoomLogic;

impl RoomLogic {

}

impl Actor for RoomLogic {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I'm a new logic actor started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("This room has been destroyed");
    }
}

impl Handler<RoomBroadcast> for RoomLogic {
    type Result = ();

    fn handle(&mut self, msg: RoomBroadcast, ctx: &mut Self::Context) {
        println!("Alriht, this is inside a worker: {}", msg.1);
    }
}

impl Handler<RoomDestroy> for RoomLogic {
    type Result = ();

    fn handle(&mut self, msg: RoomDestroy, ctx: &mut Self::Context) {
        ctx.stop();
    }
}
