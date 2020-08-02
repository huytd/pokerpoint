use actix::prelude::*;
use std::collections::HashMap;

use crate::message::{PokerMessage, JoinRoom, LeaveRoom, JoinResult};
use crate::room_logic::{RoomLogic, RoomDestroy, RoomMessage, RoomResponse};

type Client = Recipient<PokerMessage>;
struct Room {
    users: HashMap<usize, Client>,
    logic: Addr<RoomLogic>
}

#[derive(Default)]
pub struct PokerServer {
    rooms: HashMap<String, Room>,
}

// TODO: The nameing of messages referred in this class seems
// messy, need to re-organize for more meaningful names.

impl PokerServer {
    fn add_client_to_room(&mut self, room_name: String, client: Client) -> usize {
        let mut id: usize = rand::random::<usize>();

        // If there's an exists room
        if let Some(room) = self.rooms.get_mut(&room_name) {
            loop {
                if room.users.contains_key(&id) {
                    id = rand::random::<usize>();
                } else {
                    break;
                }
            }

            room.users.insert(id, client);
            println!("Client joined room {}", id);

            return id;
        }

        // If there's no existing room
        let room_logic = RoomLogic.start();
        let mut room: Room = Room {
            users: HashMap::new(),
            logic: room_logic
        };

        room.users.insert(id, client);
        println!("Client joined room {}", id);

        self.rooms.insert(room_name.to_owned(), room);

        id
    }

    fn remove_client_from_room(&mut self, room_name: String, id: usize) {
        if let Some(room) = self.rooms.get_mut(&room_name) {
            room.users.remove(&id);
            if room.users.len() <= 0 {
                room.logic.do_send(RoomDestroy);
                self.rooms.remove(&room_name);
                println!("Removed roomn {}", &room_name);
            }
        }
    }

    fn broadcast(&self, room: &Room, msg: String) {
        let clients = &room.users;
        for (id, client) in clients {
            let msg = PokerMessage(0, String::new(), msg.to_owned());
            if client.do_send(msg).is_err() {
                println!("This client is dead {}", id);
            }
        }
    }

    fn message_to_user(&self, room: &Room, user_id: usize, msg: String) {
        let clients = &room.users;
        let msg = PokerMessage(0, String::new(), msg.to_owned());
        for (id, client) in clients {
            if id == &user_id {
                if client.do_send(msg.clone()).is_err() {
                    println!("This client is dead {}", id);
                }
            }

        }
    }
}

impl Actor for PokerServer {
    type Context = Context<Self>;
}

// TODO: Join room need to specify user name

impl Handler<JoinRoom> for PokerServer {
    type Result = JoinResult;

    fn handle(&mut self, msg: JoinRoom, _ctx: &mut Self::Context) -> Self::Result {
        let JoinRoom(room_name, client) = msg;
        let joined_room = room_name.to_owned();
        let id = self.add_client_to_room(room_name, client);
        JoinResult(id, joined_room)
    }
}

impl Handler<LeaveRoom> for PokerServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _ctx: &mut Self::Context) -> Self::Result {
        let LeaveRoom(room_name, id) = msg;
        self.remove_client_from_room(room_name, id);
    }
}

// handle message from session and transfer to Room Logic
impl Handler<PokerMessage> for PokerServer {
    type Result = ();

    fn handle(&mut self, msg: PokerMessage, _ctx: &mut Self::Context) -> Self::Result {
        let PokerMessage(id, room_name, message) = msg;
        if let Some(room) = self.rooms.get(&room_name) {
            room.logic.do_send(RoomMessage(id.clone(), room_name.clone(), message.clone()));
        }
    }
}

// Handle message from Room Logic and figure out what to do next
impl Handler<RoomResponse> for PokerServer {
    type Result = ();

    fn handle(&mut self, msg: RoomResponse, _ctx: &mut Self::Context) -> Self::Result {
        let RoomResponse(receiver_id, room_name, message) = msg;
        if let Some(room) = self.rooms.get(&room_name) {
            if receiver_id == 0 {
                // Broadcast to everyone
                self.broadcast(&room, message);
            } else {
                // Only send to an individuall
                self.message_to_user(&room, receiver_id, message);
            }
        }
    }
}

impl SystemService for PokerServer {}
impl Supervised for PokerServer {}
