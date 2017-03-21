extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_timer;
extern crate timer;
extern crate chrono;

#[cfg(test)]
mod tests {
    use futures::*;
    use futures_cpupool::*;
    use timer::*;
    use chrono::*;

    #[test]
    fn executor_cpupool() {
        
        struct Myfuture {
            timer: Timer,
            guard: Option<Guard>
        };

        impl Future for Myfuture {
            type Item = bool;
            type Error = ();

            fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                
                if self.guard.is_some() {
                    println!("\nMyfuture: I am ready");
                    return Ok(Async::Ready(true)); 
                } else {
                    println!("\nMyfuture: I am not ready");   
                    let task = task::park();             
                    self.guard = Some(self.timer.schedule_with_delay(Duration::seconds(1), move || {
                        task.unpark();
                    }));
                    return Ok(Async::NotReady);
                }
            }
        }

        let pool = CpuPool::new(1);
        assert_eq!(true, 
            pool.spawn(Myfuture{ timer: Timer::new(), guard: None }).wait().unwrap()
        );
    }

    #[test]
    fn executor_thread() {
        
        struct Myfuture {
            timer: Timer,
            guard: Option<Guard>
        };

        impl Future for Myfuture {
            type Item = bool;
            type Error = ();

            fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                
                if self.guard.is_some() {
                    println!("\nMyfuture: I am ready");
                    return Ok(Async::Ready(true)); 
                } else {
                    println!("\nMyfuture: I am not ready");   
                    let task = task::park();             
                    self.guard = Some(self.timer.schedule_with_delay(Duration::seconds(1), move || {
                        task.unpark();
                    }));
                    return Ok(Async::NotReady);
                }
            }
        }

        
        assert_eq!(true, 
            (Myfuture{ timer: Timer::new(), guard: None }).wait().unwrap()
        );
    }

    #[test]
    fn executor_eventloop() {
        
        struct Myfuture {
            timer: Timer,
            guard: Option<Guard>
        };

        impl Future for Myfuture {
            type Item = ();
            type Error = ();

            fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
                
                if self.guard.is_some() {
                    println!("\nMyfuture: I am ready");
                    return Ok(Async::Ready(())); 
                } else {
                    println!("\nMyfuture: I am not ready");   
                    let task = task::park();             
                    self.guard = Some(self.timer.schedule_with_delay(Duration::seconds(1), move || {
                        task.unpark();
                    }));
                    return Ok(Async::NotReady);
                }
            }
        }

        // Create the event loop
        let mut core = ::tokio_core::reactor::Core::new().unwrap();

        core.handle().spawn(Myfuture{ timer: Timer::new(), guard: None });

        // run the event loop
        core.run(
            ::tokio_timer::Timer::default().sleep(::std::time::Duration::from_millis(1200))
        ).unwrap();
    }
}
//[cfg(test)]