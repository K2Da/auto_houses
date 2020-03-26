use super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("MeleeCombatSystem")
        .with_query(<(Read<WantsToMelee>, Read<Name>, Read<CombatStats>)>::query())
        .with_query(<(Read<MeleePowerBonus>, Read<Equipped>)>::query())
        .with_query(<(Read<DefenseBonus>, Read<Equipped>)>::query())
        .read_component::<CombatStats>()
        .read_component::<Name>()
        .write_resource::<GameLog>()
        .build(
            move |commands, world, log, (melee_query, melee_bonus_query, defense_bonus_query)| {
                for (entity, (wants_melee, name, stats)) in melee_query.iter_entities(world) {
                    if stats.hp > 0 {
                        let mut offensive_bonus = 0;
                        for (power_bonus, equipped_by) in melee_bonus_query.iter(world) {
                            if equipped_by.owner.entity() == entity {
                                offensive_bonus += power_bonus.power;
                            }
                        }

                        let target = wants_melee.target.clone();
                        let target_stats: CombatStats =
                            (*world.get_component::<CombatStats>(target.entity()).unwrap()).clone();

                        if target_stats.hp > 0 {
                            let mut defensive_bonus = 0;
                            for (defense_bonus, equipped_by) in defense_bonus_query.iter(world) {
                                if equipped_by.owner.entity() == entity {
                                    defensive_bonus += defense_bonus.defense;
                                }
                            }

                            let target_name = get_name(world, target.entity());
                            let damage = i32::max(
                                0,
                                (stats.power + offensive_bonus)
                                    - (target_stats.defense + defensive_bonus),
                            );

                            if damage == 0 {
                                log.push(format!(
                                    "{} is unable to hurt {}",
                                    &name.name, target_name
                                ));
                            } else {
                                log.push(format!(
                                    "{} hits {}, for {} hp.",
                                    &name.name, target_name, damage
                                ));
                                SufferDamage::new_damage(commands, target.entity(), damage);
                            }
                        }
                    }
                    commands.remove_component::<WantsToMelee>(entity);
                }
            },
        )
}
