#![feature(async_await, await_macro, futures_api, arbitrary_self_types)]

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Waker, Poll};
    use std::time::{Duration, Instant};

    use futures::compat::Future01CompatExt;
    use futures::executor::{block_on, ThreadPoolBuilder};
    use futures::prelude::{FutureExt, TryFutureExt};
    use tokio::timer::Delay;

    use timer::{Guard, Timer};

    struct Myfuture {
        timer: Timer,
        guard: Option<Guard>,
    }

    impl Future for Myfuture {
        type Output = bool;

        fn poll(self: Pin<&mut Self>, waker: &Waker) -> Poll<Self::Output> {
            if self.guard.is_some() {
                println!("\nMyfuture: I am ready");
                return Poll::Ready(true);
            } else {
                println!("\nMyfuture: I am not ready");
                let waker = waker.clone();
                Pin::get_mut(self).guard = Some(self.timer.schedule_with_delay(
                    chrono::Duration::seconds(1),
                    move || {
                        waker.wake();
                    },
                ));
                return Poll::Pending;
            }
        }
    }

    #[test]
    fn executor_current_thread() {
        assert_eq!(
            true,
            block_on(
                async {
                    await!(Myfuture {
                        timer: Timer::new(),
                        guard: None,
                    })
                }
            )
        );
    }

    #[test]
    fn executor_threadpool() {
        let mut pool = ThreadPoolBuilder::new().pool_size(1).create().unwrap();
        assert_eq!(
            true,
            pool.run(Myfuture {
                timer: Timer::new(),
                guard: None,
            })
        );
    }

    #[test]
    fn executor_eventloop() {
        tokio::run(
            Myfuture {
                timer: Timer::new(),
                guard: None,
            }
            .map(|x| {
                assert_eq!(true, x);
                ()
            })
            .unit_error()
            // convert futures 0.3 into 0.1 with TryFuture
            .compat(),
        );
    }

    #[test]
    #[should_panic]
    fn job_on_time() {
        let when = Instant::now() + Duration::from_millis(100);
        assert_eq!(
            1,
            // Tokio 0.1's Future doesn't work in Futures 0.3's executor now.
            block_on(Delay::new(when).compat().map(|x| {
                assert_eq!((), x.unwrap());
                1
            }))
        );
    }
}
//[cfg(test)]
