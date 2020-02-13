use legion::prelude::*;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use std::cmp::{max, min};

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

#[derive(Clone, Copy, Debug, PartialEq)]
struct LeftMover {}

#[derive(Debug)]
struct Player {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        for (pos, render) in <(Read<Position>, Read<Renderable>)>::query().iter(&mut self.ecs) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let left_walker = LeftWalker {};
        left_walker.run_now(&mut self.ecs);
    }
}

struct LeftWalker {}

impl LeftWalker {
    fn run_now(&self, ecs: &mut World) {
        for (mut pos, _) in <(Write<Position>, Read<LeftMover>)>::query().iter(ecs) {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
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
            Player {},
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
                LeftMover {},
            )
        }),
    );

    rltk::main_loop(context, gs);
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    for (mut pos, _) in <(Write<Position>, Read<Player>)>::query().iter(ecs) {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
