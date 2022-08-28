use monoglu_webapp::App;

fn main() {
    monoglu_core::init_once().unwrap();
    yew::start_app::<App>();
}