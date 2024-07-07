mod armor;
mod combat;
mod equipment;
mod item;
mod unit;
mod weapon;

use app_data_derived_lenses::char_stats;
use armor::*;
use combat::Combat;
use combo_box_derived_lenses::list_lens;
use equipment::*;
use item::ItemList;
use item::*;
use unit::{Class, ClassName, DieType, StatBlock, Unit};
use vizia::prelude::*;
use weapon::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::fs::File;
use std::io::BufReader;

#[derive(Lens)]
struct AppData {
    character_name: String,
    items: Items,
    character: Option<Unit>,
    char_stats: StatBlock,
    main_class: Option<Class>,
    main_class_level: Vec<i32>,
    selected_main_class_level: usize,
    multi_class: Option<Class>,
    multi_class_level: Vec<i32>,
    selected_multi_class_level: usize,
    main_class_vec: Vec<ClassName>,
    multiclass_class_vec: Vec<ClassName>,
    selected_main_class: usize,
    selected_multi_class: usize,
    multiclass_enabled: bool,
}

#[derive(Lens)]
struct Items {
    pub weapon_list: WeaponList,
    pub armor_list: ArmorList,
    pub item_list: ItemList,
}

pub enum AppEvent {
    CharacterNameInput(String),
    SelectMainClass(usize),
    SelectMultiClass(usize),
    Toggle,
    SetMainClassLevel(usize),
    SetMultiClassLevel(usize),
    SetStatsInt(i32),
    SetStatsStr(i32),
    SetStatsWis(i32),
    SetStatsCha(i32),
    SetStatsCon(i32),
    SetStatsDex(i32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::CharacterNameInput(name) => self.character_name = name.clone(),
            AppEvent::SelectMainClass(index) => {
                match self.main_class_vec[*index] {
                    ClassName::Artificer => self.main_class = Some(Class::create_artificer()),
                    ClassName::Barbarian => self.main_class = Some(Class::create_barbarian()),
                    ClassName::Bard => self.main_class = Some(Class::create_bard()),
                    ClassName::Cleric => self.main_class = Some(Class::create_cleric()),
                    ClassName::Druid => self.main_class = Some(Class::create_druid()),
                    ClassName::Fighter => self.main_class = Some(Class::create_fighter()),
                    ClassName::Monk => self.main_class = Some(Class::create_monk()),
                    ClassName::Paladin => self.main_class = Some(Class::create_paladin()),
                    ClassName::Ranger => self.main_class = Some(Class::create_ranger()),
                    ClassName::Rogue => self.main_class = Some(Class::create_rogue()),
                    ClassName::Sorcerer => self.main_class = Some(Class::create_sorcerer()),
                    ClassName::Warlock => self.main_class = Some(Class::create_warlock()),
                    ClassName::Wizard => self.main_class = Some(Class::create_wizard()),
                };
                self.selected_main_class = *index;
                let mut multiclass_vec = ClassName::get_all_class_names();
                multiclass_vec.remove(*index);
                self.multiclass_class_vec = multiclass_vec;
            }
            AppEvent::SelectMultiClass(index) => {
                match self.main_class_vec[*index] {
                    ClassName::Artificer => self.multi_class = Some(Class::create_artificer()),
                    ClassName::Barbarian => self.multi_class = Some(Class::create_barbarian()),
                    ClassName::Bard => self.multi_class = Some(Class::create_bard()),
                    ClassName::Cleric => self.multi_class = Some(Class::create_cleric()),
                    ClassName::Druid => self.multi_class = Some(Class::create_druid()),
                    ClassName::Fighter => self.multi_class = Some(Class::create_fighter()),
                    ClassName::Monk => self.multi_class = Some(Class::create_monk()),
                    ClassName::Paladin => self.multi_class = Some(Class::create_paladin()),
                    ClassName::Ranger => self.multi_class = Some(Class::create_ranger()),
                    ClassName::Rogue => self.multi_class = Some(Class::create_rogue()),
                    ClassName::Sorcerer => self.multi_class = Some(Class::create_sorcerer()),
                    ClassName::Warlock => self.multi_class = Some(Class::create_warlock()),
                    ClassName::Wizard => self.multi_class = Some(Class::create_wizard()),
                };
                self.selected_multi_class = *index;
            }
            AppEvent::Toggle => {
                self.multiclass_enabled ^= true;
                println!("multiclass: {:#?}", self.multiclass_enabled);
            }
            AppEvent::SetMainClassLevel(index) => {
                self.selected_main_class_level = *index;
            }
            AppEvent::SetMultiClassLevel(index) => {
                self.selected_multi_class_level = *index;
            }
            AppEvent::SetStatsInt(num) => {
                self.char_stats.intelligence = *num;
            }
            AppEvent::SetStatsStr(_) => todo!(),
            AppEvent::SetStatsWis(_) => todo!(),
            AppEvent::SetStatsCha(_) => todo!(),
            AppEvent::SetStatsCon(_) => todo!(),
            AppEvent::SetStatsDex(_) => todo!(),
        });
    }
}

fn main() {
    // let rogue = Class::create_rogue();
    // let stats = StatBlock::new(8, 14, 14, 16, 10, 14);

    // let mut weapon_list = WeaponList::new("src/Items/weapons.json");
    // let armor_list = ArmorList::new("src/Items/armors.json");
    // let item_list = ItemList::new("src/Items/items.json");

    // let weapon = Weapon::new(2, "Greatclub".to_string(), DieType::D4, 1, DamageType::Bludgeoning, vec!{Properties::TwoHanded});
    // weapon_list.add(weapon);
    // let result = weapon_list.write("src/Items/weapons.json");

    // println!("weapons: {:#?}", weapon_list);

    // println!("weapon_list: {:#?}\r\narmor_list: {:#?}\r\nitem_list: {:#?}\r\n", weapon_list, armor_list, item_list);

    // println!("armor: {:#?}", armor_list.get_armor(1));

    // let dagger = &weapon_list.weapons[0];
    // let chest = armor_list.armors.first();
    // let shield = armor_list.armors.last();
    // let armor = ArmorSlots::new(chest.cloned(), None, None, None, shield.cloned());
    // let equip = Equipment{
    //     armor,
    //     melee_weapon: Some(dagger.clone()),
    //     ranged_weapon: None,
    // };
    // let character = unit::Unit::create_player_character("Kuro".to_string(), rogue, stats, unit::HitpointsType::Average, equip);
    // println!("{:#?}", character);
    // let goblin = unit::Unit::create_goblin();
    // println!("{:#?}", goblin);

    // let mut combat = Combat::new(&vec![character], &vec![goblin]);

    // combat.start();
    // combat.fight(combat.turn_order[0].clone());
    // let (player_units, enemy_units) = combat.end();
    // println!("Player units: {:#?}", player_units);
    // println!("Enemy units: {:#?}", enemy_units);

    // println!("{:#?}", goblin);
    // character.melee_attack(&mut goblin);
    // println!("{:#?}", goblin);
    // character.level_up(rogue);
    // character.level_up(rogue);
    // character.level_up(fighter);
    // println!("{:#?}", character);

    let _ = Application::new(|cx| {
        // cx.emit(EnvironmentEvent::SetLocale(langid!("en-US")));

        cx.add_stylesheet(include_style!("src/styles.css"))
            .expect("Failed to load the stylesheet!");

        cx.add_translation(
            langid!("en-US"),
            include_str!("resources/en-US/main.ftl").to_owned(),
        );

        cx.add_translation(
            langid!("de"),
            include_str!("resources/de/main.ftl").to_owned(),
        );

        cx.add_translation(
            langid!("ja"),
            include_str!("resources/ja/main.ftl").to_owned(),
        );

        AppData {
            character_name: String::new(),
            items: Items {
                weapon_list: WeaponList::new("src/Items/weapons.json"),
                armor_list: ArmorList::new("src/Items/armors.json"),
                item_list: ItemList::new("src/Items/items.json"),
            },
            character: None,
            char_stats: StatBlock {
                intelligence: 10,
                constitution: 10,
                strength: 10,
                dexterity: 10,
                wisdom: 10,
                charisma: 10,
            },
            main_class: None,
            main_class_vec: ClassName::get_all_class_names(),
            selected_main_class: 0,
            main_class_level: (1..21).collect(),
            multiclass_enabled: false,
            multi_class: None,
            multi_class_level: (1..21).collect(),
            selected_multi_class: 0,
            multiclass_class_vec: ClassName::get_all_class_names(),
            selected_main_class_level: 0,
            selected_multi_class_level: 0,
        }
        .build(cx);
        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("char_name")).class("char_name_label");
                Textbox::new(cx, AppData::character_name)
                    .on_submit(|cx, val, _| cx.emit(AppEvent::CharacterNameInput(val)))
                    .class("char_name_textbox");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("main_class")).class("char_main_class_label");
                ComboBox::new(cx, AppData::main_class_vec, AppData::selected_main_class)
                    .on_select(|cx, index| cx.emit(AppEvent::SelectMainClass(index)));
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("int")).class("char_stats_int_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::intelligence))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsInt(val));
                    })
                    .class("stats_int");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("str")).class("char_stats_int_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::strength))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsStr(val));
                    })
                    .class("stats_str");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("wis")).class("char_stats_wis_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::wisdom))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsWis(val));
                    })
                    .class("stats_wis");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("cha")).class("char_stats_cha_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::charisma))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsCha(val));
                    })
                    .class("stats_cha");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("con")).class("char_stats_con_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::constitution))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsCon(val));
                    })
                    .class("stats_con");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("dex")).class("char_stats_dex_label");
                Textbox::new(cx, AppData::char_stats.then(StatBlock::dexterity))
                    .validate(|val| *val >= 8 && *val <= 18)
                    .on_submit(|cx, val, _| {
                        cx.emit(AppEvent::SetStatsDex(val));
                    })
                    .class("stats_dex");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("main_class_level"))
                    .class("char_main_class_level_label");
                PickList::new(cx, AppData::main_class_level, AppData::selected_main_class_level, true)
                .on_select(|cx, index| cx.emit(AppEvent::SetMainClassLevel(index)))
                .width(Pixels(100.0))
                .class("main_class_level_dropdown");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("multiclass"))
                    .class("char_multiclass_label")
                    .describing("multiclass");
                Checkbox::new(cx, AppData::multiclass_enabled)
                    .on_toggle(|cx| cx.emit(AppEvent::Toggle))
                    .id("multiclass");
            })
            .class("row");
            Binding::new(cx, AppData::multiclass_enabled, |cx, show| {
                if show.get(cx) {
                    HStack::new(cx, |cx| {
                        Label::new(cx, Localized::new("multiclass")).class("char_multi_class_label");
                        ComboBox::new(
                            cx,
                            AppData::multiclass_class_vec,
                            AppData::selected_multi_class,
                        )
                        .on_select(|cx, index| cx.emit(AppEvent::SelectMultiClass(index)));
                    })
                    .class("row");
                    HStack::new(cx, |cx| {
                        Label::new(cx, Localized::new("multi_class_level"))
                            .class("char_multi_class_level_label");
                        PickList::new(cx, AppData::multi_class_level, AppData::selected_multi_class_level, true)
                        .on_select(|cx, index| cx.emit(AppEvent::SetMultiClassLevel(index)))
                        .width(Pixels(100.0))
                        .class("multi_class_level_dropdown");
                    })
                    .class("row");
                }
            });
        })
        .class("outer_stack");
    })
    .title("D&D 5e build optimizer AI")
    .inner_size((400, 500))
    .run();
}
