use std::fmt;

pub struct KidsGift {
    pub name: String,
}

pub struct ElvesGift {
    pub name: String,
}

pub struct ReindeerGift {
    pub name: String,
}


impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kid's Gift: {}", self.name)
    }
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Elf's Gift: {}", self.name)
    }
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Reindeer's Gift: {}", self.name)
    }
}

// Update display_gift to accept any type that implements Display
pub fn display_gift(gift: &impl fmt::Display) {
    println!("{}", gift);
}

pub fn main() {
    let kids_gift = KidsGift {
        name: "toy car".to_string(),
    };
    let elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
    };
    let reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
    };

    display_gift(&kids_gift);
    display_gift(&elves_gift);
    display_gift(&reindeer_gift);
}
