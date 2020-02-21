use super::*;
use rltk::{console, Point};

pub fn build() -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MonsterAISystem")
        .with_query(
            <(Read<Viewshed>, Read<Position>, Read<Name>)>::query().filter(component::<Monster>()),
        )
        .read_resource::<Point>()
        .build(move |_commands, world, player_pos, query| {
            for (viewshed, _pos, name) in query.iter(world) {
                if viewshed.visible_tiles.contains(player_pos) {
                    console::log(&format!("{} shouts insults", name.name));
                }
            }
        })
}
