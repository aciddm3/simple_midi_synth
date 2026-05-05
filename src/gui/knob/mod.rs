// use std::f32::consts::FRAC_PI_2;

// use nih_plug_egui::egui::{Color32, Shape, Stroke, epaint::PathStroke, pos2};

pub mod param_knob;
/* 
pub struct Knob<'a> {
    pub uv_value: &'a mut f32,
    pub color: Color32,
}

impl<'a> nih_plug_egui::egui::Widget for Knob<'a> {
    fn ui(self, ui: &mut nih_plug_egui::egui::Ui) -> nih_plug_egui::egui::Response {
        let desired_size = ui.available_size_before_wrap();
        let (rect, response) =
            ui.allocate_exact_size(desired_size, nih_plug_egui::egui::Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            let knob_radius = rect.width().min(rect.height()) / 2.0;
            let center = rect.center();

            let ang = (0.875 - *self.uv_value) * 4.19;
            let stick_points = vec![
                pos2(
                    knob_radius * ang.cos() + center.x,
                    -knob_radius * ang.sin() + center.y,
                ),
                pos2(
                    0.1 * knob_radius * (ang + FRAC_PI_2).cos() + center.x,
                    -0.1 * knob_radius * (ang + FRAC_PI_2).sin() + center.y,
                ),
                pos2(
                    0.1 * knob_radius * (ang - FRAC_PI_2).cos() + center.x,
                    -0.1 * knob_radius * (ang - FRAC_PI_2).sin() + center.y,
                ),
            ];

            ui.painter()
                .circle_stroke(center, knob_radius, Stroke::new(0.6, self.color));
            ui.painter()
                .add(Shape::Path(nih_plug_egui::egui::epaint::PathShape {
                    points: stick_points,
                    closed: true,
                    fill: self.color,
                    stroke: PathStroke::NONE,
                }));
        }

        if response.hovered() {
            use nih_plug_egui::egui::CursorIcon;
            self.color.gamma_multiply(1.1);
            ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

            let scroll_delta = ui.input(|input| input.smooth_scroll_delta.y);
            *self.uv_value = (*self.uv_value + scroll_delta * 0.01).clamp(0.0, 1.0);
        }

        if response.dragged() {
            let delta_x = response.drag_delta().y;
            *self.uv_value = (*self.uv_value - delta_x * 0.01).clamp(0.0, 1.0);
        }

        response
    }
}
*/