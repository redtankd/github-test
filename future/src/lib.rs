
#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::thread;
    use std::time::{Duration, Instant};

    use futures::executor::{block_on, ThreadPoolBuilder};
    use futures::future::{lazy, FutureExt};
    use futures::task::SpawnExt;
    use tokio::timer::delay;

    use timer::{Guard, Timer};

    struct Myfuture {
        timer: Timer,
        guard: Option<Guard>,
    }

    impl Future for Myfuture {
        type Output = bool;

        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            if self.guard.is_some() {
                println!("\nMyfuture: I am ready");
                return Poll::Ready(true);
            } else {
                println!("\nMyfuture: I am not ready");
                let waker = cx.waker().clone();
                Pin::get_mut(self).guard = Some(self.timer.schedule_with_delay(
                    chrono::Duration::seconds(1),
                    move || {
                        waker.clone().wake();
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
            block_on(async {
                Myfuture {
                    timer: Timer::new(),
                    guard: None,
                }
                .await
            })
        );
    }

    #[test]
    fn executor_threadpool() -> Result<(), Box<dyn std::error::Error>> {
        let mut pool = ThreadPoolBuilder::new().pool_size(2).create()?;

        pool.spawn(lazy(|_| {
            eprintln!("--- a is runing at thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(1000));
            eprintln!("--- a am done!");
        }))?;

        pool.spawn(async {
            eprintln!("--- b is runing at thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(500));
            eprintln!("--- b am done!");
        })?;

        pool.spawn(async {
            eprintln!("--- c is runing at thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(500));
            eprintln!("--- c am done!");
        })?;

        pool.spawn(async {
            eprintln!("--- d is runing at thread {:?}!", thread::current().id());
            thread::sleep(Duration::from_millis(500));
            eprintln!("d am done!");
        })?;

        assert_eq!(
            true,
            pool.run(Myfuture {
                timer: Timer::new(),
                guard: None,
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn executor_eventloop() {
        Myfuture {
            timer: Timer::new(),
            guard: None,
        }
        .map(|x| {
            assert_eq!(true, x);
            ()
        })
        .await;
    }

    #[test]
    #[should_panic]
    fn job_on_time() {
        let when = Instant::now() + Duration::from_millis(100);
        assert_eq!(
            1,
            block_on(delay(when).map(|x| {
                assert_eq!((), x);
                1
            }))
        );
    }
}
//[cfg(test)]
