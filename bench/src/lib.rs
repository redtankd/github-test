#![cfg_attr(all(feature = "nightly", test), feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

use std::collections::HashMap;
use std::error::Error;

use failure::Fail;

type LimitAmount = u64;

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "An error occurred.")]
pub struct LimitError {
    kind: LimitErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum LimitErrorKind {
    LimitUnavailable,
    WrongEntity,
}

impl LimitError {
    pub fn new(kind: LimitErrorKind) -> Self {
        LimitError { kind: kind }
    }
}

pub struct TwoWayLimit {
    left: LimitAmount,
    right: LimitAmount,
    double: LimitAmount,
}

impl TwoWayLimit {
    fn new(left: LimitAmount, right: LimitAmount) -> TwoWayLimit {
        TwoWayLimit {
            left: left,
            right: right,
            double: left.min(right),
        }
    }

    pub fn try_deduct(&mut self, amount: LimitAmount) -> Result<(), Box<dyn Error>> {
        if self.double >= amount {
            self.left -= amount;
            self.right -= amount;
            self.double -= amount;
            return Ok(());
        } else {
            return Err(Box::new(
                LimitError::new(LimitErrorKind::LimitUnavailable).compat(),
            ));
        }
    }

    pub fn try_deduct_available(
        &mut self,
        amount: LimitAmount,
    ) -> Result<LimitAmount, Box<dyn std::error::Error>> {
        let available = amount.min(self.double);
        if available == 0 {
            return Err(Box::new(
                LimitError::new(LimitErrorKind::LimitUnavailable).compat(),
            ));
        }
        self.left -= available;
        self.right -= available;
        self.double -= available;
        return Ok(available);
    }
}

impl Default for TwoWayLimit {
    fn default() -> TwoWayLimit {
        TwoWayLimit::new(0, 0)
    }
}

pub struct LimitManager {
    limits: HashMap<usize, TwoWayLimit>,
    shift: usize,
}

impl LimitManager {
    pub fn new(capacity: usize, shift: usize) -> Self {
        LimitManager {
            limits: HashMap::with_capacity(capacity),
            shift: shift,
        }
    }

    pub fn insert(
        &mut self,
        left_entity: usize,
        left_amount: LimitAmount,
        right_entity: usize,
        right_amount: LimitAmount,
    ) -> Result<(), Box<dyn Error>> {
        if left_entity >= right_entity {
            return Err(Box::new(
                LimitError::new(LimitErrorKind::WrongEntity).compat(),
            ));
        }
        let limit = TwoWayLimit::new(left_amount, right_amount);
        self.limits
            .insert(left_entity * self.shift + right_entity, limit);
        Ok(())
    }

    pub fn deduct(
        &mut self,
        left_entity: usize,
        right_entity: usize,
        amount: LimitAmount,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(limit) = self
            .limits
            .get_mut(&(left_entity * self.shift + right_entity))
        {
            return limit.try_deduct(amount);
        } else {
            return Err(Box::new(
                LimitError::new(LimitErrorKind::WrongEntity).compat(),
            ));
        }
    }
}

#[cfg(test)]
mod test_mod {
    use super::*;

    #[test]
    fn test_try_deduct() {
        let mut limit = TwoWayLimit::new(10000, 10000);

        assert_eq!(10000, limit.double);
        assert!(limit.try_deduct(10001).is_err());

        assert_eq!((), limit.try_deduct(10000).unwrap());
        assert_eq!(0, limit.double);
        assert_eq!(0, limit.left);
        assert_eq!(0, limit.right);

        let mut limit = TwoWayLimit::new(5000, 10000);

        assert_eq!(5000, limit.double);
        assert!(limit.try_deduct(5001).is_err());

        assert_eq!((), limit.try_deduct(3000).unwrap());
        assert_eq!(2000, limit.double);
        assert_eq!(2000, limit.left);
        assert_eq!(7000, limit.right);
    }

    #[test]
    fn test_try_deduct_available() {
        let mut limit = TwoWayLimit::new(10000, 10000);

        assert_eq!(10000, limit.double);
        assert_eq!(10000, limit.try_deduct_available(10001).unwrap());
        assert_eq!(0, limit.double);
        assert_eq!(0, limit.left);
        assert_eq!(0, limit.right);

        let mut limit = TwoWayLimit::new(10000, 10000);

        assert_eq!(10000, limit.try_deduct_available(10000).unwrap());
        assert_eq!(0, limit.double);
        assert_eq!(0, limit.left);
        assert_eq!(0, limit.right);

        let mut limit = TwoWayLimit::new(5000, 10000);

        assert_eq!(3000, limit.try_deduct_available(3000).unwrap());
        assert_eq!(2000, limit.double);
        assert_eq!(2000, limit.left);
        assert_eq!(7000, limit.right);

        assert_eq!(2000, limit.try_deduct_available(8000).unwrap());
        assert_eq!(0, limit.double);
        assert_eq!(0, limit.left);
        assert_eq!(5000, limit.right);
    }

    #[test]
    fn test_limit_manager() {
        let mut limit_manager = LimitManager::new(10, 100_000);
        assert!(limit_manager.insert(1000, 10000, 100, 1000).is_err());

        assert!(limit_manager.insert(1000, 5000, 2000, 10000).is_ok());
        assert!(limit_manager.insert(1000, 50000, 3000, 10000).is_ok());

        assert!(limit_manager.deduct(1000, 1000, 3000).is_err());
        assert!(limit_manager.deduct(1000, 2000, 3000).is_ok());
        assert!(limit_manager.deduct(1000, 2000, 3000).is_err());
    }
}

#[cfg(all(feature = "nightly", test))]
mod bench {
    use super::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    #[bench]
    fn bench_limit_manager(b: &mut Bencher) {
        let entity_count = 10000;
        let shift = 100_000;
        let test_count = 100_000;

        let mut limit_manager = init_limit_manager(entity_count, shift);
        let deduct_queue = init_deduct_queue(test_count, entity_count);

        b.iter(|| {
            deduct_queue.iter().for_each(|(left, right, amount)| {
                let _ = limit_manager.deduct(*left, *right, *amount);
            })
        });
    }

    fn init_limit_manager(entity_count: usize, entity_sqno_shift: usize) -> LimitManager {
        let mut rng = thread_rng();
        let mut limit_manager = LimitManager::new(entity_count, entity_sqno_shift);

        for i in 0..entity_count {
            for j in (i + 1)..entity_count {
                let left_amount = rng.gen_range(1, 101) * 1000;
                let right_amount = rng.gen_range(1, 101) * 1000;
                let _ = limit_manager.insert(i, left_amount, j, right_amount);
            }
        }

        return limit_manager;
    }

    fn init_deduct_queue(
        queue_size: usize,
        entity_count: usize,
    ) -> Vec<(usize, usize, LimitAmount)> {
        let mut queue = Vec::with_capacity(queue_size);
        let mut rng = thread_rng();

        for _ in 0..queue_size {
            let mut i = rng.gen_range(0, entity_count);
            let j;
            if i + 1 == entity_count {
                j = i;
                i = j - 1;
            } else {
                j = rng.gen_range(i + 1, entity_count);
            }
            let amount = rng.gen_range(1, 201) * 1000;
            queue.push((i, j, amount));
        }

        return queue;
    }
}
