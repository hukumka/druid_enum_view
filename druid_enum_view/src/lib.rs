pub use druid_enum_view_derive::*;
/// Proc-macro entry to generate
/// `build_widget` function for given enum.
/// 
/// Usage:
/// ```
/// #[derive(EnumViewData)]
/// #[enum_view(StateView)]
/// enum State{
///     #[enum_view(build_widget1)]
///     Stage0(u32),
///     #[enum_view(build_widget2)]
///     Stage1(String),
/// }
/// 
/// fn build_widget1() -> impl Widget<u32>{
///     ...
/// }
///
/// fn build_widget2() -> impl Widget<String>{
///     ...
/// }
/// ```
pub trait EnumViewData {}