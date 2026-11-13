use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn find_flag(arg: &str) -> Option<Flags> {
        Flags::ALL
            .iter()
            .find(|current| current.short_flag().eq(arg) || current.long_flag().eq(arg))
            .copied()
    }

    pub fn show_help() {
        println!("---HELP---");
        Flags::ALL.iter().for_each(|current| {
            println!(
                "{} | {} => {}",
                current.short_flag(),
                current.long_flag(),
                current.description()
            )
        });
    }

    // Individual getters for convenience
    pub const fn short_flag(&self) -> &'static str {
        self.details().0
    }

    pub const fn long_flag(&self) -> &'static str {
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

impl FromStr for Flags {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Flags::find_flag(s).ok_or_else(|| format!("This flag {} doesnt exist", s))
    }
}
