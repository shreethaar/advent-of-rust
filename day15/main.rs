use std::fmt;

// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
}

pub struct ElvesGift {
    pub name: String,
}

pub struct ReindeerGift {
    pub name: String,
}

pub trait Gift // 2. Finish the trait definition //

// 3. Update the function signature
pub fn prepare_gift<T: fmt::Display>(gift: T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

// 4. Implement the Gift trait to the gift structs

impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}
