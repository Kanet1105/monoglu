use crate::gridlayout::*;

pub struct Dashboard {
    general_mobile: GridLayout,
    general_normal: GridLayout,
    general_wide: GridLayout,
    detail_mobile: GridLayout,
    detail_normal: GridLayout,
    detail_wide: GridLayout,
    graph_mobile: GridLayout,
    graph_normal: GridLayout,
    graph_wide: GridLayout,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            general_mobile: general_mobile(),
            general_normal: general_normal(),
            general_wide: general_wide(),
            detail_mobile: detail_mobile(),
            detail_normal: detail_normal(),
            detail_wide: detail_wide(),
            graph_mobile: graph_mobile(),
            graph_normal: graph_normal(),
            graph_wide: graph_wide(),
        }
    }
}

impl super::Tab for Dashboard {
    fn name(&self) -> &'static str {
        "Dashboard"
    }

    fn view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _data_states: &crate::data::DataStates) {
        let browser_width = ctx.available_rect().width();

        if browser_width < 550.0 {
            egui::CentralPanel::default().show(ctx, |_ui|{
                self.general_mobile.show(ctx);
                self.detail_mobile.show(ctx);
                self.graph_mobile.show(ctx);
            });            

        } else if browser_width < 1100.0 {
            egui::TopBottomPanel::top("general_normal").min_height(30.0).show(ctx, |_ui| {
                self.general_normal.show(ctx);
            });

            egui::SidePanel::left("detail_normal").min_width(200.0).show(ctx, |_ui| {
                self.detail_normal.show(ctx);
            });

            egui::CentralPanel::default().show(ctx, |_ui|{
                self.general_normal.show(ctx);
            });
        } else {
            egui::SidePanel::left("general_wide").min_width(200.0).show(ctx, |_ui| {
                self.general_wide.show(ctx);
            });

            egui::SidePanel::left("detail_wide").min_width(400.0).show(ctx, |_ui| {
                self.detail_wide.show(ctx);
            });

            egui::CentralPanel::default().show(ctx, |_ui| {
                self.general_wide.show(ctx);
            });
        }


    }
}


fn general_mobile() -> GridLayout {
    let layout = GridLayout::new("general_mobile".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));
    layout
}

fn general_normal() -> GridLayout {
    let layout = GridLayout::new("general_normal".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn general_wide() -> GridLayout {
    let layout = GridLayout::new("general_wide".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn detail_mobile() -> GridLayout {
    let layout = GridLayout::new("detail_mobile".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn detail_normal() -> GridLayout {
    let layout = GridLayout::new("detail_normal".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn detail_wide() -> GridLayout {
    let layout = GridLayout::new("detail_wide".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn graph_mobile() -> GridLayout {
    let layout = GridLayout::new("graph_mobile".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn graph_normal() -> GridLayout {
    let layout = GridLayout::new("graph_normal".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}
fn graph_wide() -> GridLayout {
    let layout = GridLayout::new("graph_wide".into(), 1, 1, SizePolicy::responsive(1.0, 1.0));

    layout
}