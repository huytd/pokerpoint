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

// Communication to Server
impl PokerSesssion {
    fn request_leave(&mut self) -> Request<PokerServer, LeaveRoom> {
        let room_name = self.room.to_owned();
        PokerServer::from_registry()
            .send(LeaveRoom(room_name, self.id))
    }

    fn send_message(&mut self, msg: String) {
        let room_name = self.room.to_owned();
        PokerServer::from_registry()
            .do_send(PokerMessage(self.id, room_name, msg));
    }

    fn request_join(&mut self, room_name: String, ctx: &mut ws::WebsocketContext<Self>) -> Request<PokerServer, JoinRoom> {
        let next_room = room_name;
        let recipient = ctx.address().recipient();
        PokerServer::from_registry()
            .send(JoinRoom(next_room, recipient))
    }
}

// Handle Session Logic
impl PokerSesssion {
    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn join_room(&mut self, room_name: String, ctx: &mut ws::WebsocketContext<Self>) {
        let leave_request = self.request_leave();
        let join_request = self.request_join(room_name, ctx);
        let action = async move {
            leave_request.await?;
            join_request.await
        };

        actix::fut::wrap_future::<_, Self>(action)
            .map(|result, session, _ctx| {
                if let Ok(result) = result {
                    session.id = result.0;
                    session.room = result.1;
                }
            })
            .wait(ctx);
    }

    fn leave_room(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.request_leave()
            .into_actor(self)
            .then(|_, session, _context| {
                session.id = 0;
                fut::ready(())
            })
        .wait(ctx);
    }

    fn disconnect(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.request_leave()
            .into_actor(self)
            .then(|_, session, context| {
                session.id = 0;
                context.close(None);
                context.stop();
                fut::ready(())
            })
        .wait(ctx);
    }
}

impl Actor for PokerSesssion {
    type Context = ws::WebsocketContext<Self>;

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Client disconnected");
    }
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
                    let parts = msg.split_whitespace().collect::<Vec<&str>>();
                    let command = parts[0];
                    if command.starts_with("/") {
                        // We got a command
                        match command {
                            "/join" => {
                                let room_name = parts[1..].join(" ");
                                self.join_room(room_name, ctx);
                            }
                            "/leave" => {
                                self.leave_room(ctx);
                            }
                            "/name" => {
                                let name = parts[1..].join(" ");
                                self.set_name(name);
                            }
                            _ => {}
                        }
                    } else {
                        // Broadcast message to others
                        let sender = self.name.to_owned().unwrap_or_default();
                        let msg = format!("[{}]: {}", sender, msg);
                        self.send_message(msg);
                    }
                }
                ws::Message::Close(_reason) => {
                    self.disconnect(ctx);
                }
                _ => {}
            }
            return;
        }
        // Close connection if anything happen
        self.disconnect(ctx);
    }
}
