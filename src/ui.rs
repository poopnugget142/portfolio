use egui::{Color32, RichText};
use crate::app::{Page, TemplateApp};

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn about(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.set_max_width(ui.available_width()/2.0);
        ui.label(RichText::new("This is a Portfolio written completely in Rust").size(20.0));
        ui.separator();
        ui.label(RichText::new("ABOUT ME").size(70.0).color(Color32::WHITE));
        ui.separator();

        ui.vertical(|ui| {
            ui.label(RichText::new("
            Hi! I'm Jace, a game programmer from The United States. I have been making games since I was 10. Henceforth, I have published several games and developer tools that have been widely used and have been able to grow a community around myself. I have spent many years teaching myself all the newest programming languages and software so that I may use it for my upcoming projects.
            ").size(15.0));
            ui.label(RichText::new("
            Here you can see all of my works that I've had the privilege of contributing to, or some of my own personal projects that I do for my own enjoyment. I want to continue to learn new talents and improve my skills as a programmer. There is always so much to learn and I am excited to do so as I develop my career.
            ").size(15.0));
        });
    });
}

fn projects(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.set_max_width(ui.available_width()/2.0);

        ui.label(RichText::new("PROJECTS AND EXPERIENCE").size(70.0).color(Color32::WHITE));
        ui.separator();

        egui::Frame::none()
            .show(ui, |ui| {
                ui.label(RichText::new("HELMETS").size(50.0));
                    ui.label("
Helmets is my work in progress top down WW1 shooter. It relies heavilly on predicted rollback netcode that allowes a smooth expierence for all players involved.
                    ");
                ui.add(
                    egui::Image::new(egui::include_image!("../assets/helmets.png"))
                        .max_width(500.0)
                        .rounding(10.0),
                );
            });
    });
}


pub fn ui(app: &mut TemplateApp, ctx: &egui::Context) {
    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::menu::bar(ui, |ui| {
            // NOTE: no File->Quit on web pages!
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            }

            ui.label("JACE WILSON");

            if ui.button("ABOUT").clicked() {
                app.page = Page::About
            };

            if ui.button("PROJECTS AND EXPERIENCE").clicked() {
                app.page = Page::Projects
            };

            ui.hyperlink_to(
                "GITHUB",
                "https://github.com/poopnugget142",
            );
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's

        match app.page  {
            Page::About => {
                about(ui);
            }
            Page::Projects => {
                projects(ui)
            }
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            ui.label(RichText::new("⚠ W.I.P ⚠").color(Color32::YELLOW));
            egui::warn_if_debug_build(ui);
        });
    });
}