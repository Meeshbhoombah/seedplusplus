use seed::{prelude:: *, *};

enum Msg {
    Increment,
}

type Model = i32;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match Msg {
        Msg::Increment => *model += 1,
    } 
}

fn view() {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

