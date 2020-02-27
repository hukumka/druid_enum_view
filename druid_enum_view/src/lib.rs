pub use druid_enum_view_derive::*;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Size, UpdateCtx, Widget
};
pub use druid;
/// Proc-macro entry to generate
/// `build_widget` function for given enum.
pub trait ViewSwitcherData {}

/// Widget wrapper to turn data `Option<U>` to `U`
/// while supression any event propagation in it is `None`
pub struct OptionWidget<T, U>{
    widget: T,
    _marker: std::marker::PhantomData<U>,
}

impl<T, U> OptionWidget<T, U>{
    pub fn new(widget: T) -> Self{
        Self{
            widget,
            _marker: Default::default(),
        }
    }
}

impl<U: Data, T: Widget<U>> Widget<Option<U>> for OptionWidget<T, U>{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Option<U>, env: &Env) {
        if let Some(data) = data{
            self.widget.event(ctx, event, data, env)
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Option<U>, data: &Option<U>, env: &Env) {
        if let (Some(old_data), Some(data)) = (old_data, data){
            self.widget.update(ctx, old_data, data, env)
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Option<U>, env: &Env) -> Size {
        if let Some(data) = data{
            self.widget.layout(ctx, bc, data, env)
        }else{
            Size::default()
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &Option<U>, env: &Env) {
        if let Some(data) = data{
            self.widget.paint(paint_ctx, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Option<U>, env: &Env) {
        if let Some(data) = data{
            self.widget.lifecycle(ctx, event, data, env)
        }
    }
}
