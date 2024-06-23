mod unit;
mod combat;
mod equipment;
// use character::Unit::melee_attack;
use std::{collections::HashMap, vec};

use equipment::{Armor, ArmorSlot, ArmorType, Equipment, Weapon, WeaponSize};
use unit::{Class, DieType, StatBlock};
use combat::Combat;
use combo_box_derived_lenses::list_lens;
use vizia::prelude::*;
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

    let dagger = Weapon{
        name: "Dagger".to_string(),
        damage_die: DieType::D6,
        damage_die_count: 1,
        range: 5,
        size: WeaponSize::OneHanded,
    };
    let chest = Armor{
        name: "Studded Leather Armor".to_string(),
        armor_class: 12,
        armor_type: Some(ArmorType::Light),
    };
    let shield = Armor{
        name: "Shield".to_string(),
        armor_class: 2,
        armor_type: Some(ArmorType::Shield)
    };
    let mut armor: HashMap<ArmorSlot, Armor> = HashMap::new();
    armor.insert(ArmorSlot::Chest, chest);
    armor.insert(ArmorSlot::Shield, shield);
    let equip = Equipment{
        armor,
        melee_weapon: Some(dagger),
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
