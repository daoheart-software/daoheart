use std::env::args;

use editor_widget::EditorWidget;
use iced::{
    Element,
    widget::{column, text},
};

struct Application {}

enum ApplicationMessage {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, _message: ApplicationMessage) {}

    fn view(&self) -> Element<'_, ApplicationMessage> {
        let arg = args()
            .into_iter()
            .skip(1)
            .next()
            .expect("to have a single argument");

        let file = std::fs::File::open(arg).unwrap();

        column![EditorWidget::new(file)].into()
    }
}

fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view).run()
}
