mod questions;

use rand::prelude::*;
use questions::{SDE, PM, COMMON, ALL};

pub fn pick_rand_sde() -> &'static str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, SDE.len());

    SDE[index]
}

pub fn pick_rand_pm() -> &'static str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, PM.len());

    PM[index]
}

pub fn pick_rand_common() -> &'static str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, COMMON.len());

    COMMON[index]
}

pub fn pick_rand_all() -> &'static str {
    let mut rng = thread_rng();
    let index = rng.gen_range(0, ALL.len());

    ALL[index]
}
