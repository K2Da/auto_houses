use legion::prelude::*;
use legion::*;
use rltk::{Console, GameState, Rltk, RGB};

struct State {
    ecs: World,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let query = <(Read<Position>, Read<Renderable>)>::query();
        for (pos, render) in query.iter(&mut self.ecs) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();

    let universe = Universe::new();
    let mut gs = State {
        ecs: universe.create_world(),
    };
    gs.ecs.insert(
        (),
        vec![(
            Position { x: 40, y: 25 },
            Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            },
        )],
    );

    gs.ecs.insert(
        (),
        (0..10).map(|i| {
            (
                Position { x: i * 7, y: 20 },
                Renderable {
                    glyph: rltk::to_cp437('☺'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
            )
        }),
    );

    rltk::main_loop(context, gs);
}
