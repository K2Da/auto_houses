use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("PotionUseSystem")
        .with_query(<(Read<WantsToDrinkPotion>, Write<CombatStats>)>::query())
        .read_resource::<Entity>()
        .write_resource::<GameLog>()
        .read_component::<Name>()
        .read_component::<Potion>()
        .build(move |commands, world, (player_entity, gamelog), query| {
            let player_entity: &Entity = player_entity;

            for (entity, (drink, mut stats)) in query.iter_entities(world) {
                let potion = world.get_component::<Potion>(drink.potion);
                match potion {
                    None => {}
                    Some(potion) => {
                        stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
                        if entity == *player_entity {
                            gamelog.entries.push(format!(
                                "You drink the {}, healing {} hp.",
                                world.get_component::<Name>(drink.potion).unwrap().name,
                                potion.heal_amount
                            ));
                        }
                        commands.delete(drink.potion);
                    }
                }
                commands.remove_component::<WantsToDrinkPotion>(entity);
            }
        })
}
