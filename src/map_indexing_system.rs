
use specs::prelude::*;
use super::{Map, Position};

pub struct MapIndexingSystem {}

impl <'a> System <'a> for MapIndexingSystem {
    type SystemData =
        WriteExpect<'a, Map>;


    fn run(&mut self, data : Self::SystemData) {
        let mut map = data;

        map.populate_blocked();
        //TODO: clear content
        //TODO: add entity blocking

    }
}
