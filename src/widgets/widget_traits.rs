use crate::Assets;
use quicksilver::{geom::Vector, lifecycle::Window};

///Turns a simple configuration into a real widget that can be drawn and interacted with.
pub trait WidgetConfig<R: Sized, W: Widget> {
    fn to_widget(self) -> (W, R);
}
///This is the real widget. It isn't meant to interact directly with except when creating other widgets that exist of multiple smaller ones
pub trait Widget {
    fn contains(&self, pos: &Vector) -> bool;
    fn is_focusable(&self) -> bool;
    fn render(&self, assets: &dyn Assets, window: &mut Window, z: u32);
    fn get_cursor_on_hover(&self) -> quicksilver::input::MouseCursor {
        quicksilver::input::MouseCursor::Default
    }
    fn set_focus(&mut self, _: bool) {}
    fn set_hover(&mut self, _: bool) {}
    fn on_click(&mut self, _location: &Vector) {}
    fn on_key_press(
        &mut self,
        _key: quicksilver::input::Key,
        _state: quicksilver::input::ButtonState,
    ) {
    }
    fn on_typed(&mut self, _char: char) {}
}