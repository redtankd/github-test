#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

use std::collections::HashMap;
use std::convert::TryInto;

use failure::Fail;

use rand::{thread_rng, Rng};

const MAX_ENTITY_COUNT: u64 = 10000;
const ENTITY_SQNO_MOD: u64 = 100000;

type LimitAmount = u64;

struct TwoWayLimit {
    left: LimitAmount,
    right: LimitAmount,
    double: LimitAmount,
}

#[derive(Debug, Fail)]
#[fail(display = "An error occurred.")]
struct LimitUnavailableError;

impl TwoWayLimit {
    fn new() -> TwoWayLimit {
        TwoWayLimit {
            left: 0,
            right: 0,
            double: 0,
        }
    }

    fn try_deduct(&mut self, amount: LimitAmount) -> Result<(), Box<dyn std::error::Error>> {
        if self.double >= amount {
            self.left -= amount;
            self.right -= amount;
            self.double -= amount;
            return Ok(());
        } else {
            return Err(Box::new(LimitUnavailableError.compat()));
        }
    }

    fn try_deduct_available(
        &mut self,
        amount: LimitAmount,
    ) -> Result<LimitAmount, Box<dyn std::error::Error>> {
        let available = amount.min(self.double);
        if available == 0 {
            return Err(Box::new(LimitUnavailableError.compat()));
        }
        self.left -= available;
        self.right -= available;
        self.double -= available;
        return Ok(available);
    }
}

impl Default for TwoWayLimit {
    fn default() -> TwoWayLimit {
        TwoWayLimit::new()
    }
}

// let mut rng = thread_rng();
//         let left = rng.gen_range(1, 101) * 1000;
//         let right = rng.gen_range(1, 101) * 1000;
//         let double = left.min(right);

//         TwoWayLimit {
//             left: left,
//             right: right,
//             double: double,
//         }



#[cfg(all(feature = "nightly", test))]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    #[ignore]
    fn bench_add_two(b: &mut Bencher) {
        // let n = test::black_box(100_000);
        let mut credits_maps = init_credits();
        let calc_queue = init_calc_queue(100_000);

        b.iter(|| {
            calc_queue.iter().for_each(|(key, amount)| {
                if let Some(credit) = credits_maps.get_mut(&key) {
                    credit.calc(*amount);
                }
            })
        });
    }

    fn init_credits() -> HashMap<u64, TwoWayLimit> {
        let mut credits = HashMap::new();
    
        for i in 1..=MAX_ENTITY_COUNT {
            for j in 1..=MAX_ENTITY_COUNT {
                let key = if i > j {
                    i * ENTITY_SQNO_MOD + j
                } else if j > i {
                    j * ENTITY_SQNO_MOD + i
                } else {
                    continue;
                };
                credits.insert(key, TwoWayLimit::new());
            }
        }
    
        return credits;
    }
    
    fn init_calc_queue(count: u64) -> Vec<(u64, u64)> {
        let mut queue = Vec::with_capacity(count.try_into().unwrap());
        let mut rng = thread_rng();
    
        for _ in 0..count {
            let i = rng.gen_range(1, MAX_ENTITY_COUNT);
            let j = rng.gen_range(1, MAX_ENTITY_COUNT);
            let key = if i > j {
                i * ENTITY_SQNO_MOD + j
            } else if j > i {
                j * ENTITY_SQNO_MOD + i
            } else {
                (i + 1) * ENTITY_SQNO_MOD + j
            };
    
            let amount = rng.gen_range(1, 101) * 1000;
            queue.push((key, amount));
        }
    
        queue
    }
}
