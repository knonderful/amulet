use crate::geom::{Point, Rect, Size, Vector};
use crate::mouse::MouseButton;
use crate::VuiResult;
pub use frame::Frame;
pub use mouse_sensor::{MouseSensor, MouseSensorState};
use paste::paste;
pub use position::Position;

mod frame;
mod mouse_sensor;
mod position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FramedPosition {
    absolute_position: Point,
    frame_rect: Rect,
}

impl FramedPosition {
    pub fn new(pos: Point, frame_rect: Rect) -> Self {
        Self {
            absolute_position: pos,
            frame_rect,
        }
    }

    pub fn clip(self, vector: Vector) -> Self {
        Self {
            absolute_position: self.absolute_position,
            frame_rect: self.frame_rect.clip(vector),
        }
    }

    pub fn resize_clipped(self, size: Size) -> Self {
        Self {
            absolute_position: self.absolute_position,
            frame_rect: self.frame_rect.resize_clipped(size),
        }
    }

    pub fn is_hit(&self) -> bool {
        self.frame_rect.contains(self.absolute_position)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentEvent {
    LoopStart,
    MouseMotion(FramedPosition),
    MouseButtonDown(MouseButton, FramedPosition),
    MouseButtonUp(MouseButton, FramedPosition),
}

impl ComponentEvent {
    pub fn clip(self, vector: Vector) -> Self {
        match self {
            ComponentEvent::MouseMotion(pos) => ComponentEvent::MouseMotion(pos.clip(vector)),
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos.clip(vector))
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos.clip(vector))
            }
            other => other,
        }
    }

    pub fn resize(self, size: Size) -> Self {
        match self {
            ComponentEvent::MouseMotion(pos) => {
                ComponentEvent::MouseMotion(pos.resize_clipped(size))
            }
            ComponentEvent::MouseButtonDown(btn, pos) => {
                ComponentEvent::MouseButtonDown(btn, pos.resize_clipped(size))
            }
            ComponentEvent::MouseButtonUp(btn, pos) => {
                ComponentEvent::MouseButtonUp(btn, pos.resize_clipped(size))
            }
            other => other,
        }
    }
}

pub trait HandleEvent {
    type State<'a>;

    fn handle_event(
        &self,
        #[allow(unused)] state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        Ok(event)
    }
}

impl<T> HandleEvent for &T
where
    T: HandleEvent,
{
    type State<'a> = T::State<'a>;

    fn handle_event(
        &self,
        state: Self::State<'_>,
        event: ComponentEvent,
    ) -> VuiResult<ComponentEvent> {
        (*self).handle_event(state, event)
    }
}

pub trait UpdateLayout {
    type State<'a>;

    fn update_layout(
        &self,
        #[allow(unused)] state: Self::State<'_>,
        layout: Layout,
    ) -> VuiResult<Layout> {
        Ok(layout)
    }
}

impl<T> UpdateLayout for &T
where
    T: UpdateLayout,
{
    type State<'a> = T::State<'a>;

    fn update_layout(&self, state: Self::State<'_>, layout: Layout) -> VuiResult<Layout> {
        (*self).update_layout(state, layout)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Layout {
    clip_rect: Rect,
}

impl Layout {
    pub fn new(clip_rect: Rect) -> Self {
        Self { clip_rect }
    }

    pub fn clip_rect(&self) -> Rect {
        self.clip_rect
    }

    pub fn clip(&self, vector: Vector) -> Self {
        Self::new(self.clip_rect.clip(vector))
    }

    pub fn resize_clipped(&self, size: Size) -> Self {
        Self::new(self.clip_rect.resize_clipped(size))
    }
}

pub struct ComponentChain<T> {
    components: T,
}

pub trait AsChain {
    type Target<'a>
    where
        Self: 'a;

    fn as_chain(&self) -> Self::Target<'_>;
}

impl<T> ComponentChain<T> {
    pub fn new(components: T) -> Self {
        Self { components }
    }
}

macro_rules! impl_tuple_component {
    ( () ) => {};
    ( ( $t0:ident $(, $tx:ident)* ) ) => {
        impl<$t0, $($tx,)*> HandleEvent for ComponentChain<&($t0, $($tx,)*)> where $t0 : HandleEvent, $($tx : HandleEvent,)* {
            type State<'a> = ($t0::State<'a>, $($tx::State<'a>,)*);

            fn handle_event(&self, state: Self::State<'_>, event: ComponentEvent) -> VuiResult<ComponentEvent> {
                paste!{
                    let ([<$t0:lower>], $([<$tx:lower>],)*) = &self.components;
                    let ([<$t0:lower _state>], $([<$tx:lower _state>],)*) = state;

                    let event = [<$t0:lower>].handle_event([<$t0:lower _state>], event)?;
                    $(
                    let event = [<$tx:lower>].handle_event([<$tx:lower _state>], event)?;
                    )*
                }
                Ok(event)
            }
        }

        impl<$t0, $($tx,)*> UpdateLayout for ComponentChain<&($t0, $($tx,)*)> where $t0 : UpdateLayout, $($tx : UpdateLayout,)* {
            type State<'a> = ($t0::State<'a>, $($tx::State<'a>,)*);

            fn update_layout(&self, state: Self::State<'_>, layout: Layout) -> VuiResult<Layout> {
                paste!{
                    let ([<$t0:lower>], $([<$tx:lower>],)*) = &self.components;
                    let ([<$t0:lower _state>], $([<$tx:lower _state>],)*) = state;

                    let layout = [<$t0:lower>].update_layout([<$t0:lower _state>], layout)?;
                    $(
                    let layout = [<$tx:lower>].update_layout([<$tx:lower _state>], layout)?;
                    )*
                }
                Ok(layout)
            }

        }

        impl<$t0, $($tx,)*> AsChain for ($t0, $($tx,)*) {
            type Target<'a> = ComponentChain<&'a Self> where $t0 : 'a, $($tx : 'a,)*;

            fn as_chain(&self) -> Self::Target<'_> {
                ComponentChain::new(self)
            }
        }

        impl_tuple_component! { ($($tx),*) }
    };
}

impl_tuple_component! {(A, B, C, E, F, G, H, I, J, K)}

pub trait SizeAttr {
    fn size(&self) -> Size;
}

pub trait PositionAttr {
    fn position(&self) -> Point;
}

#[cfg(test)]
mod test {
    use crate::component::{
        AsChain, ComponentEvent, Frame, FramedPosition, HandleEvent, Layout, Position, UpdateLayout,
    };
    use crate::geom::{Point, Rect};

    #[test]
    fn test_as_chain_handle_event() {
        let comps = (
            Position::new((12, 34).into()),
            Frame::new((100, 200).into()),
        );
        let state = ((), ());
        let event = ComponentEvent::MouseMotion(FramedPosition::new(
            Point::new(123, 456),
            Rect::from_size((80, 300).into()),
        ));
        let event = comps.as_chain().handle_event(state, event).unwrap();
        let expected_event = ComponentEvent::MouseMotion(FramedPosition::new(
            Point::new(123, 456),
            Rect::from_xywh(12, 34, 68, 200),
        ));
        assert_eq!(expected_event, event);
    }

    #[test]
    fn test_as_chain_adjust_layout() {
        let comps = (
            Position::new((12, 34).into()),
            Frame::new((100, 200).into()),
        );
        let state = ((), ());
        let layout = Layout::new(Rect::from_xywh(600, 700, 200, 180));
        let layout = comps.as_chain().update_layout(state, layout).unwrap();
        let expected_layout = Layout::new(Rect::from_xywh(612, 734, 100, 146));

        assert_eq!(expected_layout, layout);
    }
}
