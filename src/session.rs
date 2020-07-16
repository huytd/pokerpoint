use actix::prelude::*;
use actix_web_actors::ws;

use crate::server::PokerServer;

use crate::message::{PokerMessage, JoinRoom, LeaveRoom};

type WebsocketMessage = Result<ws::Message, ws::ProtocolError>;

#[derive(Default, Debug)]
pub struct PokerSesssion {
    id: usize,
    room: String,
    name: Option<String>
}

impl PokerSesssion {
    fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name= room_name.to_owned();
        PokerServer::from_registry()
            .send(JoinRoom(room_name.to_owned(), ctx.address().recipient()))
            .into_actor(self)
            .then(|id, session, _context| {
                if let Ok(id) = id {
                    session.id = id;
                    session.room = room_name;
                    println!("JOINED {:?} {:?}", id, session);
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn leave_room(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = self.room.to_owned();
        PokerServer::from_registry()
            .send(LeaveRoom(room_name, self.id))
            .into_actor(self)
            .then(|_, session, _context| {
                session.id = 0;
                fut::ready(())
            })
        .wait(ctx);
    }

    fn send_message(&mut self, msg: String) {
        let room_name = self.room.to_owned();
        PokerServer::from_registry()
            .do_send(PokerMessage(self.id, room_name, msg));
    }
}

impl Actor for PokerSesssion {
    type Context = ws::WebsocketContext<Self>;
}

impl Handler<PokerMessage> for PokerSesssion {
    type Result = ();

    fn handle(&mut self, msg: PokerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.2);
    }
}

impl StreamHandler<WebsocketMessage> for PokerSesssion {
    fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) {
        if let Ok(msg) = msg {
            match msg {
                ws::Message::Text(msg) => {
                    // TODO: Handle commands here
                    let parts = msg.split_whitespace().collect::<Vec<&str>>();
                    let command = parts[0];
                    if command.starts_with("/") {
                        // We got a command
                        match command {
                            "/join" => {
                                let room_name = parts[1..].join(" ");
                                self.join_room(&room_name, ctx);
                            }
                            "/leave" => {
                                self.leave_room(ctx);
                            }
                            _ => {}
                        }
                    } else {
                        // Broadcast message to others
                        self.send_message(msg);
                    }
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
