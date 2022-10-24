use std::collections::BTreeMap;

pub struct Cell {
    id: String,
    area: egui::Area,
    frame: egui::Frame,
    resize: egui::Resize,
}

impl Cell {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            area: egui::Area::new(id),
            frame: egui::Frame::none()
                .inner_margin(0.0)
                .fill(egui::Color32::GRAY),
            resize: egui::Resize::default()
                .resizable(false),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, offset: &egui::Pos2, height: f32, width: f32) {
        self.area
            .default_pos(*offset)
            .movable(true)
            .show(ctx, |ui| {
                self.frame.show(ui, |ui| {
                    self.resize
                        .default_height(height)
                        .default_width(width)
                        .show(ui, |ui| {
                            ui.label(&self.id);
                        });
                });
            });
    }
}

pub struct Grid {
    row: usize,
    col: usize,
    area: egui::Area,
    frame: egui::Frame,
    resize: egui::Resize,
    grid: BTreeMap<(usize, usize), Cell>
    // contents: Vec<Box<dyn egui::Widget>>,
}

impl Grid {
    pub fn new(id: &str, row: usize, col: usize) -> Self {
        let mut grid = BTreeMap::new();
        for y in 0..row {
            for x in 0..col {
                let cell_id = format!("{}_inner_{}_{}", id, y, x);
                log::info!("{}", &cell_id);
                grid.insert((y, x), Cell::new(&cell_id));
            }
        }

        Self {
            row,
            col,
            area: egui::Area::new(id),
            frame: egui::Frame::none()
                .inner_margin(0.0)
                .fill(egui::Color32::BLACK),
            resize: egui::Resize::default()
                .resizable(false),
            grid,
        }
    }

    pub fn get_cell(&mut self, row: usize, col: usize) -> &mut Cell {
        self.grid.get_mut(&(row, col)).unwrap()
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let cell_height = ctx.available_rect().height() / self.row as f32;
        let cell_width = ctx.available_rect().width() / self.col as f32;

        for y in 0..self.row {
            for x in 0..self.col {
                let cell_offset = egui::pos2(
                    ctx.available_rect().min.y + (cell_height * y as f32),
                    ctx.available_rect().min.x + (cell_width * x as f32),
                );
                let cell = self.get_cell(y, x);
                cell.show(ctx, &cell_offset, cell_height, cell_width);
            }
        }
    }
}
