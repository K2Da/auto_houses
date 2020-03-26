use super::super::*;

pub fn build() -> SystemBox {
    SystemBuilder::<()>::new("ItemUseSystem")
        .with_query(<Read<WantsToUseItem>>::query())
        .with_query(<(Read<Equipped>, Read<Name>)>::query())
        .read_resource::<Entity>()
        .read_resource::<Map>()
        .write_resource::<GameLog>()
        .read_component::<AreaOfEffect>()
        .read_component::<Confusion>()
        .read_component::<Equippable>()
        .read_component::<InflictsDamage>()
        .read_component::<Name>()
        .read_component::<ProvidesHealing>()
        .write_component::<CombatStats>()
        .write_component::<Equipped>()
        .build(
            move |commands, world, (player_entity, map, gamelog), (item_query, equipped_query)| {
                let player_entity: &Entity = player_entity;

                for (entity, use_item) in item_query.iter_entities(world) {
                    let map: &Map = map;
                    let item = use_item.item.entity();
                    let item_name = get_name(world, item).to_owned();
                    let mut used_item = true;

                    let mut targets: Vec<Entity> = Vec::new();
                    match use_item.target {
                        None => targets.push(*player_entity),
                        Some(target) => {
                            let area_effect = world.get_component::<AreaOfEffect>(item);
                            match area_effect {
                                None => {
                                    let idx = map.xy_idx(target.x, target.y);
                                    for mob in map.tile_content[idx].iter() {
                                        targets.push(*mob);
                                    }
                                }
                                Some(area_effect) => {
                                    let mut blast_tiles =
                                        rltk::field_of_view(target, area_effect.radius, map);
                                    retain_tiles(map, &mut blast_tiles);
                                    for tile_idx in blast_tiles.iter() {
                                        let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                        for mob in map.tile_content[idx].iter() {
                                            targets.push(*mob);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let item_heals = world
                        .get_component::<ProvidesHealing>(item)
                        .map(|i| (*i).clone());
                    match item_heals {
                        None => (),
                        Some(healer) => {
                            for target in targets.iter() {
                                let stats = world.get_component_mut::<CombatStats>(*target);
                                if let Some(mut stats) = stats {
                                    stats.hp =
                                        i32::min(stats.max_hp, stats.hp + healer.heal_amount);
                                    gamelog.push(format!(
                                        "You use the {}, healing {} hp.",
                                        item_name, healer.heal_amount
                                    ));
                                }
                            }
                        }
                    }

                    match world.get_component::<InflictsDamage>(item) {
                        None => {}
                        Some(damage) => {
                            for mob in targets.iter() {
                                SufferDamage::new_damage(commands, *mob, damage.damage);

                                if entity == *player_entity {
                                    gamelog.push(format!(
                                        "You use {} on {}, inflicting {} hp.",
                                        item_name,
                                        get_name(world, *mob),
                                        damage.damage
                                    ));
                                }
                            }
                            used_item = true;
                        }
                    }

                    let mut add_confusion = Vec::new();
                    {
                        if let Some(confusion) = world.get_component::<Confusion>(item) {
                            used_item = false;
                            for mob in targets.iter() {
                                add_confusion.push((*mob, confusion.turns));
                                if entity == *player_entity {
                                    gamelog.push(format!(
                                        "You use {} on {}, confusing them.",
                                        item_name,
                                        get_name(world, *mob)
                                    ));
                                }
                            }
                        }
                    }

                    for mob in add_confusion.iter() {
                        commands.add_component(mob.0, Confusion::new(mob.1));
                    }

                    let equippable = world
                        .get_component::<Equippable>(item)
                        .map(|i| (*i).clone());
                    match equippable {
                        None => (),
                        Some(can_equip) => {
                            let target_slot = can_equip.slot;
                            let target = targets[0];

                            let mut to_unequip: Vec<Entity> = Vec::new();
                            for (item_entity, (already_equipped, name)) in
                                equipped_query.iter_entities(world)
                            {
                                if already_equipped.owner.entity() == target
                                    && already_equipped.slot == target_slot
                                {
                                    to_unequip.push(item_entity);

                                    if target == *player_entity {
                                        gamelog.push(format!("You unequip {}.", name.name));
                                    }
                                }
                            }

                            for unequip_item in to_unequip.iter() {
                                commands.remove_component::<Equipped>(*unequip_item);
                                commands.add_component(*unequip_item, InBackpack::new(target))
                            }

                            commands.add_component(item, Equipped::new(target, target_slot));
                            commands.remove_component::<InBackpack>(item);

                            if target == *player_entity {
                                gamelog.push(format!("You equip {}.", item_name));
                            }
                        }
                    }

                    if used_item {
                        let consumable = world.get_tag::<Consumable>(item);
                        match consumable {
                            None => {}
                            Some(_) => commands.delete(item),
                        }
                    }

                    commands.remove_component::<WantsToUseItem>(entity);
                }
            },
        )
}
