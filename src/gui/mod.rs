mod knob;

use nih_plug::prelude::*;
use nih_plug_egui::{
    create_egui_editor,
    egui::{self, Color32, vec2},
    widgets,
};

use crate::{gui::knob::param_knob::ParamKnob, simple_synth_struct::SimpleSynth};

impl SimpleSynth {
    #[inline]
    pub fn make_gui(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();

        create_egui_editor(
            self.editor_state.clone(),
            (),
            |_ctx, _data| {},
            move |egui_ctx, setter, _data| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label("Master gain");
                                    ui.add_sized(
                                        vec2(40.0, 40.0),
                                        ParamKnob {
                                            setter,
                                            param: &params.gain,
                                            color: Color32::WHITE,
                                        },
                                    );
                                    ui.label(format!(
                                        "{:.2}{}",
                                        params.gain.value(),
                                        params.gain.unit()
                                    ));
                                });
                            });
                            ui.label("Transpose");
                            ui.add(widgets::ParamSlider::for_param(&params.transpose, setter));
                        });
                    })
                });
            },
        )
    }
}
