extern crate proc_macro;

use quote::quote;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{parse_macro_input, DeriveInput, Data, Type, Variant, Fields, Error, Attribute, parse::Parse, Path};

#[proc_macro_derive(EnumViewData, attributes(enum_view))]
pub fn derive_view_switcher_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream{
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    match view_switcher_data_(input){
        Ok(tt) => tt.into(),
        Err(e) => e.to_compile_error().into()
    }
}

fn view_switcher_data_(input: DeriveInput) -> Result<TokenStream, Error> {
    let widget_name: Ident = get_attr(&input.attrs)?;
    let enum_ = if let Data::Enum(data) = &input.data{
        data
    }else{
        return Err(Error::new(Span::call_site(), "Data must be enum"));
    };
    let variant_names: Vec<&Ident> = enum_.variants
        .iter()
        .map(|x| &x.ident)
        .collect();
    let variant_view_builders: Result<Vec<Path>, Error> = enum_.variants
        .iter()
        .map(|x| {
            let path: Path = get_attr(&x.attrs)?;
            Ok(path)
        }).collect();
    let variant_types: Result<Vec<&Type>, Error> = enum_.variants
        .iter()
        .map(get_variant_type)
        .collect();
    let variant_view_builders = variant_view_builders?;
    let variant_types = variant_types?;
    let widget = generate_widget(&widget_name, &input.ident, &variant_names, &variant_view_builders, &variant_types);
    let implementation = generate_impl(&input.ident, &variant_names);
    Ok(quote! {
        #widget
        #implementation
    })
}

fn get_variant_type(variant: &Variant) -> Result<&Type, Error>{
    match &variant.fields{
        Fields::Unnamed(var) if var.unnamed.len() == 1 => {
            let field = var.unnamed.first().unwrap();
            Ok(&field.ty)
        },
        _ => Err(Error::new(Span::call_site(), "Expected single unnamed parameter"))
    }
}

fn generate_widget(name: &Ident, enum_ident: &Ident, variants: &[&Ident], view_builders: &[Path], types: &[&Type]) -> TokenStream {
    let enum_ident_repeat: Vec<_> = std::iter::repeat(enum_ident).take(variants.len()).collect();
    let name_repeat: Vec<_> = std::iter::repeat(name).take(variants.len()).collect();
    let res = quote! {
        enum #name{
            Uninit,
            #(
            #variants(druid::WidgetPod<#types, Box<dyn druid::Widget<#types>>>),
            )*
        }

        impl #name{
            fn new() -> Self{
                Self::Uninit
            }

            fn update_widget(&mut self, data: &#enum_ident) {
                match data{
                    #(
                    #enum_ident_repeat::#variants(_) => {
                        let widget = Box::new(#view_builders());
                        *self = #name_repeat::#variants(druid::WidgetPod::new(widget));
                    },
                    )*
                }
            }
        }

        impl druid::Widget<#enum_ident> for #name{
            fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &#enum_ident, env: &druid::Env) {
                if let druid::LifeCycle::WidgetAdded = event{
                    self.update_widget(data);
                }
                match (self, data) {
                    #(
                    (#name_repeat::#variants(ref mut widget), #enum_ident_repeat::#variants(data)) => widget.lifecycle(ctx, event, data, env),
                    )*
                    _ => {},
                }
            }

            fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut #enum_ident, env: &druid::Env) {
                match (self, data) {
                    #(
                    (#name_repeat::#variants(ref mut widget), #enum_ident_repeat::#variants(data)) => widget.event(ctx, event, data, env),
                    )*
                    _ => {},
                }
            }

            fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &#enum_ident, data: &#enum_ident, env: &druid::Env) {
                match (self, data) {
                    #(
                    (#name_repeat::#variants(ref mut widget), #enum_ident_repeat::#variants(data)) => widget.update(ctx, data, env),
                    )*
                    (s, data)=> {
                        s.update_widget(data);
                        ctx.children_changed();
                    },
                }
            }

            fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &#enum_ident, env: &druid::Env) -> druid::Size {
                match (self, data) {
                    #(
                    (#name_repeat::#variants(ref mut widget), #enum_ident_repeat::#variants(data)) => {
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

            fn paint(&mut self, paint_ctx: &mut druid::PaintCtx, data: &#enum_ident, env: &druid::Env) {
                match (self, data) {
                    #(
                    (#name_repeat::#variants(ref mut widget), #enum_ident_repeat::#variants(data)) => widget.paint(paint_ctx, data, env),
                    )*
                    _ => {}
                }
            }
        }
    };
    res
}

fn generate_impl(ident: &Ident, variants: &[&Ident]) -> TokenStream {
    quote!{
        impl druid::Data for #ident{
            fn same(&self, other: &Self) -> bool {
                match (self, other) {
                    #((#ident::#variants(a), #ident::#variants(b)) => a.same(b),)*
                    _ => false,
                }
            }
        }
    }
}

fn get_attr<T: Parse>(attrs: &[Attribute]) -> Result<T, Error> {
    let attr = attrs
        .iter()
        .find(|x| {
            let ident = Ident::new("enum_view", Span::call_site());
            x.path.is_ident(&ident)
        })
        .ok_or_else(|| syn::Error::new(Span::call_site(), "Expected `#[enum_view(...)] attribute"))?;
    attr.parse_args::<T>()
}
