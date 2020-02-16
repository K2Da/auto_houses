use legion::prelude::*;
use rltk::{Console, GameState, Rltk, RGB};
mod rect;
use rect::Rect;
mod components;
use components::{Player, Position, Renderable};
mod map;
use map::*;
mod player;
use player::*;

pub struct State {
    ecs: World,
    map: Vec<TileType>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.map, ctx);

        for (pos, render) in <(Read<Position>, Read<Renderable>)>::query().iter(&mut self.ecs) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {}
}

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    let universe = Universe::new();
    let mut gs = State {
        ecs: universe.create_world(),
        map: vec![],
    };

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.map = map;
    let (player_x, player_y) = rooms[0].center();

    gs.ecs.insert(
        (),
        vec![(
            Position {
                x: player_x,
                y: player_y,
            },
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
            Player {},
        )],
    );

    rltk::main_loop(context, gs);
}
