use std::fmt;

enum Unit {
    KILOBYTE,
    MEGABYTE,
    GIGABYTE,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suffix = match self {
            Unit::KILOBYTE => "Kb",
            Unit::MEGABYTE => "Mb",
            Unit::GIGABYTE => "Gb",
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
            0..1_000_000 => Unit::KILOBYTE,
            1_000_000..1_000_000_000 => Unit::MEGABYTE,
            _ => Unit::GIGABYTE,
        };
        return Size { size, unit };
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep_size = (self.size as f32)
            / match self.unit {
                Unit::KILOBYTE => 1_000.0,
                Unit::MEGABYTE => 1_000_000.0,
                Unit::GIGABYTE => 1_000_000_000.0,
            };
        return write!(f, "{:.2}{}", rep_size, self.unit);
    }
}
