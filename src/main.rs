use std::{env::args, path::Path, sync::Arc};

use editor_widget::state::EditorStateInit;
use editor_widget::{EditorWidget, message::EditorMessage, state::EditorState};
use iced::Element;

mod constants;

#[derive(Debug)]
struct Application {
    es: EditorState,
}

#[derive(Debug)]
enum ApplicationMessage {
    EditorMessage(EditorMessage),
}

impl Application {
    fn new() -> Self {
        let arg = args().nth(1).expect("to have a single argument");
        Self {
            es: EditorState::new(EditorStateInit {
                path: Some(Arc::from(Path::new(&arg))),
                scroll_px: 0.0,
            }),
        }
    }

    fn update(&mut self, _message: ApplicationMessage) {}

    fn view(&self) -> Element<'_, ApplicationMessage> {
        Element::from(EditorWidget::new(&self.es)).map(ApplicationMessage::EditorMessage)
    }
}

fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view).run()
}
