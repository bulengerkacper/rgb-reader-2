extern crate image;
use dioxus::prelude::*;

pub mod analysis;

use analysis::RgbData;

pub fn Calc(cx: Scope) -> Element {
    let filenames: &UseRef<Vec<String>> = use_ref(cx, Vec::new);
    let r = use_state(cx, || 0);
    let g = use_state(cx, || 0);
    let b = use_state(cx, || 0);
    let grey = use_state(cx, || 0.0);

    cx.render(rsx! {
        input {
            r#type:"file",
            accept: ".jpg, .jpeg, .png",
            multiple: false,
            onchange: |evt| {
                if let Some(file_engine) = &evt.files {
                    let rgb_data = RgbData::default();
                    let files = file_engine.files();
                    for file_name in &files {
                        filenames.write().push(file_name.to_string());
                        let rgbs = rgb_data.count_avgs(&file_name);
                            grey.set(rgb_data.to_grey(rgbs.0,rgbs.1,rgbs.2));

                            r.set(rgbs.0);
                            g.set(rgbs.1);
                            b.set(rgbs.2);
                    }
                }
            }
        }
         br {}
         "r avg: " "{ r }" br {}
         "g avg: " "{ g }" br {}
         "b avg: " "{ b }" br {}
         "grey avg: ""{ grey }" br {}

    })
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Calc {},
        Calc {},
    })
}

pub fn main() {

    dioxus_desktop::launch(App);

}
