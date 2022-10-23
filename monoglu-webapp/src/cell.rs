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
}
