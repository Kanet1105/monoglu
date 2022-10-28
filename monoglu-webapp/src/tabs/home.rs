use crate::cell::Grid;

pub struct Home {
    grid: Grid,
}

impl Home {
    pub fn new() -> Self {

        let mut grid = Grid::new("home", 10, 10);

        for i in 0..10 {
            for j in 0..10 {
                grid.get_cell(i, j)
                .unwrap()
                .add_contents(Box::new(move |ui| {
                    ui.label(format!("row:{}, col:{}", i, j));
                }));    
            }
        }

        
        // grid.horizontal_merge((1, 3), (1, 4)).unwrap();
        // grid.vertical_merge((2, 1), (3, 1)).unwrap();
        // grid.vertical_merge((2, 2), (3, 2)).unwrap();
        // grid.vertical_merge((2, 3), (3, 3)).unwrap();

        // grid.horizontal_merge((6, 3), (6, 5)).unwrap();
        // grid.vertical_merge((6, 3), (7, 3)).unwrap();
 
        // grid.merge((0, 0), (1, 1)).unwrap();
        // grid.merge((0, 3), (0, 5)).unwrap();
        // grid.merge((2, 0), (5, 0)).unwrap();
        
        // grid.merge((3, 3), (4, 4)).unwrap();
        // grid.merge((3, 3), (5, 5)).unwrap();
       

        Self {
            grid,
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
        });
    }
}