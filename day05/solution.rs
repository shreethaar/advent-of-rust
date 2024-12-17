pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,

}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        let mut fields=csv_row.split(',');
        let name=fields.next().ok_or("Missing_name")?.to_string();
        let good_deeds:u32=fields
            .next()
            .ok_or("Missing good deeds")?
            .parse()
            .map_err(|_| "Invalid good deeds")?;

        let bad_deeds: u32=fields
            .next()
            .or_or("Missing bad deeds")?
            .parse()
            .map_err(|_| "Invalid bad deeds")?;

        if fields.next().is_some() {
            return Err("Too many fields");
        }

        Ok(Self::new(name, good_deeds, bad_deeds))
    }
}

