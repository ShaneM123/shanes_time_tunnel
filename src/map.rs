use rltk::{Rltk, RGB, RandomNumberGenerator, Algorithm2D, Point, BaseMap};
use std::cmp::{max, min};
use crate::{player::Player, rect::Rect};
use specs::{World, Join, WorldExt, Entity};

const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 43;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq,Copy,Clone)]
pub enum TileType{
    Wall, Floor
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}
impl Map{
    pub fn xy_idx(&self, x: i32, y: i32) -> usize { (y as usize * MAPWIDTH) + x as usize}

    fn apply_room_to_map(&mut self, room: &Rect){
        for y in room.y1 +1 ..=room.y2{
            for x in room.x1 +1 ..= room.x2 {
                let idx = self.xy_idx(x,y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        self.apply_tunnel(x1,x2,y,true)
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        self.apply_tunnel( y1, y2, x, false );
    }

    fn apply_tunnel(&mut self, coord1: i32, coord2: i32, coord3: i32, horz: bool){
        for i in min(coord1, coord2) ..= max(coord1, coord2) {
            let mut idx: usize = 0;
            if horz == true {         idx = self.xy_idx(i, coord3);
            }
            else{
                idx = self.xy_idx(coord3, i );
            }
            if idx > 0 && idx < self.width as usize * self.height as usize  {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }

    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map::default();

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, ( MAPWIDTH as i32) - w - 1) -1;
            let y = rng.roll_dice(1, (MAPHEIGHT as i32) -h -1 ) -1;
            let new_room = Rect::new(x,y,w,h);
            if map.rooms.iter().any(|x| x.intersect(&new_room)){continue;}
            map.apply_room_to_map(&new_room);
            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                //TODO: apply more interesting corridors
                if rng.range(0,2) ==1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y,new_y,new_x);
                }
                else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }
            map.rooms.push(new_room);
        }
        map
    }
    //TODO: implement entity tile blocking and map system logic

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
    // pub fn clear_content_index(&mut self) {
    //     for content in self.tile_content.iter_mut() {
    //         content.clear();
    //     }
    // }
}

impl Default for Map {
    fn default() -> Self {
        Map{
            tiles : vec![TileType::Wall; MAPWIDTH*MAPHEIGHT],
            rooms : Vec::new(),
            width : MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            // revealed_tiles: vec![false; MAPWIDTH*MAPHEIGHT],
            // visible_tiles:  vec![false; MAPWIDTH*MAPHEIGHT],
            blocked: vec![false; MAPWIDTH*MAPHEIGHT],
            tile_content : vec![Vec::new(); MAPWIDTH*MAPHEIGHT],
        } }

}

pub fn draw_map(ecs: &World, ctx: &mut Rltk){

    let map = ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        //TODO: revealed nad visible tiles
        let glyph;
        let fg;
        match tile {
            TileType::Floor => {
                glyph = rltk::to_cp437('·');
                fg = RGB::from_f32(0.1, 0.5, 0.5);
            },
            TileType::Wall => {
                glyph = rltk::to_cp437('#');
                fg = RGB::from_f32(0.5, 1.0, 0.1);
            }
        }
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

}
