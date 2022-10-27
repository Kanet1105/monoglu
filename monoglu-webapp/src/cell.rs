use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, write},
};

pub struct Cell {
    id: String,
    row: usize,
    col: usize,
    aspect_ratio: [usize; 2],
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
            aspect_ratio: [1, 1],
            area: egui::Area::new(id),
            resize: egui::Resize::default().resizable(false),
            frame: egui::Frame::none().fill(egui::Color32::TRANSPARENT).stroke(egui::Stroke::new(2.0, egui::Color32::DARK_GRAY)),
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
            self.frame.show(ui, |ui| {
                self.resize.fixed_size(egui::vec2(size.x * self.aspect_ratio[0] as f32, size.y * self.aspect_ratio[1] as f32)*size.x ).show(ui, |ui| {
                    self.view_contents(ui);
                });    
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

    /// Input start to end points(x, y) on the same row.
    /// Merging cells should not be diverged from the square. 
    pub fn horizontal_merge(&mut self, begin_cell: (usize, usize), end_cell: (usize, usize)) -> Result<(), GridMergeError>{
        //  Begin - end points should be on the same row.
        if begin_cell.0 != end_cell.0 {
            return Err(GridMergeError::ToBeTheSameRow);
        }

        let row = begin_cell.0;
        let mut begin_col: usize;
        let mut end_col: usize;

        // Setting the lower value of colunm to begin column point.
        if begin_cell.1 == end_cell.1 {
            return  Err(GridMergeError::NotToBeTheSamePoint);
        } else if begin_cell.1 < end_cell.1 {
            begin_col = begin_cell.1;
            end_col = end_cell.1;
        } else {
            begin_col = end_cell.1;
            end_col = begin_cell.1;
        }

        // All of the required points should exist on grid.
        if begin_cell.0 >= self.row || end_col >= self.col {
            return  Err(GridMergeError::OutOfGrid);
        }

        // Before calculating the merging aspect ratio,
        // begin_cell should not be merged from the other cell.
        let mut merging_aspect_ratio: [usize; 2];
        match self.get_cell(row, begin_col) {
            Some(cell) => merging_aspect_ratio = cell.aspect_ratio,
            None => return Err(GridMergeError::CollisionWithMergedCell),
        }
        
        // During accumulating row aspect ratio (width), 
        // the process should be failed if any cell has different column aspect ratio (height)
        for col in (begin_col + 1)..=end_col {
            match self.get_cell(row, col) {
                Some(cell) => {
                    if merging_aspect_ratio[0] == cell.aspect_ratio[0] {
                        merging_aspect_ratio[1] = merging_aspect_ratio[1] + cell.aspect_ratio[1];
                    } else {
                        return Err(GridMergeError::CollisionWithMergedCell);
                    }
                },
                None => {},
            }
        }

        // Inspect the result of calculation (It should be the same with length of row)
        // and then remove all merged cells   
        if merging_aspect_ratio[1] == end_col - begin_col + 1 {
            self.get_cell(row, begin_col)
                .unwrap()
                .aspect_ratio = merging_aspect_ratio;
            
            for col in begin_col + 1..=end_col {
                self.grid.remove(&(row, col));
            }
        } else {
            return Err(GridMergeError::CollisionWithMergedCell);
        }

        Ok(())
    }

    /// input start to end points(x, y) on the same column.
    /// merging cells should not be diverged from the square. 
    pub fn vertical_merge(&mut self, begin_cell: (usize, usize), end_cell: (usize, usize)) -> Result<(), GridMergeError>{
        //  Begin - end points should be on the same column.
        if begin_cell.1 != end_cell.1 {
            return Err(GridMergeError::ToBeTheSameCol);
        } 
        
        let col = begin_cell.1;
        let mut begin_row: usize;
        let mut end_row: usize;

        // Setting the lower value of row to begin row point.
        if begin_cell.0 == end_cell.0 {
            return  Err(GridMergeError::NotToBeTheSamePoint);
        } else if begin_cell.0 < end_cell.0 {
            begin_row = begin_cell.0;
            end_row = end_cell.0;
        } else {
            begin_row = end_cell.0;
            end_row = begin_cell.0;
        }

        // All of the required points should exist on grid.
        if begin_cell.1 >= self.col || end_row >= self.row {
            return  Err(GridMergeError::OutOfGrid);
        }

        // Before calculating the merging aspect ratio,
        // begin_cell should not be merged from the other cell.
        let mut merging_aspect_ratio: [usize; 2];
        match self.get_cell(begin_row, col) {
            Some(cell) => merging_aspect_ratio = cell.aspect_ratio,
            None => return Err(GridMergeError::CollisionWithMergedCell),
        }

        // During accumulating column aspect ratio (height), 
        // the process should be failed if any cell has different row aspect ratio (width)
        for row in begin_row + 1..=end_row {
            match self.get_cell(row, col) {
                Some(cell) => {
                    if merging_aspect_ratio[1] == cell.aspect_ratio[1] {
                        merging_aspect_ratio[0] = merging_aspect_ratio[0] + cell.aspect_ratio[0];
                    } else {
                        return Err(GridMergeError::CollisionWithMergedCell);
                    }
                },
                None => {},
            }
        }

        // Inspect the result of calculation (It should be the same with length of column)
        // and then remove all merged cells
        if merging_aspect_ratio[0] == end_row - begin_row + 1 {
            self.get_cell(begin_row, col)
                .unwrap()
                .aspect_ratio = merging_aspect_ratio;
            
            for row in (begin_row + 1)..=end_row {
                self.grid.remove(&(row, col));
            }
        } else {
            return Err(GridMergeError::CollisionWithMergedCell);
        }

        Ok(())
    }
}


pub enum GridMergeError {
    ToBeTheSameRow,
    ToBeTheSameCol,
    NotToBeTheSamePoint,
    OutOfGrid,
    CollisionWithMergedCell, 
}

impl Debug for GridMergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for GridMergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridMergeError::CollisionWithMergedCell => write!(f, "Collision With Merged Cell"),
            GridMergeError::NotToBeTheSamePoint => write!(f, "Not To Be The Same Point (begin_cell, end_cell)"),
            GridMergeError::OutOfGrid => write!(f, "Out Of Grid"),
            GridMergeError::ToBeTheSameCol => write!(f, "To Be The Same Column"),
            GridMergeError::ToBeTheSameRow => write!(f, "To Be The Same Row"),
        }
    }
}

impl std::error::Error for GridMergeError {}
