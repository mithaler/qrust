use crate::qr::image::save_qrcode;
use crate::qr::version::Version;
use crate::qr::{Error, QREncodedData};
use std::path::Path;

const FIRST_POSITION: i32 = 6;

/// Calculates the alignment pattern centers, according to Table E.1 of the spec.
/// Algorithm from StackOverflow:
/// https://stackoverflow.com/questions/13238704/calculating-the-position-of-qr-code-alignment-patterns/51370697#51370697
fn alignment_pattern_centers(version_num: u8) -> Vec<usize> {
    let pattern_count = (version_num / 7 + 2) as i32;
    let mut positions = Vec::with_capacity(pattern_count as usize);
    if version_num > 1 {
        positions.push(FIRST_POSITION as usize);
        let matrix_width = (17 + 4 * version_num) as i32;
        let last_position = matrix_width - 1 - FIRST_POSITION;
        let second_last_position =
            ((FIRST_POSITION + last_position * (pattern_count - 2) + (pattern_count - 1) / 2)
                / (pattern_count - 1))
                & -2;
        let step = last_position - second_last_position;
        let second_position = last_position - (pattern_count - 2) * step;
        for position in (second_position..(last_position + 1)).step_by(step as usize) {
            positions.push(position as usize);
        }
    }
    positions
}

type Coordinates = (usize, usize);

/// Returns all the coordinates of the centers of the alignment patterns for the version number.
/// Does not exclude the patterns that overlap with finder patterns; the caller must handle that.
fn alignment_pattern_coordinates(version_num: u8) -> Vec<Coordinates> {
    let centers = alignment_pattern_centers(version_num);
    let mut coords = Vec::new();
    for center in &centers {
        for next in &centers {
            coords.push((*center, *next));
        }
    }
    coords
}

/// A QR code pixel (the spec calls them "modules" for some reason).
/// White modules are false, black modules are true.
pub enum Module {
    Unset,
    Data(bool),
    Finder(bool),
    TimingHorizontal(bool),
    TimingVertical(bool),
    Alignment(bool),
    Format(bool),
    Dark,
    Version(bool),
}

impl Module {
    pub fn black(&self) -> bool {
        match self {
            Self::Unset => false,
            Self::Dark => true,
            Self::Data(black)
            | Self::Finder(black)
            | Self::TimingHorizontal(black)
            | Self::TimingVertical(black)
            | Self::Alignment(black)
            | Self::Format(black)
            | Self::Version(black) => *black,
        }
    }
}

pub struct QRCode {
    pub version: &'static Version,
    pub rows: Vec<Vec<Module>>,
}

impl QRCode {
    fn module(&self, (x, y): Coordinates) -> &Module {
        &self.rows[x][y]
    }

    fn set_module(&mut self, module: Module, (x, y): Coordinates) {
        self.rows[x][y] = module;
    }

    fn insert_timing_bands(&mut self) {
        let mut black = true;
        for x in 8..(self.version.modules_per_side() - 8) {
            self.set_module(Module::TimingHorizontal(black), (x, 6));
            black = !black;
        }
        black = true;
        for y in 8..(self.version.modules_per_side() - 8) {
            self.set_module(Module::TimingVertical(black), (6, y));
            black = !black;
        }
    }

    fn insert_finder(&mut self, (x, y): Coordinates, horiz_backwards: bool, vert_backwards: bool) {
        let mut curr_row = y;

        // first empty row if on bottom
        if vert_backwards {
            for i in 0..7 {
                self.set_module(Module::Finder(false), (x + i, curr_row - 1));
            }
        }

        // top row
        for i in 0..7 {
            self.set_module(Module::Finder(true), (x + i, curr_row))
        }

        // second row
        curr_row += 1;
        self.set_module(Module::Finder(true), (x, curr_row));
        for i in 1..6 {
            self.set_module(Module::Finder(false), (x + i, curr_row))
        }
        self.set_module(Module::Finder(true), (x + 6, curr_row));

        // middle three rows
        for _ in 2..5 {
            curr_row += 1;
            self.set_module(Module::Finder(true), (x, curr_row));
            self.set_module(Module::Finder(false), (x + 1, curr_row));
            self.set_module(Module::Finder(true), (x + 2, curr_row));
            self.set_module(Module::Finder(true), (x + 3, curr_row));
            self.set_module(Module::Finder(true), (x + 4, curr_row));
            self.set_module(Module::Finder(false), (x + 5, curr_row));
            self.set_module(Module::Finder(true), (x + 6, curr_row));
        }

        // second to last row
        curr_row += 1;
        self.set_module(Module::Finder(true), (x, curr_row));
        for i in 1..6 {
            self.set_module(Module::Finder(false), (x + i, curr_row))
        }
        self.set_module(Module::Finder(true), (x + 6, curr_row));

        // last row
        curr_row += 1;
        for i in 0..7 {
            self.set_module(Module::Finder(true), (x + i, curr_row));
        }

        // last empty row if on top
        if !vert_backwards {
            curr_row += 1;
            for i in 0..7 {
                self.set_module(Module::Finder(false), (x + i, curr_row));
            }
        }

        // last horizontal column
        let last_column_x = if horiz_backwards { x - 1 } else { x + 7 };
        let y_start = if vert_backwards { y - 1 } else { y };
        for i in 0..8 {
            self.set_module(Module::Finder(false), (last_column_x, y_start + i));
        }
    }

    fn insert_finders(&mut self) {
        // top left
        self.insert_finder((0, 0), false, false);
        // top right
        self.insert_finder(
            ((((self.version.num as usize - 1) * 4) + 21) - 7, 0),
            true,
            false,
        );
        // bottom left
        self.insert_finder(
            (0, (((self.version.num as usize - 1) * 4) + 21) - 7),
            false,
            true,
        );
    }

    fn insert_alignment_pattern(&mut self, center_x: usize, center_y: usize) {
        let (x, mut y) = (center_x - 2, center_y - 2);

        // top row
        for i in 0..5 {
            self.set_module(Module::Alignment(true), (x + i, y));
        }

        // second row
        y += 1;
        self.set_module(Module::Alignment(true), (x, y));
        for i in 1..3 {
            self.set_module(Module::Alignment(false), (x + i, y));
        }
        self.set_module(Module::Alignment(true), (x + 4, y));

        // third row
        y += 1;
        self.set_module(Module::Alignment(true), (x, y));
        self.set_module(Module::Alignment(false), (x + 1, y));
        self.set_module(Module::Alignment(true), (x + 2, y));
        self.set_module(Module::Alignment(false), (x + 3, y));
        self.set_module(Module::Alignment(true), (x + 4, y));

        // fourth row
        y += 1;
        self.set_module(Module::Alignment(true), (x, y));
        for i in 1..3 {
            self.set_module(Module::Alignment(false), (x + i, y));
        }
        self.set_module(Module::Alignment(true), (x + 4, y));

        // last row
        y += 1;
        for i in 0..5 {
            self.set_module(Module::Alignment(true), (x + i, y));
        }
    }

    fn insert_alignment_patterns(&mut self) {
        let center_coords = alignment_pattern_coordinates(self.version.num);
        for (x, y) in center_coords {
            match self.module((x, y)) {
                Module::Finder(_) => (),
                _ => self.insert_alignment_pattern(x, y),
            };
        }
    }

    fn insert_format_and_dark(&mut self) {
        let edge = self.version.modules_per_side() - 1;

        // dark module
        self.set_module(Module::Dark, (8, edge - 7));

        // TODO: replace with a scanner iterator?
        // top left
        for i in 0..8 {
            let coords = (i, 8);
            if let Module::TimingVertical(_) = self.module(coords) {
                continue;
            }
            self.set_module(Module::Format(false), coords)
        }
        for i in 0..9 {
            let coords = (8, i);
            if let Module::TimingHorizontal(_) = self.module(coords) {
                continue;
            }
            self.set_module(Module::Format(false), coords)
        }

        // bottom left
        for i in 0..7 {
            self.set_module(Module::Format(false), (8, edge - i))
        }

        // top right
        for i in 0..7 {
            self.set_module(Module::Format(false), (edge - i, 8))
        }
    }

    fn insert_version_blocks(&mut self) {
        // TODO
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        save_qrcode(self, path)
    }

    pub fn new(version: &'static Version, _bitstream: QREncodedData) -> QRCode {
        let per_side = version.modules_per_side();
        let mut rows = Vec::with_capacity(per_side);
        rows.resize_with(per_side, || {
            let mut row = Vec::with_capacity(per_side);
            row.resize_with(per_side, || Module::Unset);
            row
        });
        let mut code = QRCode { version, rows };
        code.insert_finders();
        code.insert_timing_bands();
        code.insert_alignment_patterns();
        code.insert_format_and_dark();
        code.insert_version_blocks();
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_pattern_centers() {
        assert_eq!(alignment_pattern_centers(1), Vec::<usize>::new());
        assert_eq!(alignment_pattern_centers(6), vec![6, 34]);
        assert_eq!(alignment_pattern_centers(20), vec![6, 34, 62, 90]);
        assert_eq!(alignment_pattern_centers(32), vec![6, 34, 60, 86, 112, 138]);
        assert_eq!(
            alignment_pattern_centers(39),
            vec![6, 26, 54, 82, 110, 138, 166]
        );
        assert_eq!(
            alignment_pattern_centers(40),
            vec![6, 30, 58, 86, 114, 142, 170]
        );
    }

    #[test]
    fn test_alignment_pattern_coordinates() {
        assert_eq!(alignment_pattern_coordinates(1), Vec::<Coordinates>::new());
        assert_eq!(
            alignment_pattern_coordinates(6),
            vec![(6, 6), (6, 34), (34, 6), (34, 34)]
        );
        assert_eq!(
            alignment_pattern_coordinates(7),
            vec![
                (6, 6),
                (6, 22),
                (6, 38),
                (22, 6),
                (22, 22),
                (22, 38),
                (38, 6),
                (38, 22),
                (38, 38)
            ]
        );
    }
}
