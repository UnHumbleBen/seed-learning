//! An example demonstrating structure and syntax.

// TODO(benlee12): Can we drop this macro?
#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

struct Model {
    count: i32,
    what_we_count: String,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            count: 0,
            // TODO(benlee12): Why do we need into()?
            what_we_count: "click".into(),
        }
    }
}

// Update

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement,
    ChangeWWC(String),
}

/// Updates the model.
// TODO(benlee12): what is _orders?
fn update(msg: Msg, model: &mut Model, _orders: &mut Orders<Msg>) {
    match msg {
        Msg::Increment => model.count += 1,
        Msg::Decrement => model.count -= 1,
        // TODO(benlee12): What is this for?
        Msg::ChangeWWC(what_we_count) => model.what_we_count = what_we_count,
    }
}

// View

/// A simple component.
fn success_level(clicks: i32) -> El<Msg> {
    let descrip = match clicks {
        0...5 => "Not very many 🙁",
        6...9 => "I got my first real six-string 😐",
        10...11 => "Spinal Tap 🙂",
        _ => "Double pendulum 🙃",
    };
    p![descrip]
}

/// The top-level component to pass to the virtual Document Object Model.
fn view(model: &Model) -> El<Msg> {
    let plural = if model.count == 1 { "" } else { "s" };

    // Attributes, style, events, and children may be defined separately.
    // TODO(benlee12): Why use 8 instead of 4 spaces.
    let outer_style = style! {
            "display" => "flex";
            "flex-direction" => "column";
            "text-align" => "center"
    };

    div![
        outer_style,
        div![
            style! {
                // Example of conditional logic in a tyle.
                "color" => if model.count > 4 {"purple"} else {"gray"};
                // When passing numerical values to style!, "px" is implied.
                "border" => "2px solid #004422"; "padding" => 20
            },
            // We use normal Rust code and comments in the view.
            h3![format!(
                "{} {}{} so far",
                model.count, model.what_we_count, plural
            )],
            button![simple_ev(Ev::Click, Msg::Increment), "+"],
            button![simple_ev(Ev::Click, Msg::Decrement), "-"],
            // Optionally-displaying an element.
            if model.count >= 10 {
                h2![style! {"padding" => 50}, "Nice!"]
            } else {
                empty![]
            }
        ],
        success_level(model.count), // Incorporating a separate component.
        h3!["What are we counting?"],
        input![
            attrs! {At::Value => model.what_we_count},
            input_ev(Ev::Input, Msg::ChangeWWC)
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
