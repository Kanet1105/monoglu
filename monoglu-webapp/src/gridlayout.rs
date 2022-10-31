//! Never use the term "row" or "column".
//! Use 'y' for the row and 'x' for the column to avoid shooting myself in the foot.
use std::collections::HashMap;

pub struct Grid {
    area: egui::Area,
    resize: egui::Resize,
    contents: Option<Box<dyn Fn(&mut egui::Ui)>>,
}

impl Grid {
    pub fn new(id: &str) -> Self {
        Self {
            area: egui::Area::new(id),
            resize: egui::Resize::default().resizable(false),
            contents: None,
        }
    }

    pub fn add_contents(&mut self, contents: Box<dyn Fn(&mut egui::Ui)>) {
        self.contents = Some(contents);
    }

    pub fn view_contents(&mut self, ui: &mut egui::Ui) {
        if let Some(contents) = &self.contents {
            contents(ui);
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, offset: &egui::Pos2, grid_size: &egui::Vec2) {
        self.area.fixed_pos(*offset).show(ctx, |ui| {  
            self.resize.fixed_size(*grid_size).show(ui, |ui| {
                self.view_contents(ui);
            });
        });
    }
}

pub struct GridLayout {
    id: String,
    x: usize,
    y: usize,
    stretch: Stretch,
    grid: HashMap<(usize, usize), Grid>,
    policy: SizePolicy,
}

impl GridLayout {
    pub fn new(id: String, x: usize, y: usize, policy: SizePolicy) -> Self {
        let mut stretch = Stretch::new(x, y);

        let mut grid = HashMap::<(usize, usize), Grid>::new();
        for y_index in 0..y {
            for x_index in 0..x {
                let grid_id = format!("{}_{}_{}", &id, x_index, y_index);
                grid.insert((x_index, y_index), Grid::new(&grid_id));
            }
        }

        Self {
            id,
            x,
            y,
            stretch,
            grid,
            policy,
        }
    }

    pub fn get_grid(&mut self, x_index: usize, y_index: usize) -> Option<&mut Grid> {
        self.grid.get_mut(&(x_index, y_index))
    }

    pub fn set_y_stretch(
        &mut self, 
        y_index: usize, 
        value: usize,
    ) -> Result<(), GridError> {
        if y_index >= self.y {
            return Err(GridError::IndexOutofRange(y_index, 0, self.y));
        }
        self.stretch.set_y_stretch(y_index, value);
        Ok(())
    }

    pub fn set_x_stretch(
        &mut self, 
        x_index: usize, 
        value: usize,
    ) -> Result<(), GridError> {
        if x_index >= self.x {
            return Err(GridError::IndexOutofRange(x_index, 0, self.x));
        }
        self.stretch.set_x_stretch(x_index, value);
        Ok(())
    }

    pub fn get_grid_size(&self, ctx: &egui::Context) -> egui::Vec2 {
        let grid_size = match self.policy {
            SizePolicy::Absolute(size) => {
                let size = self.policy.to_vec().unwrap();
                egui::vec2(
                    size.x / self.stretch.x_total(),
                    size.y / self.stretch.y_total(),
                )
            },
            SizePolicy::Responsive(size) => {
                let size = self.policy.to_vec().unwrap();
                egui::vec2(
                    size.x * ctx.available_rect().width() / self.stretch.x_total(),
                    size.y * ctx.available_rect().height() / self.stretch.y_total(),
                )
            },
        };
        grid_size
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        // An offset always starts from the "left-top" corner of the ui.
        let mut offset = egui::pos2(
            ctx.available_rect().left_top().x,
            ctx.available_rect().left_top().y,
        );

        let grid_size = self.get_grid_size(ctx);

        for y_index in 0..self.y {
            for x_index in 0..self.x {
                if let Some(grid) = self.get_grid(x_index, y_index) {
                    grid.show(ctx, &offset, &grid_size);
                }
                offset.x += grid_size.x * self.stretch.get_x_stretch(x_index);
            }
            offset.x = ctx.available_rect().left_top().x;
            offset.y += grid_size.y * self.stretch.get_y_stretch(y_index);
        }
    }
}

pub struct Stretch {
    x_total: usize,
    y_total: usize,
    x: Vec<usize>,
    y: Vec<usize>,
}

impl Stretch {
    pub fn new(x: usize, y: usize) -> Self {
        let mut x_vec = Vec::with_capacity(x);
        for x_index in 0..x {
            x_vec.push(1);
        }

        let mut y_vec = Vec::with_capacity(y);
        for y_index in 0..y {
            y_vec.push(1);
        }

        Self {
            x_total: x,
            y_total: y,
            x: x_vec,
            y: y_vec,
        }
    }

    pub fn set_x_stretch(&mut self, x_index: usize, value: usize) {
        let prev = std::mem::replace(&mut self.x[x_index], value);
        self.x_total -= prev;
        self.x_total += value;
    }

    pub fn set_y_stretch(&mut self, y_index: usize, value: usize) {
        let prev = std::mem::replace(&mut self.y[y_index], value);
        self.y_total -= prev;
        self.y_total += value;
    }

    pub fn get_x_stretch(&self, x_index: usize) -> f32 {
        self.x[x_index] as f32
    }

    pub fn get_y_stretch(&self, y_index: usize) -> f32 {
        self.y[y_index] as f32
    }

    pub fn x_total(&self) -> f32 {
        self.x_total as f32
    }

    pub fn y_total(&self) -> f32 {
        self.y_total as f32
    }
}

pub enum SizePolicy {
    Absolute(egui::Vec2),
    Responsive(egui::Vec2),
}

impl Default for SizePolicy {
    fn default() -> Self {
        Self::Responsive(egui::vec2(1.0, 1.0))
    }
}

impl SizePolicy {
    pub fn absolute(x: f32, y: f32) -> Self {
        Self::Absolute(egui::vec2(x, y))
    }

    pub fn responsive(x: f32, y: f32) -> Self {
        Self::Responsive(egui::vec2(x, y))
    }

    pub fn to_vec(&self) -> Result<egui::Vec2, GridError> {
        match self {
            Self::Absolute(size) => Ok(*size),
            Self::Responsive(size) => {
                if size.x >= 1.0 && size.x < 0.0 {
                    return Err(GridError::ValueOutofRange(size.x, 0.0, 1.0));
                }
                if size.y >= 1.0 && size.y < 0.0 {
                    return Err(GridError::ValueOutofRange(size.y, 0.0, 1.0));
                }
                Ok(*size)
            },
        }
    }
}

use std::fmt::{Debug, Display};

pub enum GridError {
    ValueOutofRange(f32, f32, f32),
    IndexOutofRange(usize, usize, usize),
}

impl Debug for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for GridError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueOutofRange(value, lb, ub) => {
                let message = format!("Out of range: {} < {} <= {}", lb, value, ub);
                write!(f, "{}", message)
            }

            Self::IndexOutofRange(value, lb, ub) => {
                let message = format!("Out of range: {} <= {} < {}", lb, value, ub);
                write!(f, "{}", message)
            }
        }
    }
}

impl std::error::Error for GridError {}
