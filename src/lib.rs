#[macro_export]
macro_rules! druid_enum_view {
    ($vis: vis enum $widget_name: ident ($data_enum: ident) {
        $(
            #[$builder: expr]
            $(#[$($t: tt)*])*
            $variant: ident ($variant_type: ty)
        ),*
    }) => {
        // Generate widget enum
        $vis enum $widget_name {
            Uninit,
            $(
            $(#[$($t)*])*
            $variant(druid::WidgetPod<$variant_type, Box<dyn druid::Widget<$variant_type>> >)
            ),*
        }

        // Generate impls
        impl $widget_name {
            /// Create new widget
            pub fn new() -> Self {
                Self::Uninit
            }

            // Update current widget variant
            fn update_variant(&mut self, data: &$data_enum) {
                match data {
                    $(
                        $(#[$($t)*])*
                        $data_enum::$variant(value) => {
                            let widget = Box::new($builder());
                            *self = $widget_name::$variant(druid::WidgetPod::new(widget));
                        }
                    ),*
                }
            }
        }

        // generate widget impl
        impl druid::Widget<$data_enum> for $widget_name {
            fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &$data_enum, env: &druid::Env) {
                if let druid::LifeCycle::WidgetAdded = event{
                    self.update_variant(data);
                }
                match (self, data) {
                    $(
                    $(#[$($t)*])*
                    ($widget_name::$variant(ref mut widget), $data_enum::$variant(data)) => widget.lifecycle(ctx, event, data, env),
                    )*
                    _ => {},
                }
            }

            fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut $data_enum, env: &druid::Env) {
                match (self, data) {
                    $(
                    $(#[$($t)*])*
                    ($widget_name::$variant(ref mut widget), $data_enum::$variant(data)) => widget.event(ctx, event, data, env),
                    )*
                    _ => {},
                }
            }

            fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &$data_enum, data: &$data_enum, env: &druid::Env) {
                match (self, data) {
                    $(
                    $(#[$($t)*])*
                    ($widget_name::$variant(ref mut widget), $data_enum::$variant(data)) => widget.update(ctx, data, env),
                    )*
                    (s, data)=> {
                        s.update_variant(data);
                        ctx.children_changed();
                    },
                }
            }

            fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &$data_enum, env: &druid::Env) -> druid::Size {
                match (self, data) {
                    $(
                    $(#[$($t)*])*
                    ($widget_name::$variant(ref mut widget), $data_enum::$variant(data)) => {
                        let size = widget.layout(ctx, bc, data, env);
                        widget.set_layout_rect(druid::Rect::from_origin_size(druid::Point::ORIGIN, size));
                        size
                    },
                    )*
                    _ => {
                        bc.max()
                    },
                }
            }

            fn paint(&mut self, paint_ctx: &mut druid::PaintCtx, data: &$data_enum, env: &druid::Env) {
                match (self, data) {
                    $(
                    $(#[$($t)*])*
                    ($widget_name::$variant(ref mut widget), $data_enum::$variant(data)) => widget.paint(paint_ctx, data, env),
                    )*
                    _ => {}
                }
            }
        }
    };
}