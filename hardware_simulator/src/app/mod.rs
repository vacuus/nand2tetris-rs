use eframe::egui::{self, Context, TextEdit, Ui, Vec2};
use eframe::epi::Frame;
use std::sync::mpsc;
use derive_more::*;

#[derive(From)]
struct SendRecvPair<T> {
    tx: mpsc::SyncSender<T>,
    rx: mpsc::Receiver<T>,
}

pub struct App {
    code: String,
    code_channel: SendRecvPair<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            code: "When the".to_string(),
            code_channel: mpsc::sync_channel(1).into(),
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &Context, _: &Frame) {
        // check for an input file, and if received, display it
        if let Ok(str) = self.code_channel.rx.try_recv() {
            self.code = str;
        }

        // repaint ui
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |uis| {
                uis[0].vertical_centered_justified(|ui| {
                    if ui.button("Load local file").clicked() {
                        self.load_file();
                    }
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut self.code).code_editor(),
                    );
                });
                uis[1].vertical_centered_justified(|ui| {
                    ui.heading("Pins");
                    self.pin_table(ui);
                });
            });
        });
    }

    fn name(&self) -> &str {
        "Nand2Tetris"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::INFINITY, f32::INFINITY)
    }
}

impl App {
    fn load_file(&mut self) {
        let tx = self.code_channel.tx.clone();
        #[cfg(feature = "web")]
        wasm_bindgen_futures::spawn_local(async move {
            let f = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(f) = f {
                let buf = f.read().await;
                if let Ok(str) = String::from_utf8(buf) {
                    // crate::log(&str);
                    tx.send(str).unwrap(); //TODO: Find a better way to send info
                } else {
                    // crate::log(&format!("Could not decode given file"));
                }
            }
        });
        #[cfg(feature = "native")]
        futures_lite::future::block_on(async move {
            let f = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(f) = f {
                let buf = f.read().await;
                if let Ok(str) = String::from_utf8(buf) {
                    tx.send(str).unwrap();
                }
            }
        })

    }

    fn pin_table(&self, ui: &mut Ui) {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                egui::Grid::new("_external_pin_table")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Pin Name");
                        ui.label("Pin Value");
                        ui.end_row();
                    });
            },
        );
    }
}
