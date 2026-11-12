#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flags {
    Title,
    Column,
    Line,
    Out,
    Help,
}

impl Flags {
    // Returns all three values as a tuple
    pub const fn details(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Flags::Title => ("-t", "--title", "Show the header line"),
            Flags::Column => ("-c", "--cols", "Show specific columns"),
            Flags::Line => (
                "-l",
                "--line",
                "Show specific lines\n1.Argument: Until line x\n2. Arguments: From line x until line y (x > y)",
            ),
            Flags::Out => ("-o", "--out", "Create an output file"),
            Flags::Help => ("-h", "--help", "Show help"),
        }
    }

    // Individual getters for convenience
    pub const fn short(&self) -> &'static str {
        self.details().0
    }

    pub const fn long(&self) -> &'static str {
        self.details().1
    }

    pub const fn description(&self) -> &'static str {
        self.details().2
    }

    pub const ALL: [Flags; 5] = [
        Flags::Title,
        Flags::Column,
        Flags::Line,
        Flags::Out,
        Flags::Help,
    ];
}
