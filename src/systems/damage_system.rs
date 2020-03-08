use super::*;

impl SufferDamage {
    // 取り出して配列に追加ができないので、別々のentityにする
    pub fn new_damage(commands: &mut CommandBuffer, victim: Entity, amount: i32) {
        commands.insert(
            (),
            vec![(SufferDamage {
                victim: EntityHolder::new(victim),
                amount,
            },)],
        );
    }
}

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("DamageSystem")
        .with_query(<Read<SufferDamage>>::query())
        .write_component::<CombatStats>()
        .build(move |commands, world, _resources, query| {
            for (entity, damage) in query.iter_entities(world) {
                let stats: &mut CombatStats =
                    &mut world.get_component_mut(damage.victim.entity()).unwrap();
                stats.hp -= damage.amount;
                commands.delete(entity);
            }
        })
}
