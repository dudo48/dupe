use std::fmt;

enum Unit {
    Kilobyte,
    Megabyte,
    Gigabyte,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suffix = match self {
            Unit::Kilobyte => "Kb",
            Unit::Megabyte => "Mb",
            Unit::Gigabyte => "Gb",
        };
        return write!(f, "{suffix}");
    }
}

pub struct Size {
    size: u64,
    unit: Unit,
}

impl Size {
    pub fn new(size: u64) -> Self {
        let unit = match size {
            0..1_000_000 => Unit::Kilobyte,
            1_000_000..1_000_000_000 => Unit::Megabyte,
            _ => Unit::Gigabyte,
        };
        return Size { size, unit };
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep_size = (self.size as f32)
            / match self.unit {
                Unit::Kilobyte => 1_000.0,
                Unit::Megabyte => 1_000_000.0,
                Unit::Gigabyte => 1_000_000_000.0,
            };
        return write!(f, "{:.2}{}", rep_size, self.unit);
    }
}
