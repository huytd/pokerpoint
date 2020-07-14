use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use std::collections::HashMap;

use crate::message::{PokerMessage};

type Client = Recipient<PokerMessage>;
type Room = HashMap<usize, Client>;

#[derive(Default)]
struct PokerServer {
    rooms: HashMap<usize, Room>,
}

impl Actor for PokerServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<PokerMessage>(ctx);
    }
}

impl SystemService for PokerServer {}
impl Supervised for PokerServer {}
