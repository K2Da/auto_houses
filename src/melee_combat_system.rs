use super::*;
use rltk::console;

pub fn build() -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("MeleeCombatSystem")
        .with_query(<(Read<WantsToMelee>, Read<Name>, Read<CombatStats>)>::query())
        .read_component::<CombatStats>()
        .read_component::<SufferDamage>()
        .read_component::<Name>()
        .build(move |commands, world, _resources, query| {
            for (entity, (wants_melee, name, stats)) in query.iter_entities(world) {
                if stats.hp > 0 {
                    let target = wants_melee.target;
                    let target_stats = world.get_component::<CombatStats>(target).unwrap();

                    if target_stats.hp > 0 {
                        let target_name = world.get_component::<Name>(target).unwrap();
                        let damage = i32::max(0, stats.power - target_stats.defense);

                        if damage == 0 {
                            console::log(&format!(
                                "{} is unable to hurt {}",
                                &name.name, &target_name.name
                            ));
                        } else {
                            console::log(&format!(
                                "{} hits {}, for {} hp.",
                                &name.name, &target_name.name, damage
                            ));
                            commands.add_component(target, SufferDamage { amount: damage });
                        }
                    }
                }
                commands.remove_component::<WantsToMelee>(entity);
            }
        })
}
