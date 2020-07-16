use actix::prelude::*;
use std::collections::HashMap;

use crate::message::{PokerMessage, JoinRoom};

type Client = Recipient<PokerMessage>;
type Room = HashMap<usize, Client>;

#[derive(Default)]
pub struct PokerServer {
    rooms: HashMap<String, Room>,
}

impl PokerServer {
    fn add_client_to_room(&mut self, room_name: String, client: Client) -> usize {
        let mut id: usize = rand::random::<usize>();

        // If there's an exists room
        if let Some(room) = self.rooms.get_mut(&room_name) {
            // TODO: Switch to uuid instead of random usize
            loop {
                if room.contains_key(&id) {
                    id = rand::random::<usize>();
                } else {
                    break;
                }
            }

            room.insert(id, client);

            println!("Client joined room {}", id);

            return id;
        }

        // If there's no existing room
        let mut room: Room = HashMap::new();
        room.insert(id, client);

        println!("Client joined room {}", id);

        self.rooms.insert(room_name.to_owned(), room);

        id
    }
}

impl Actor for PokerServer {
    type Context = Context<Self>;
}

impl Handler<JoinRoom> for PokerServer {
    type Result = usize;

    fn handle(&mut self, msg: JoinRoom, ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client) = msg;
        let id = self.add_client_to_room(room_name, client);
        id
    }
}

impl Handler<PokerMessage> for PokerServer {
    type Result = ();

    fn handle(&mut self, msg: PokerMessage, ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.rooms.get("lobby") {
            // TODO: Find a way to remove dead client in this step
            for (id, client) in room {
                if &msg.0 != id {
                    if client.do_send(msg.to_owned()).is_err() {
                        println!("This client is dead {}", id);
                    }
                }
            }
        }
    }
}

impl SystemService for PokerServer {}
impl Supervised for PokerServer {}
