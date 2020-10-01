use crate::qr::version::Version;
use crate::qr::QREncodedData;

/// A QR code pixel (the spec calls them "modules" for some reason).
/// White modules are false, black modules are true.
enum Module {
    Unset,
    Data(bool),
    Finder(bool),
    TimingHorizontal(bool),
    TimingVertical(bool),
    Alignment(bool),
    Version(bool),
}

struct QRCode {
    version: &'static Version,
    rows: Vec<Vec<Module>>,
}

impl QRCode {
    fn module(&self, x: usize, y: usize) -> &Module {
        &self.rows[x][y]
    }

    fn set_module(&mut self, module: Module, x: usize, y: usize) {
        self.rows[x][y] = module;
    }

    fn insert_finder(&mut self, x: usize, y: usize) {
        // top row
        for i in 0..7 {
            self.set_module(Module::Finder(true), x, y + i)
        }

        // second row
        self.set_module(Module::Finder(true), x + 1, y);
        for i in 1..6 {
            self.set_module(Module::Finder(false), x + 1, y + i)
        }
        self.set_module(Module::Finder(true), x + 1, y + 6);

        // middle three rows
        for i in 2..5 {
            self.set_module(Module::Finder(true), x + i, y);
            self.set_module(Module::Finder(false), x + i, y + 1);
            self.set_module(Module::Finder(true), x + i, y + 2);
            self.set_module(Module::Finder(true), x + i, y + 3);
            self.set_module(Module::Finder(true), x + i, y + 4);
            self.set_module(Module::Finder(false), x + i, y + 5);
            self.set_module(Module::Finder(true), x + i, y + 6);
        }

        // second to last row
        self.set_module(Module::Finder(true), x + 5, y);
        for i in 1..6 {
            self.set_module(Module::Finder(false), x + 5, y + i)
        }
        self.set_module(Module::Finder(true), x + 5, y + 6);

        // last row
        for i in 0..7 {
            self.set_module(Module::Finder(true), x + 6, y + i)
        }
    }

    fn insert_finders(&mut self) {
        // top left
        self.insert_finder(0, 0);
        // top right
        self.insert_finder((((self.version.num as usize - 1) * 4) + 21) - 7, 0);
        // bottom left
        self.insert_finder(0, (((self.version.num as usize - 1) * 4) + 21) - 7);
    }

    pub fn new(version: &'static Version, bitstream: QREncodedData) -> QRCode {
        let per_side = version.modules_per_side();
        let mut rows = Vec::with_capacity(per_side);
        rows.resize_with(per_side, || {
            let mut row = Vec::with_capacity(per_side);
            row.resize_with(per_side, || Module::Unset);
            row
        });
        QRCode { version, rows }
    }
}
