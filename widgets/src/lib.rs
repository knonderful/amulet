// use vui_core::component::*;

// pub trait AsComponent {
//     fn as_component(&self) -> &dyn Component;
// }
//
// pub trait AsComponentMut {
//     fn as_component_mut(&mut self) -> &mut dyn Component;
// }
//
// pub struct Label<R> {
//     component: Text<R>,
// }
//
// impl<R> Label<R> {
//     pub fn new(text: Text<R>) -> Self {
//         Self {
//             component: text,
//         }
//     }
// }
//
// impl<R> AsComponent for Label<R> where R: TextRenderer {
//     fn as_component(&self) -> &dyn Component {
//         &self.component
//     }
// }
//
// impl<R> AsComponentMut for Label<R> where R: TextRenderer {
//     fn as_component_mut(&mut self) -> &mut dyn Component {
//         &mut self.component
//     }
// }
