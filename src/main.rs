use legion::prelude::*;
use rltk::{Console, GameState, Rltk, RGB};
mod rect;
use rect::Rect;
mod components;
use components::{Player, Position, Renderable, Viewshed};
mod map;
use map::*;
mod player;
use player::*;
mod visibility_system;

pub struct State {
    world: World,
    schedule: legion::schedule::Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);

        self.run_systems();

        draw_map(&mut self.world, ctx);

        for (pos, render) in <(Read<Position>, Read<Renderable>)>::query().iter(&mut self.world) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.schedule.execute(&mut self.world);
    }
}

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    let universe = Universe::new();

    let mut gs = State {
        world: universe.create_world(),
        schedule: Schedule::builder()
            .add_system(visibility_system::visibility_system())
            .build(),
    };
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    gs.world.resources.insert(map);

    gs.world.insert(
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
            Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            },
        )],
    );

    rltk::main_loop(context, gs);
}
