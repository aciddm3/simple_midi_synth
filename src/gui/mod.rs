mod adsr_graph;
mod knob;

use nih_plug::prelude::*;
use nih_plug_egui::{
    create_egui_editor,
    egui::{self, Color32, RichText, vec2},
    widgets,
};

const ADSR_PARAMTERS_TEXT_SIZE: f32 = 14.0;
const ADSR_HEIGHT: f32 = 60.0;
use crate::{
    gui::{
        adsr_graph::DrawADSR,
        knob::{Knob, param_knob::ParamKnob},
    },
    simple_synth_struct::SimpleSynth,
};

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
                        ui.columns(2, |col| {
                            col[0].vertical(|ui| {
                                ui.group(|ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label("Master gain");
                                        ui.add_sized(
                                            vec2(20.0, 20.0),
                                            ParamKnob {
                                                setter,
                                                param : &params.gain,
                                                color : Color32::WHITE,
                                            },
                                        );
                                        ui.label(format!(
                                            "{:.2}{}",
                                            params.gain.value(),
                                            params.gain.unit()
                                        ));
                                    });

                                    ui.label("Osc balance");
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Harm").size(14.0));
                                        ui.add(
                                            widgets::ParamSlider::for_param(
                                                &params.osc_xfade,
                                                setter,
                                            )
                                            .without_value(),
                                        );
                                        ui.label(RichText::new("Saw").size(14.0));
                                    });
                                    ui.label("Transpose");
                                    ui.add(widgets::ParamSlider::for_param(
                                        &params.transpose,
                                        setter,
                                    ));
                                })
                            });
                            col[1].vertical_centered(|ui| {
                                ui.group(|ui| {
                                    ui.draw_adsr_graph(ADSR_HEIGHT, params.clone());

                                    ui.columns(4, |col| {
                                        use knob::Knob;
                                        col[0].vertical_centered(|ui| {
                                            ui.label(
                                                RichText::new("Attack")
                                                    .size(ADSR_PARAMTERS_TEXT_SIZE),
                                            );
                                            let mut val = params.amp_adsr_attack.value() / 8.000;
                                            ui.add_sized(
                                                egui::vec2(40.0, 40.0),
                                                Knob {
                                                    uv_value: &mut val,
                                                    color: Color32::RED,
                                                },
                                            );
                                            setter.begin_set_parameter(&params.amp_adsr_attack);
                                            setter
                                                .set_parameter(&params.amp_adsr_attack, val * 8.0);
                                            setter.end_set_parameter(&params.amp_adsr_attack);
                                        });

                                        col[1].vertical_centered(|ui| {
                                            ui.label(
                                                RichText::new("Decay")
                                                    .size(ADSR_PARAMTERS_TEXT_SIZE),
                                            );
                                            let mut val = params.amp_adsr_decay.value() / 8.000;
                                            ui.add_sized(
                                                egui::vec2(40.0, 40.0),
                                                Knob {
                                                    uv_value: &mut val,
                                                    color: Color32::BLUE,
                                                },
                                            );
                                            setter.begin_set_parameter(&params.amp_adsr_decay);
                                            setter.set_parameter(&params.amp_adsr_decay, val * 8.0);
                                            setter.end_set_parameter(&params.amp_adsr_decay);
                                        });

                                        col[2].vertical_centered(|ui| {
                                            ui.label(
                                                RichText::new("Sustain")
                                                    .size(ADSR_PARAMTERS_TEXT_SIZE),
                                            );
                                            let mut val = params.amp_adsr_sustain.value() / 100.0;
                                            ui.add_sized(
                                                egui::vec2(40.0, 40.0),
                                                Knob {
                                                    uv_value: &mut val,
                                                    color: Color32::GREEN,
                                                },
                                            );
                                            setter.begin_set_parameter(&params.amp_adsr_sustain);
                                            setter.set_parameter(
                                                &params.amp_adsr_sustain,
                                                val * 100.0,
                                            );
                                            setter.end_set_parameter(&params.amp_adsr_sustain);
                                        });

                                        col[3].vertical_centered(|ui| {
                                            ui.label(
                                                RichText::new("Release")
                                                    .size(ADSR_PARAMTERS_TEXT_SIZE),
                                            );
                                            let mut val = params.amp_adsr_release.value() / 8.000;
                                            ui.add_sized(
                                                egui::vec2(40.0, 40.0),
                                                Knob {
                                                    uv_value: &mut val,
                                                    color: Color32::MAGENTA,
                                                },
                                            );
                                            setter.begin_set_parameter(&params.amp_adsr_release);
                                            setter
                                                .set_parameter(&params.amp_adsr_release, val * 8.0);
                                            setter.end_set_parameter(&params.amp_adsr_release);
                                        });
                                    })
                                })
                            })
                        });
                    })
                });
            },
        )
    }
}
