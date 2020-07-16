use actix::prelude::*;
use actix_web_actors::ws;

use crate::server::PokerServer;

use crate::message::{PokerMessage, JoinRoom};

type WebsocketMessage = Result<ws::Message, ws::ProtocolError>;

#[derive(Default)]
pub struct PokerSesssion {
    id: usize,
    room: String,
    name: Option<String>
}

impl PokerSesssion {
    fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        PokerServer::from_registry()
            .send(JoinRoom(room_name.to_owned(), ctx.address().recipient()))
            .into_actor(self)
            .then(|a, b, c| {
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Actor for PokerSesssion {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room("lobby", ctx);
    }
}

impl Handler<PokerMessage> for PokerSesssion {
    type Result = ();

    fn handle(&mut self, msg: PokerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<WebsocketMessage> for PokerSesssion {
    fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(msg) => {
                    ctx.text(msg);
                }
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                }
                _ => {}
            }
            return;
        }
        // Close connection if anything happen
        ctx.stop();
    }
}
