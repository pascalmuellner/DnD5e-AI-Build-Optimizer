mod armor;
mod class;
mod combat;
mod equipment;
mod item;
mod species;
mod spells;
mod subclass;
mod unit;
mod weapon;

use armor::*;
use class::*;
use item::ItemList;
use item::*;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use species::*;
use subclass::*;
use unit::*;
use vizia::prelude::*;
use weapon::*;

use std::fs::{self, File};
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

#[derive(Lens)]
struct AppData {
    character_name: String,
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
    show_stats_popup: bool,
    species_list: Vec<Species>,
    selected_species: usize,
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
    CreateCharacter,
    SaveCharacter,
    LoadCharacter,
    ShowStatsPopup,
    StatsPopupClosed,
    SetSpecies(usize),
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
            AppEvent::SetStatsStr(num) => {
                self.char_stats.strength = *num;
            }
            AppEvent::SetStatsWis(num) => {
                self.char_stats.wisdom = *num;
            }
            AppEvent::SetStatsCha(num) => {
                self.char_stats.charisma = *num;
            }
            AppEvent::SetStatsCon(num) => {
                self.char_stats.constitution = *num;
            }
            AppEvent::SetStatsDex(num) => {
                self.char_stats.dexterity = *num;
            }
            AppEvent::CreateCharacter => {
                let stats = self
                    .char_stats
                    .add(self.species_list[self.selected_species].stats_increase);
                let species = self.species_list[self.selected_species].clone();
                self.character = Some(Unit::create_player_character(
                    self.character_name.clone(),
                    self.main_class.clone().unwrap(),
                    stats,
                    HitpointsType::Random,
                    None,
                    Some(species),
                    None,
                    None,
                ));
                println!("Created character: {:#?}", self.character);
            }
            AppEvent::SetSpecies(index) => {
                self.selected_species = *index;
            }
            AppEvent::SaveCharacter => {
                let mut file_name = "PlayerUnits/".to_owned();
                if !Path::new(&file_name).exists() {
                    let folder_creation_result = fs::create_dir(&file_name);
                    println!("Folder creation: {:#?}", folder_creation_result);
                }
                file_name.push_str(&self.character_name);
                file_name.push_str(".json");
                if self.character.as_ref() != None {
                    let result = self.character.as_ref().unwrap().write(file_name.as_str());
                    println!("{:#?}", result);
                }
            }
            AppEvent::LoadCharacter => {
                let path = std::env::current_dir().unwrap();
                let files = FileDialog::new()
                    .add_filter("json", &["json"])
                    .set_directory(&path)
                    .pick_file();

                if files != None {
                    self.character = Some(Unit::new(files.unwrap()));
                    self.character_name = self.character.clone().unwrap().character_name;
                    self.char_stats = self.character.clone().unwrap().stats;
                    self.main_class = Some(self.character.clone().unwrap().starting_class.unwrap());
                    self.selected_main_class = self
                        .main_class_vec
                        .iter()
                        .position(|&r| r == self.main_class.clone().unwrap().class_name)
                        .unwrap();
                    self.selected_main_class_level = (self
                        .character
                        .clone()
                        .unwrap()
                        .starting_class
                        .unwrap()
                        .class_level
                        - 1) as usize;
                    println!("Loaded character: {:#?}", self.character);
                }
            }
            AppEvent::ShowStatsPopup => self.show_stats_popup = true,
            AppEvent::StatsPopupClosed => self.show_stats_popup = false,
        });
    }
}

fn main() {
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
            species_list: SpeciesList::new("src/Data/species.json").species,
            selected_species: 0,
            show_stats_popup: false,
        }
        .build(cx);

        Binding::new(cx, AppData::show_stats_popup, |cx, show_subwindow| {
            if show_subwindow.get(cx) {
                Window::popup(cx, false, |cx| {
                    // have a horizontal stack of 6 vertical stacks, each vertical stack is one stat
                    HStack::new(cx, |cx| {
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("int")).class("char_stats_int_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::intelligence))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsInt(val));
                                })
                                .class("stats_int");
                        })
                        .class("row");
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("str")).class("char_stats_int_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::strength))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsStr(val));
                                })
                                .class("stats_str");
                        })
                        .class("row");
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("wis")).class("char_stats_wis_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::wisdom))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsWis(val));
                                })
                                .class("stats_wis");
                        })
                        .class("row");
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("cha")).class("char_stats_cha_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::charisma))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsCha(val));
                                })
                                .class("stats_cha");
                        })
                        .class("row");
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("con")).class("char_stats_con_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::constitution))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsCon(val));
                                })
                                .class("stats_con");
                        })
                        .class("row");
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("dex")).class("char_stats_dex_label");
                            Textbox::new(cx, AppData::char_stats.then(StatBlock::dexterity))
                                .validate(|val| *val >= 8 && *val <= 18)
                                .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                                .on_submit(|cx, val, _| {
                                    cx.emit(AppEvent::SetStatsDex(val));
                                })
                                .class("stats_dex");
                        })
                        .class("row");
                    });
                })
                .on_close(|cx| {
                    cx.emit(AppEvent::StatsPopupClosed);
                })
                .title("Character Stats")
                .inner_size((500, 10));
            };
        });

        VStack::new(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("char_name")).class("char_name_label");
                Textbox::new(cx, AppData::character_name)
                    .on_focus_out(|cx| cx.emit(TextEvent::Submit(false)))
                    .on_submit(|cx, val, _| cx.emit(AppEvent::CharacterNameInput(val)))
                    .class("char_name_textbox");
            })
            .class("row");
            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("select_species")).class("char_select_species_label");
                ComboBox::new(cx, AppData::species_list, AppData::selected_species)
                    .on_select(|cx, index| cx.emit(AppEvent::SetSpecies(index)));
            });

            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("main_class")).class("char_main_class_label");
                ComboBox::new(cx, AppData::main_class_vec, AppData::selected_main_class)
                    .on_select(|cx, index| cx.emit(AppEvent::SelectMainClass(index)));
                // .width(Pixels(100.0));
            })
            .class("row");

            HStack::new(cx, |cx| {
                Label::new(cx, Localized::new("main_class_level"))
                    .class("char_main_class_level_label");
                PickList::new(
                    cx,
                    AppData::main_class_level,
                    AppData::selected_main_class_level,
                    true,
                )
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
                        Label::new(cx, Localized::new("multiclass"))
                            .class("char_multi_class_label");
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
                        PickList::new(
                            cx,
                            AppData::multi_class_level,
                            AppData::selected_multi_class_level,
                            true,
                        )
                        .on_select(|cx, index| cx.emit(AppEvent::SetMultiClassLevel(index)))
                        .width(Pixels(100.0))
                        .class("multi_class_level_dropdown");
                    })
                    .class("row");
                }
            });
            Button::new(cx, |cx| Label::new(cx, Localized::new("create_character")))
                .on_press(|cx| cx.emit(AppEvent::CreateCharacter));
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, Localized::new("save_character")))
                    .on_press(|cx| cx.emit(AppEvent::SaveCharacter));
                Button::new(cx, |cx| Label::new(cx, Localized::new("load_character")))
                    .on_press(|cx| cx.emit(AppEvent::LoadCharacter));
                Button::new(cx, |cx| Label::new(cx, Localized::new("open_stats_popup")))
                    .on_press(|cx| cx.emit(AppEvent::ShowStatsPopup));
            });
        })
        .class("outer_stack");
    })
    .title("D&D 5e build optimizer AI")
    .inner_size((400, 500))
    .run();
}
