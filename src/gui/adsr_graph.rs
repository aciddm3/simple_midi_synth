use std::sync::Arc;

use nih_plug_egui::egui;
use nih_plug_egui::egui::{Color32, Stroke, Ui, pos2};

use crate::simple_synth_parameters::SimpleSynthParams;

pub trait DrawADSR {
    fn draw_adsr_graph(&mut self, graph_height: f32, params: Arc<SimpleSynthParams>);
}

impl DrawADSR for Ui {
    fn draw_adsr_graph(&mut self, graph_height: f32, params: Arc<SimpleSynthParams>) {
        // ADSR Graph
        let available_width = self.available_width();

        let (rect, _response) = self.allocate_at_least(
            egui::vec2(available_width, graph_height),
            egui::Sense::focusable_noninteractive(),
        );
        let painter = self.painter_at(rect);

        let height_scale = rect.height();
        let width = rect.width();

        painter.rect_filled(rect, 0.0, Color32::BLACK);

        let atck = params.amp_adsr_attack.value();
        let decay = params.amp_adsr_decay.value();
        let sus = params.amp_adsr_sustain.value() / 100.0;
        let rel = params.amp_adsr_release.value();

        let all_time = atck + decay + rel + 1.0;

        let mut x = rect.left();

        let mut points = vec![pos2(x, rect.bottom())];
        let mut vertical_lines = vec![vec![pos2(x, rect.bottom()), pos2(x, rect.top())]];
        for (phase_width, y) in [atck, decay, 1.0, rel]
            .into_iter()
            .zip([1.0, sus, sus, 0.0])
            .map(|s| (width * s.0 / all_time, rect.bottom() - height_scale * s.1))
        {
            x += phase_width;
            points.push(pos2(x, y));
            vertical_lines.push(vec![pos2(x, rect.bottom()), pos2(x, rect.top())]);
        }
        painter.line(points, Stroke::new(2.0, Color32::RED));
        vertical_lines.into_iter().for_each(|s| {
            painter.line(s, Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 64)));
        });
    }
}
