use super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("MeleeCombatSystem")
        .with_query(<(Read<WantsToMelee>, Read<Name>, Read<CombatStats>)>::query())
        .read_component::<CombatStats>()
        .read_component::<Name>()
        .write_resource::<GameLog>()
        .build(move |commands, world, log, query| {
            for (entity, (wants_melee, name, stats)) in query.iter_entities(world) {
                if stats.hp > 0 {
                    let target = wants_melee.target.clone();
                    let target_stats = world.get_component::<CombatStats>(target.entity()).unwrap();

                    if target_stats.hp > 0 {
                        let target_name = get_name(world, target.entity());
                        let damage = i32::max(0, stats.power - target_stats.defense);

                        if damage == 0 {
                            log.entries
                                .push(format!("{} is unable to hurt {}", &name.name, target_name));
                        } else {
                            log.entries.push(format!(
                                "{} hits {}, for {} hp.",
                                &name.name, target_name, damage
                            ));
                            SufferDamage::new_damage(commands, target.entity(), damage);
                        }
                    }
                }
                commands.remove_component::<WantsToMelee>(entity);
            }
        })
}
