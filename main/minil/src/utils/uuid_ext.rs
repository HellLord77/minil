use std::hash::Hash;

use rand::prelude::*;
use rand_seeder::Seeder;
use uuid::Uuid;

pub(crate) trait UuidExt {
    fn v4_from_seed(seed: impl Hash) -> Uuid;
}

impl UuidExt for Uuid {
    fn v4_from_seed(seed: impl Hash) -> Uuid {
        Uuid::from_u128(
            Seeder::from(seed).into_rng::<StdRng>().random::<u128>()
                & 0xFFFFFFFFFFFF4FFFBFFFFFFFFFFFFFFF
                | 0x40008000000000000000,
        )
    }
}
