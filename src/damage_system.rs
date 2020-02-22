use super::*;

pub fn build() -> Box<dyn legion::schedule::Schedulable> {
    SystemBuilder::<()>::new("DamageSystem")
        .with_query(<(Write<CombatStats>, Read<SufferDamage>)>::query())
        .build(move |commands, world, _resources, query| {
            for (entity, (mut stats, damage)) in query.iter_entities(world) {
                stats.hp -= damage.amount;
                println!("hp: {}", stats.hp);
                commands.remove_component::<SufferDamage>(entity)
            }
        })
}
