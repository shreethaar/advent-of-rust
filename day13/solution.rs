use std::{cmp::Ordering, error::Error, ops::Deref};

pub fn find_most_dense_location<'a>(locations: &'a [Location]) -> Result<&'a Location, Box<dyn Error>> {
    locations
        .iter()
        .max_by(|a, b| {
            a.density()
                .partial_cmp(&b.density())
                .unwrap_or(Ordering::Equal)
        })
        .ok_or_else(|| "No locations found".into())
}

pub fn find_nearest_location<'a>(locations: &'a [Location]) -> Result<&'a Location, Box<dyn Error>> {
    locations
        .iter()
        .filter(|location| location.density() >= 1000.0)
        .min_by(|a, b| {
            let dist_a = a.x.hypot(a.y);
            let dist_b = b.x.hypot(b.y);
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(Ordering::Equal)
        })
        .ok_or_else(|| "No locations found with density >= 1000.0".into())
}
