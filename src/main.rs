mod character;
use std::{alloc::Layout, vec};

use character::Class;
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
    let artificer = Class::create_artificer();
    let fighter = Class::create_fighter();
    let mut character = character::Character::create_character("Kuro".to_string(), rogue, character::HitpointsType::Average);

    println!("{:#?}", character);
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
