extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(ViewSwitcherData)]
pub fn derive_view_switcher_data(input: TokenStream) -> TokenStream{
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    match view_switcher_data_(input){
        Ok(tt) => tt,
        Err(e) => e.to_compile_error().into()
    }
}

fn view_switcher_data_(input: DeriveInput) -> Result<TokenStream, Error>{
    let enum_ = if let syn::Data::Enum(enum_) = input.data{
        enum_   
    }else{
        return Err(Error::new_spanned(input, "Only enums supported!"));
    };

    // Get variants
    let mut variant_names = vec![];
    let mut variant_types = vec![];
    let mut errors = vec![];
    for var in enum_.variants.iter(){
        match get_variant(var){
            Ok((name, type_)) => {
                variant_names.push(name);
                variant_types.push(type_);
            },
            Err(e) => {
                errors.push(e);
            }
        }
    }
    combine_errors(errors)?;

    // generate code
    let size = variant_names.len();
    let ident = input.ident.clone();
    let mod_name = format!("mod{}", to_snake_case(&input.ident.to_string()));
    let mod_name = syn::Ident::new(&mod_name, ident.span());
    let variant_enum_name = syn::Ident::new(&format!("_Variant{}", input.ident.to_string()), ident.span());
    let repeat_ident: Vec<_> = std::iter::repeat(ident.clone()).take(size).collect();
    let repeat_variant_enum_name: Vec<_> = std::iter::repeat(variant_enum_name.clone()).take(size).collect();
    let lens_names: Vec<_> = variant_names.iter().map(|s| syn::Ident::new(&format!("{}Lens", s), ident.span())).collect();

    let res = quote!{
        impl druid_enum_view::druid::Data for #ident {
            fn same(&self, other: &Self) -> bool {
                match (self, other){
                    #(
                        (#repeat_ident::#variant_names(a), #repeat_ident::#variant_names(b)) => a.same(&b),
                    )*
                    _ => false,
                }
            }
        }

        impl #ident {
            pub fn build_widget() -> impl druid::Widget<Self> {
                druid::widget::ViewSwitcher::new(
                    |data, _env| {#mod_name::#variant_enum_name::new(data)},
                    #mod_name::#variant_enum_name::child_builder
                )
            }
        }

        mod #mod_name{
            use druid_enum_view::druid::{Data, Widget, widget::WidgetExt};
            use druid_enum_view::OptionWidget;
            use super::{#ident, #(#variant_types),*};

            #[derive(PartialEq)]
            pub enum #variant_enum_name{
                #(#variant_names),*
            }

            #(
                pub struct #lens_names;
                impl druid::Lens<#repeat_ident, Option<#variant_types>> for #lens_names{
                    fn with<V, F: FnOnce(&Option<#variant_types>) -> V>(&self, data: &#repeat_ident, f: F) -> V {
                        let data = match &data{
                            #repeat_ident::#variant_names(data) => Some(data.clone()),
                            _ => None
                        };
                        f(&data)
                    }
                    fn with_mut<V, F: FnOnce(&mut Option<#variant_types>) -> V>(&self, data: &mut #repeat_ident, f: F) -> V {
                        let mut opt_data = match &data{
                            #repeat_ident::#variant_names(data) => Some(data.clone()),
                            _ => None
                        };
                        let res = f(&mut opt_data);
                        match data{
                            #repeat_ident::#variant_names(data) => {
                                if let Some(x) = opt_data{
                                    *data = x;
                                }
                            },
                            _ => {}
                        }
                        res
                    }
                }
            )*

            impl #variant_enum_name{
                pub fn new(base: &super::#ident) -> Self{
                    match base{
                        #(#repeat_ident::#variant_names(_) => #repeat_variant_enum_name::#variant_names),*
                    }
                }

                pub fn child_builder(&self, env: &druid::Env) -> Box<dyn druid::Widget<super::#ident>>{
                    match self{
                        #(#repeat_variant_enum_name :: #variant_names 
                            => {
                                let widget = OptionWidget::new(super::#variant_types::build_widget()).lens(#lens_names);
                                Box::new(widget)
                            },)*
                    }
                }
            }
        }
    };
    //println!("{}", res.to_string());
    Ok(res.into())
}

fn to_snake_case(from: &str) -> String{
    let mut res = String::new();
    for c in from.chars(){
        if c.is_uppercase(){
            res.push_str(&format!("_{}", c.to_lowercase()));
        }else{
            res.push(c);
        }
    }
    res
}

fn combine_errors(data: impl IntoIterator<Item=Error>) -> Result<(), Error>{
    let mut iter = data.into_iter();
    if let Some(error) = iter.next(){
        let mut error = error;
        for e in iter{
            error.combine(e);
        }
        Err(error)
    }else{
        Ok(())
    }
}

fn get_variant(var: &syn::Variant) -> Result<(syn::Ident, syn::Type), Error>{
    if var.fields.len() != 1{
        Err(Error::new_spanned(var, "Each variant must contain single values"))?;
    }
    let field = var.fields.iter().next().unwrap();
    if field.ident.is_some(){
        Err(Error::new_spanned(var, "Variants must not containt named fields"))?;
    }
    Ok((var.ident.clone(), field.ty.clone()))
}

