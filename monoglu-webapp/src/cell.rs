use std::collections::BTreeMap;

pub struct CellLayout {
    row: usize,
    col: usize,
    cell: BTreeMap<(usize, usize), egui::Frame>,
}

impl CellLayout {
    pub fn new(row: usize, col: usize) -> Self {
        let mut cell = BTreeMap::<(usize, usize), egui::Frame>::new();
        for y in 0..row {
            for x in 0..col {
                let frame = egui::Frame::none();
                cell.insert((y, x), frame);
            }
        }

        Self {
            row,
            col,
            cell,
        }
    }

    pub fn get_frame<'a>(&'a mut self, row: usize, col: usize) -> &'a mut egui::Frame {
        self.cell.get_mut(&(row, col)).unwrap()
    }

    /// Lay inner frames out on the grid.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            for y in 0..self.row {
                ui.horizontal(|ui| {
                    for x in 0..self.col {
                        let frame = self.cell
                            .get(&(y, x))
                            .unwrap()
                            .show(ui, |ui| {
                                ui.label(format!("{}, {}", y, x));
                            });
                    }
                });
            } 
        });
    }
}
