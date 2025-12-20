use crate::core::lib::rsx::component::Component;
use std::any::Any;
use std::rc::Rc;

pub trait Panel: Component + Any {
    fn panel_type(&self) -> PanelType;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelType {
    Top,
    Bottom,
    Left,
    Right,
    Central,
}

pub fn is_panel(component: &Rc<dyn Component>) -> Option<PanelType> {
    None
}
