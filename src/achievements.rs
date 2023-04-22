struct Achievements {
    name_overflow: bool,   // Crash the game by choosing a big name
    unstable: bool,        // Crash the game by unstability
    second_chance: bool,   // Regain full health by underflowing HP
    over_healed: bool,     // Go back to zero health by regeneration
    unlimited_power: bool, // Use the special ability without having any energy for it
    over_9000: bool,       // Overflow your energy back to zero by regeneration too much
    up: bool,              // Leave the map
    reload_anyway: bool,   // Restart loading by
}

impl Achievements {
    pub fn new() -> Self {
        Self {
            name_overflow: false,
            unstable: false,
            second_chance: false,
            over_healed: false,
            unlimited_power: false,
            over_9000: false,
            up: false,
            reload_anyway: false,
        }
    }
}
