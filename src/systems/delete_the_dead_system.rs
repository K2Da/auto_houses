use super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("DeleteTheDeadSystem")
        .with_query(<(Read<CombatStats>, TryRead<Player>)>::query())
        .read_component::<Name>()
        .write_resource::<GameLog>()
        .write_resource::<RunState>()
        .build(move |commands, world, (log, runstate), query| {
            for (entity, (stats, player)) in query.iter_entities(world) {
                if stats.hp < 1 {
                    match player {
                        None => {
                            log.push(format!("{} is dead", get_name(world, entity)));
                            commands.delete(entity)
                        }
                        Some(_) => {
                            let runstate: &mut RunState = runstate;
                            *runstate = RunState::GameOver;
                        }
                    }
                }
            }
        })
}
