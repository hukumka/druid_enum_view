use druid::widget::{Button, Flex, Label, WidgetExt, Widget};
use druid::{WindowDesc, AppLauncher};
use druid_enum_view::EnumViewData;

fn main() {
    let main_window = WindowDesc::new(build_ui);
    let data = State::Stage1("q".to_string());
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("launch failed");
}

fn build_ui() -> impl Widget<State> {
    Flex::column()
        .with_child(
            Button::new("update_state", |_ctx, data: &mut State, _| {
                *data = State::Stage0(13);
            })
            .padding(5.0),
            1.0,
        )
        .with_child(StateView::new(), 1.0)
}

#[derive(Debug, Clone, PartialEq, EnumViewData)]
#[enum_view(StateView)]
enum State {
    #[enum_view(build_stage0)]
    Stage0(u32),
    #[enum_view(build_stage1)]
    Stage1(String),
}

fn build_stage0(_value: &u32) -> impl Widget<u32> {
    Button::new(
        |data: &u32, _env: &_| format!("Count: {}", data),
        |_ctx, data: &mut u32, _| {
            *data += 1;
        },
    )
}

fn build_stage1(_value: &String) -> impl Widget<String> {
    Label::new(|data: &String, _env: &_| data.clone())
}