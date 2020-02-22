use super::*;
use rltk::console;

pub fn build() -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("DeleteTheDeadSystem")
        .with_query(<(Read<CombatStats>, TryRead<Player>)>::query())
        .build(move |commands, world, _resources, query| {
            for (entity, (stats, player)) in query.iter_entities(world) {
                if stats.hp < 1 {
                    match player {
                        Some(_) => console::log("You are dead"),
                        None => commands.delete(entity),
                    }
                }
            }
        })
}
