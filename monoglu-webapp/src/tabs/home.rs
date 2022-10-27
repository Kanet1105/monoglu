use crate::cell::Grid;

pub struct Home {
    grid: Grid,
}

impl Home {
    pub fn new() -> Self {
        
        Self {
            grid: Grid::new("ex", 3, 3),
        }
    }
}

impl super::Tab for Home {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn view(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            self.grid.show(ctx);

            for i in (0..3) {
                for j in (0..3) {
                    self.grid.get_cell(j, i)
                    .add_contents(Box::new(|ui| {
                        ui.label(format!("Home"));
                    }));    
                }
            }



        });
    }
}