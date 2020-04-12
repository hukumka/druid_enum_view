use druid::widget::{Button, Flex, Label, WidgetExt, Widget};
use druid::{WindowDesc, AppLauncher, Data};
use druid_enum_view::druid_enum_view;

fn main() {
    let main_window = WindowDesc::new(build_ui);
    let data = State::Stage1("q".to_string());
    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}

fn build_ui() -> impl Widget<State> {
    Flex::column()
        .with_flex_child(
            Button::new("update_state").on_click(|_ctx, data: &mut State, _| {
                *data = State::Stage0(13);
            })
            .padding(5.0),
            1.0,
        )
        .with_flex_child(StateView::new(), 1.0)
}

#[derive(Debug, Clone, PartialEq, Data)]
enum State {
    Stage0(u32),
    Stage1(String),
}

druid_enum_view!{
    pub enum StateView(State)
    {
        #[build_stage0]
        Stage0(u32),
        #[build_stage1]
        Stage1(String)
    }
}

fn build_stage0() -> impl Widget<u32> {
    Button::new(|data: &u32, _env: &_| format!("Count: {}", data))
        .on_click(|_ctx, data: &mut u32, _| {
            *data += 1;
        })
}

fn build_stage1() -> impl Widget<String> {
    Label::new(|data: &String, _env: &_| data.clone())
}