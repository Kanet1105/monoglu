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

    pub fn show(&mut self, ctx: &egui::Context, offset: &egui::Pos2, size: &egui::Vec2) {
        self.area
            .current_pos(*offset)
            .movable(false)
            .show(ctx, |ui| {
                self.frame.show(ui, |ui| {
                    self.resize
                        .fixed_size(*size)
                        // .default_width(width)
                        // .default_height(height)
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
    grid: BTreeMap<(usize, usize), Cell>,
}

impl Grid {
    pub fn new(id: &str, row: usize, col: usize) -> Self {
        let mut grid = BTreeMap::new();
        for y in 0..row {
            for x in 0..col {
                let cell_id = format!("{}_inner_{}_{}", id, y, x);
                grid.insert((y, x), Cell::new(&cell_id));
            }
        }

        Self {
            row,
            col,
            grid,
        }
    }

    pub fn get_cell(&mut self, row: usize, col: usize) -> &mut Cell {
        self.grid.get_mut(&(row, col)).unwrap()
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let cell_width = ctx.available_rect().width() / self.col as f32;
        let cell_height = ctx.available_rect().height() / self.row as f32;
        let cell_size = egui::vec2(cell_width, cell_height);

        for y in 0..self.row {
            for x in 0..self.col {
                let cell_offset = egui::pos2(
                    ctx.available_rect().min.x + (cell_width * x as f32),
                    ctx.available_rect().min.y + (cell_height * y as f32),
                );
                self.get_cell(y, x).show(ctx, &cell_offset, &cell_size);
            }
        }
    }
}
