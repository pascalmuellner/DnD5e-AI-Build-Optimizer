use vizia::prelude::*;
#[derive(Lens)]
pub struct AppData{
    pub character_name: String,
}

pub enum AppEvent{
    Textinput,
    Optimize,
}

impl Model for AppData {}

fn main() {
    let _ = Application::new(|cx|{
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
        AppData{ character_name: String::new() }.build(cx);
        HStack::new(cx, |cx|{
            Label::new(cx, Localized::new("char_name"))
            .class("char_name_label");
            Textbox::new(cx, AppData::character_name).class("char_name_textbox");
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(20.0))
        .class("char_name_row");
    })
    .title("D&D 5e build optimizer AI")
    .inner_size((400,500))
    .run();
}
