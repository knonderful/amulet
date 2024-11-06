use vui_core::component::*;

pub trait AsComponent {
    type Component;
    fn as_component(&self) -> &Self::Component;
}

pub trait AsComponentMut {
    type Component;
    fn as_component_mut(&mut self) -> &mut Self::Component;
}

pub struct Label<R> {
    component: Text<R>,
}

impl<R> Label<R> {
    pub fn new(text: Text<R>) -> Self {
        Self { component: text }
    }
}

impl<R> AsComponent for Label<R>
where
    R: TextRenderer,
{
    type Component = Text<R>;

    fn as_component(&self) -> &Self::Component {
        &self.component
    }
}

impl<R> AsComponentMut for Label<R>
where
    R: TextRenderer,
{
    type Component = Text<R>;

    fn as_component_mut(&mut self) -> &mut Self::Component {
        &mut self.component
    }
}
