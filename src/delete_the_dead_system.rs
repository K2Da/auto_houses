use super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("DeleteTheDeadSystem")
        .with_query(<(Read<CombatStats>, TryRead<Player>)>::query())
        .read_component::<Name>()
        .write_resource::<GameLog>()
        .build(move |commands, world, log, query| {
            for (entity, (stats, player)) in query.iter_entities(world) {
                if stats.hp < 1 {
                    match player {
                        None => {
                            let victim_name = world.get_component::<Name>(entity).unwrap();
                            log.entries.push(format!("{} is dead", victim_name.name));
                            commands.delete(entity)
                        }
                        Some(_) => log.entries.push("You are dead".to_string()),
                    }
                }
            }
        })
}
