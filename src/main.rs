extern crate image;
use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

pub mod analysis;

use analysis::RgbData;

pub fn Calc(cx: Scope) -> Element {
    let filenames: &UseRef<Vec<String>> = use_ref(cx, Vec::new);
    let r = use_state(cx, || 0);
    let g = use_state(cx, || 0);
    let b = use_state(cx, || 0);
    let grey = use_state(cx, || 0.0);

    cx.render(rsx! {
    div {
        width: "300px",
        height: "300px",
        border_width: "thick",
        overflow:"scroll",
        border_style: "solid solid solid solid",
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
        for file in filenames.read().iter() {
              "{file}"
        }
         br {}
         "r avg: " "{ r }" br {}
         "g avg: " "{ g }" br {}
         "b avg: " "{ b }" br {}
         "Scalled grey avg: ""{ grey }" br {}
    }
    })
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            position: "relative",
            table {
                tr {
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                }
                tr {
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                    td {
                        Calc {}
                    }
                }

            }

        }
    })
}

pub fn main() {
    dioxus_desktop::launch_with_props(
        App,(),
        Config::new().with_window(
            WindowBuilder::default()
                .with_title("Spectrophotometer")
                .with_inner_size(dioxus_desktop::LogicalSize::new(1300.0, 700.0)),
        ));
}
