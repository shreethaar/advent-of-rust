use std::error::Error;
use std::fmt;

pub struct Kid {
    pub name: String,
    pub gifted: bool,
}

pub struct Reindeer {
    pub name: String,
    pub gifted: bool,
}

pub struct Elf {
    pub name: String,
    pub gifted: bool,
}

pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub trait Giftable {
    fn receive_gift(&mut self);
}

impl Giftable for Kid {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

impl Giftable for Reindeer {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

impl Giftable for Elf {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

pub trait Gift {
    fn wrap(&mut self);
    fn is_wrapped(&self) -> bool;
}
impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

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

pub struct Santa;
impl Santa {
    pub fn give_gift<R, G>(&self, recipient: &mut R, gift: &G) -> Result<(), Box<dyn Error>>
    where
        R: Giftable,
        G: Gift + fmt::Display,
    {
        if gift.is_wrapped() {
            recipient.receive_gift();
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("The gift '{}' is not wrapped!", gift),
            )))
        }
    }
}


pub fn prepare_gift<T: Gift + fmt::Display>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
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

    let mut alice = Kid {
        name: "Alice".to_string(),
        gifted: false,
    };

    let mut prancer = Reindeer {
        name: "Prancer".to_string(),
        gifted: false,
    };

    let mut bernard = Elf {
        name: "Buddy".to_string(),
        gifted: false,
    };

    let santa = Santa;

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);

    if let Ok(_) = santa.give_gift(&mut alice, &kids_gift) {
        println!("{} received {}", alice.name, kids_gift);
        assert_eq!(alice.gifted, true);
    } else {
        panic!("{} should have received {}", alice.name, kids_gift);
    }

    if let Ok(_) = santa.give_gift(&mut prancer, &reindeer_gift) {
        println!("{} received {}", prancer.name, reindeer_gift);
        assert_eq!(prancer.gifted, true);
    } else {
        panic!("{} should have received {}", prancer.name, reindeer_gift);
    }

    if let Ok(_) = santa.give_gift(&mut bernard, &elves_gift) {
        println!("{} received {}", bernard.name, elves_gift);
        assert_eq!(bernard.gifted, true);
    } else {
        panic!("{} should have received {}", bernard.name, elves_gift);
    }
}

