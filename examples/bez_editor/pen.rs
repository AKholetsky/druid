//! The bezier pen tool.

use druid::kurbo::Point;
use druid::{Event, MouseButton, MouseEvent};

use super::{Contents, Mouse, Path, Tool, MIN_POINT_DISTANCE};

/// The state of the pen.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pen(Mouse);

impl Tool for Pen {
    fn event(&mut self, data: &mut Contents, event: &Event) -> bool {
        match event {
            Event::MouseDown(mouse) if mouse.count == 1 => self.mouse_down(data, mouse),
            Event::MouseDown(mouse) if mouse.count == 2 => self.double_click(data, mouse),
            Event::MouseUp(mouse) => self.mouse_up(data, mouse),
            Event::MouseMoved(mouse) => self.mouse_moved(data, mouse),
            _ => false,
        }
    }

    fn boxed_clone(&self) -> Box<dyn Tool> {
        Box::new(self.clone())
    }

    fn name(&self) -> &str {
        "pen"
    }
}

impl Pen {
    pub(crate) fn new() -> Self {
        Pen(Mouse::Up(Point::ZERO))
    }

    fn mouse_down(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        if event.button != MouseButton::Left {
            return false;
        }

        let point = match canvas.active_path().map(Path::start_point) {
            Some(start) if start.point.distance(event.pos) < MIN_POINT_DISTANCE => start.point,
            _ => event.pos,
        };

        canvas.add_point(point);

        self.0 = Mouse::Down(event.pos);
        true
    }

    fn double_click(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        if event.button != MouseButton::Left {
            return false;
        }

        canvas.selection_mut().clear();
        true
    }

    fn mouse_moved(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        // does this start or change a drag?
        self.0 = match self.0 {
            Mouse::Up(_) => Mouse::Up(event.pos),
            Mouse::Drag { start, current, .. } => Mouse::Drag {
                start,
                last: current,
                current: event.pos,
            },
            Mouse::Down(point) => {
                if point.distance(event.pos) > MIN_POINT_DISTANCE {
                    Mouse::Drag {
                        start: point,
                        last: point,
                        current: event.pos,
                    }
                } else {
                    Mouse::Down(point)
                }
            }
        };
        if let Mouse::Drag { start, current, .. } = self.0 {
            canvas.update_for_drag(start, current);
            true
        } else {
            false
        }
    }

    fn mouse_up(&mut self, canvas: &mut Contents, event: &MouseEvent) -> bool {
        if event.button != MouseButton::Left {
            return false;
        }

        if let Some(path) = canvas.active_path_mut() {
            if path.is_closed() || path.points().len() > 1 && !path.last_segment_is_curve() {
                path.clear_trailing();
            }
        }

        if canvas
            .active_path_mut()
            .map(|p| p.is_closed())
            .unwrap_or(false)
        {
            canvas.selection_mut().clear();
        }

        self.0 = Mouse::Up(event.pos);
        true
    }
}
