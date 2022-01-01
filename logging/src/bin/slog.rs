use slog::{self, info, o, warn, Drain};

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain_full = slog_term::FullFormat::new(decorator)
        .use_file_location()
        .use_original_order()
        .build()
        .fuse();
    let drain_full = slog_async::Async::new(drain_full).build().fuse();
    let root = slog::Logger::root(drain_full, o!("key1" => "value1", "key2" => "value2"));
    info!(root, "test info log {}", "key1"; "log-key" => true);
    warn!(root, "a warning!");

    std::thread::sleep(std::time::Duration::from_millis(10));
    println!("");

    // plain text, no color
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain_compact = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain_compact = slog_async::Async::new(drain_compact).build().fuse();
    let root = slog::Logger::root(drain_compact, o!("key1" => "value1", "key2" => "value2"));
    info!(root, "test info log {}", "key1"; "log-key" => true);
    warn!(root, "a warning, no color!");
}
