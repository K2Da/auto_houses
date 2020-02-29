use super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("DamageSystem")
        .with_query(<(Write<CombatStats>, Read<SufferDamage>)>::query())
        .build(move |commands, world, _resources, query| {
            for (entity, (mut stats, damage)) in query.iter_entities(world) {
                stats.hp -= damage.amount;
                commands.remove_component::<SufferDamage>(entity)
            }
        })
}
