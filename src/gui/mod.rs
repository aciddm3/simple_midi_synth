mod param_knob;

use nih_plug::prelude::*;
use nih_plug_egui::{
    create_egui_editor,
    egui::{self, Button, Color32, Rect, pos2, vec2},
    widgets,
};

use crate::{
    background_tasks::BackgroundTasks, gui::param_knob::ParamKnob, simple_synth_struct::SimpleSynth,
};

const DALETH_COLOR: Color32 = Color32::from_rgb(64, 64, 64);

impl SimpleSynth {
    #[inline]
    pub fn make_gui(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();

        let glass_fill = egui::Color32::from_rgba_unmultiplied(25, 25, 35, 180);
        create_egui_editor(
            self.editor_state.clone(),
            (),
            |_ctx, _data| {},
            move |egui_ctx, setter, _data| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(glass_fill))
                    .show(egui_ctx, |ui| {
                        let mut style = (*egui_ctx.style()).clone();
                        let base_alpha = 160;
                        style.visuals.widgets.inactive.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(40, 40, 50, base_alpha);
                        style.visuals.widgets.hovered.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(60, 60, 70, base_alpha + 30);
                        style.visuals.widgets.active.bg_fill =
                            egui::Color32::from_rgba_unmultiplied(80, 80, 90, base_alpha + 50);
                        egui_ctx.set_style(style);

                        let painter = egui_ctx.layer_painter(egui::LayerId::background());
                        let screen = egui_ctx.screen_rect();
                        painter.rect_filled(screen, 0.0, Color32::BLACK);
                        painter.rect_filled(
                            Rect {
                                min: pos2(500.0, 70.0),
                                max: pos2(515.0, 150.0),
                            },
                            0.0,
                            DALETH_COLOR,
                        );
                        painter.rect_filled(
                            Rect {
                                min: pos2(450.0, 55.0),
                                max: pos2(540.0, 70.0),
                            },
                            0.0,
                            DALETH_COLOR,
                        );
                        ui.vertical(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Master Gain");
                                ui.add_sized(
                                    vec2(40.0, 40.0),
                                    ParamKnob {
                                        setter,
                                        color: Color32::WHITE,
                                        param: &params.gain,
                                    },
                                );
                                ui.label("Transpose");
                                ui.add(widgets::ParamSlider::for_param(&params.transpose, setter));

                                let response = ui.add(Button::new("Load File"));
                                if response.clicked() {
                                    async_executor
                                        .execute_background(BackgroundTasks::OpenFileDialog);
                                }
                                let path_guard = params.file_path.read();
                                let filename: &str = path_guard.as_deref().unwrap_or("None");
                                ui.label(format!("File loaded : {filename}"));
                            });
                        })
                    });
            },
        )
    }
}
