use nih_plug::{
    context::gui::ParamSetter,
    params::{IntParam, Param, range::IntRange},
};
use nih_plug_egui::egui::{Color32, Shape, Stroke, Ui, Widget, epaint::PathStroke, pos2};

use std::f32::consts::FRAC_PI_2;
pub struct ParamKnobInt<'a> {
    pub param: &'a IntParam,
    pub color: Color32,
    pub setter: &'a ParamSetter<'a>,
}

impl<'a> Widget for ParamKnobInt<'a> {
    fn ui(self, ui: &mut Ui) -> nih_plug_egui::egui::Response {
        let desired_size = ui.available_size_before_wrap();
        let (rect, response) =
            ui.allocate_exact_size(desired_size, nih_plug_egui::egui::Sense::click_and_drag());

        let (param_min, param_max) = match self.param.range() {
            IntRange::Linear { min, max } => (min, max),
            IntRange::Reversed(int_range) => match int_range {
                IntRange::Linear { min, max } => (*min, *max),
                _ => (0, 0),
            },
        };

        let mut norm_val = self.param.value() - param_min;
        let divisor = param_max - param_min;

        if response.double_clicked() {
            norm_val = self.param.default_plain_value() - param_min;
        }

        if ui.is_rect_visible(rect) {
            let uv_value = norm_val as f32 / divisor as f32;
            let knob_radius = rect.width().min(rect.height()) / 2.0;
            let center = rect.center();

            let ang = (0.875 - uv_value) * 4.19;
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
            let divisor_f = divisor as f32;
            for val in (0..=divisor).map(|x| (0.875 - x as f32 / divisor_f) * 4.19) {
                ui.painter().line(
                    vec![
                        pos2(
                            knob_radius * val.cos() + center.x,
                            -knob_radius * val.sin() + center.y,
                        ),
                        pos2(
                            1.1 * knob_radius * val.cos() + center.x,
                            -1.1 * knob_radius * val.sin() + center.y,
                        ),
                    ],
                    Stroke::new(1.0, self.color),
                );
            }
            ui.painter()
                .circle_stroke(center, 0.9 * knob_radius, Stroke::new(0.6, self.color));
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

            let scroll_delta = ui.input(|input| input.smooth_scroll_delta.y) / 5.0;
            norm_val = (norm_val + scroll_delta as i32).clamp(0, divisor);
        }

        if response.dragged() {
            let delta_y = -response.drag_delta().y / 5.0;
            norm_val = (norm_val + delta_y as i32).clamp(0, divisor);
        }

        self.setter.begin_set_parameter(self.param);
        self.setter.set_parameter(self.param, norm_val + param_min);
        self.setter.end_set_parameter(self.param);

        response
    }
}
