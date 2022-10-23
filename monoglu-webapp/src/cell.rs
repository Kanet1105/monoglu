use std::{
    collections::BTreeMap, hash::Hash,
};

pub struct Cell {
    row: usize,
    col: usize,
    cell: BTreeMap<(usize, usize), egui::Area>,
}

impl Cell {
    pub fn new(id: &str, row: usize, col: usize) -> Self {
        let mut cell = BTreeMap::<(usize, usize), egui::Area>::new();
        for y in 0..row {
            for x in 0..col {
                let area = egui::Area::new(format!("{}_{}_{}", id, row, col))
                    .movable(false);
                cell.insert((y, x), area);
            }
        }

        Self {
            row,
            col,
            cell,
        }
    }

    pub fn get_area<'a>(&'a mut self, row: usize, col: usize) -> &'a mut egui::Area {
        self.cell.get_mut(&(row, col)).unwrap()
    }

    /// Lay inner areas out on the grid.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            for y in 0..self.row {
                ui.horizontal(|ui| {
                    for x in 0..self.col {}
                });
            } 
        });
    }
}
