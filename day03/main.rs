pub const GOOD_WEIGHT:f32=1.0;
pub const BAD_WEIGHT:f32=2.0;

pub fn is_nice(good_deeds:u32,bad_deeds:u32) -> bool {
    if good_deeds==0 && bad_deeds == 0 {
        return false 
    }

    let good_score=good_deeds as f32 * GOOD_WEIGHT;
    let bad_score=bad_deeds as f32 * BAD_WEIGHT;
    let nice_ratio = good_score/(good_score+bad_score);
    nice_ratio>=0.75
}

