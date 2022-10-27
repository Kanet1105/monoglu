use std::collections::BTreeMap;

pub struct Cell {
    id: String,
    row: usize,
    col: usize,
    aspect_ratio: egui::Vec2,
    area: egui::Area,
    resize: egui::Resize,
    frame: egui::Frame,
    contents: Option<Box<dyn Fn(&mut egui::Ui)>>,
}

impl Cell {
    pub fn new(id: &str, row: usize, col: usize) -> Self {
        Self {
            id: id.to_string(),
            row,
            col,
            aspect_ratio: egui::vec2(1.0, 1.0),
            area: egui::Area::new(id),
            resize: egui::Resize::default().resizable(false),
            frame: egui::Frame::none().fill(egui::Color32::TRANSPARENT),
            contents: None,
        }
    }

    pub fn add_contents(&mut self, contents: Box<dyn Fn(&mut egui::Ui)>) {
        self.contents = Some(contents);
    }

    pub fn view_contents(&self, ui: &mut egui::Ui) {
        if let Some(contents) = &self.contents {
            contents(ui);
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, offset: &egui::Pos2, size: &egui::Vec2, margin: f32) {
        self.area.current_pos(*offset).movable(false).show(ctx, |ui| {
            self.resize.fixed_size(*size).show(ui, |ui| {
                self.view_contents(ui);
            });
        });
    }
}

/// ```
/// use crate::cell::Grid;
/// 
/// pub struct Example {
///     grid: Grid,
/// }
/// 
/// impl User {
///     pub fn new() -> Self {
///         let mut grid = Grid::new("user_tab_grid", 3, 3);
///         grid.get_cell(1, 1)
///             .add_contents(Box::new(|ui: &mut egui::Ui| {
///                 ui.label("This is the middle cell.");
///             }));
/// 
///         Self { grid }
///     }
/// }
/// ```
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
                grid.insert((y, x), Cell::new(&cell_id, row, col));
            }
        }

        Self {
            row,
            col,
            grid,
        }
    }

    pub fn get_cell(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.grid.get_mut(&(row, col))
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let cell_width = ctx.available_rect().width() / self.col as f32;
        let cell_height = ctx.available_rect().height() / self.row as f32;
        let cell_size = egui::vec2(cell_width, cell_height);
        let cell_margin = ctx.available_rect().width() * 0.02;

        for y in 0..self.row {
            for x in 0..self.col {
                let cell_offset = egui::pos2(
                    ctx.available_rect().min.x + (cell_width * x as f32),
                    ctx.available_rect().min.y + (cell_height * y as f32),
                );

                if let Some(cell) = self.get_cell(y, x) {
                    cell.show(ctx, &cell_offset, &cell_size, cell_margin);
                }
                // self.get_cell(y, x).show(ctx, &cell_offset, &cell_size, cell_margin);
            }
        }
    }
}
