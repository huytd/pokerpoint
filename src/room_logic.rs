use actix::prelude::*;

/** TODO: This class now serving a very specific purpose
 * for now, but in the future, it need to be abstracted
 * as an interface for room logic, to allow the developer
 * to create many different room logic.
 */

/* RoomMessage(sender_id, room_name, message) */
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage(pub usize, pub String, pub String);

/* RoomResponse(receiver_id, room_name, message) */
#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomResponse(pub usize, pub String, pub String);

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
        let RoomMessage(sender_id, room_name, message) = msg;
        // TODO: Handle logic message from here
    }
}

impl Handler<RoomDestroy> for RoomLogic {
    type Result = ();

    fn handle(&mut self, _msg: RoomDestroy, ctx: &mut Self::Context) {
        ctx.stop();
    }
}
