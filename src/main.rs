mod unit;
mod combat;
mod equipment;
mod weapon;
mod armor;
mod item;

use armor::*;
use weapon::*;
use item::*;
use equipment::*;
use item::ItemList;
use unit::{Class, DieType, StatBlock};
use combat::Combat;
use combo_box_derived_lenses::list_lens;
use vizia::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;


use std::fs::File;
use std::io::BufReader;

#[derive(Lens)]
pub struct AppData {
    pub character_name: String,
}

pub enum AppEvent {
    CharacterNameInput(String),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::CharacterNameInput(name) => self.character_name = name.clone()
        });
    }
}

fn main() {

    let rogue = Class::create_rogue();
    let stats = StatBlock::new(8, 14, 14, 16, 10, 14);

    let mut weapon_list = WeaponList::new("src/Items/weapons.json");
    let armor_list = ArmorList::new("src/Items/armors.json");
    let item_list = ItemList::new("src/Items/items.json");

    let weapon = Weapon::new(2, "Greatclub".to_string(), DieType::D4, 1, DamageType::Bludgeoning, vec!{Properties::TwoHanded});
    // weapon_list.add(weapon);
    let result = weapon_list.write("src/Items/weapons.json");

    println!("weapons: {:#?}", weapon_list);


    // println!("weapon_list: {:#?}\r\narmor_list: {:#?}\r\nitem_list: {:#?}\r\n", weapon_list, armor_list, item_list);

    println!("armor: {:#?}", armor_list.get_armor(1));


    let dagger = &weapon_list.weapons[0];
    let chest = armor_list.armors.first();
    let shield = armor_list.armors.last();
    let armor = ArmorSlots::new(chest.cloned(), None, None, None, shield.cloned());
    let equip = Equipment{
        armor,
        melee_weapon: Some(dagger.clone()),
        ranged_weapon: None,
    };
    let character = unit::Unit::create_player_character("Kuro".to_string(), rogue, stats, unit::HitpointsType::Average, equip);
    // println!("{:#?}", character);
    let goblin = unit::Unit::create_goblin();
    // println!("{:#?}", goblin);

    let mut combat = Combat::new(&vec![character], &vec![goblin]);

    combat.start();
    combat.fight(combat.turn_order[0].clone());
    let (player_units, enemy_units) = combat.end();
    println!("Player units: {:#?}", player_units);
    println!("Enemy units: {:#?}", enemy_units);


    
    // println!("{:#?}", goblin);
    // character.melee_attack(&mut goblin);
    // println!("{:#?}", goblin);
    // character.level_up(rogue);
    // character.level_up(rogue);
    // character.level_up(fighter);
    // println!("{:#?}", character);
    
    // let _ = Application::new(|cx| {
    //     cx.emit(EnvironmentEvent::SetLocale(langid!("de")));

    //     cx.add_stylesheet(include_style!("src/styles.css"))
    //         .expect("Failed to load the stylesheet!");

    //     cx.add_translation(
    //         langid!("en-US"),
    //         include_str!("resources/en-US/main.ftl").to_owned(),
    //     );

    //     cx.add_translation(
    //         langid!("de"),
    //         include_str!("resources/de/main.ftl").to_owned(),
    //     );

    //     cx.add_translation(
    //         langid!("ja"),
    //         include_str!("resources/ja/main.ftl").to_owned(),
    //     );

    //     AppData {
    //         character_name: String::new(),
    //     }.build(cx);
    //     HStack::new(cx, |cx|{
    //         HStack::new(cx, |cx| {
    //             Label::new(cx, Localized::new("char_name")).class("char_name_label");
    //             Label::new(cx, Localized::new("main_class")).class("char_main_class_label");
    //         })
    //         .layout_type(LayoutType::Column)
    //         .class("row_char_name");
    //         HStack::new(cx, |cx| {
    //             Textbox::new(cx, AppData::character_name)
    //             .on_submit(|cx, val, _| cx.emit(AppEvent::CharacterNameInput(val)))
    //             .class("char_name_textbox");
    //             // ComboBox::new(cx, vec, selected);
    //         })
    //         .layout_type(LayoutType::Column)
    //         .class("row_char_class");
    //     }).class("outer_stack");

    // })
    // .title("D&D 5e build optimizer AI")
    // .inner_size((400, 500))
    // .run();
}
