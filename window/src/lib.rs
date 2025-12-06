use editor_component::editor;
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

#[derive(Default)]
struct AppState {}

fn app_logic(_: &mut AppState, init_file_name: String) -> impl WidgetView<AppState> + use<> {
    editor(init_file_name)
}

pub fn window(init_file_name: String) -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState::default(),
        move |it| app_logic(it, init_file_name.clone()),
        WindowOptions::new("title"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
