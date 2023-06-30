use gui::{Button, Draw, Screen};
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox: {:#?}", self.options)
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 25,
                height: 25,
                options: vec![String::from("Ok"), String::from("Cancel")],
            }),
            Box::new(Button {
                width: 200,
                height: 40,
                label: String::from("Done"),
            }),
        ],
    };
    screen.run();
}
