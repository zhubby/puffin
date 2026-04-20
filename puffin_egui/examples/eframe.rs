use eframe::egui;

struct MyApp {
    frame_counter: u64,
    keep_repainting: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        puffin::profile_function!();
        puffin::GlobalProfiler::lock().new_frame();

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut profile = puffin::are_scopes_on();
            ui.checkbox(&mut profile, "Show profiler window");
            puffin::set_scopes_on(profile);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.keep_repainting, "Keep repainting this window");
                if self.keep_repainting {
                    ui.spinner();
                    ctx.request_repaint();
                }
            });

            if ui.button("Quit").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });

        puffin_egui::show_viewport_if_enabled(ctx);

        std::thread::Builder::new()
            .name("Other thread".to_owned())
            .spawn(|| {
                sleep_ms(5);
            })
            .unwrap();

        sleep_ms(9);
        if self.frame_counter % 49 == 0 {
            puffin::profile_scope!("Spike");
            std::thread::sleep(std::time::Duration::from_millis(20))
        }
        if self.frame_counter % 343 == 0 {
            puffin::profile_scope!("Big spike");
            std::thread::sleep(std::time::Duration::from_millis(50))
        }
        if self.frame_counter % 55 == 0 {
            for (name, ms) in [("First".to_string(), 20), ("Second".to_string(), 15)] {
                puffin::profile_scope!("Spike", name);
                std::thread::sleep(std::time::Duration::from_millis(ms))
            }
            for (_name, ms) in [("First".to_string(), 20), ("Second".to_string(), 15)] {
                puffin::profile_scope!("Spike");
                std::thread::sleep(std::time::Duration::from_millis(ms))
            }
        }

        for _ in 0..1000 {
            puffin::profile_scope!("very thin");
        }

        self.frame_counter += 1;
    }
}

fn main() -> eframe::Result<()> {
    puffin::set_scopes_on(true);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "puffin egui eframe",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp { frame_counter: 0, keep_repainting: true }))),
    )
}

fn sleep_ms(ms: usize) {
    puffin::profile_function_if!(ms > 1);
    match ms {
        0 => {}
        1 => std::thread::sleep(std::time::Duration::from_millis(1)),
        _ => {
            sleep_ms(ms / 2);
            sleep_ms(ms - (ms / 2));
        }
    }
}
