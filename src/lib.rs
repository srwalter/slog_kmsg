#[macro_use]
extern crate slog;
extern crate slog_async;

use std::{fs, io};
use std::cell::RefCell;
use std::io::Write as IoWrite;

use slog::{Drain, Record, OwnedKVList};

const BUFFER_SIZE: usize = 1024 - 32;

pub struct Kmsg {
    fd: RefCell<fs::File>,
    buffer: RefCell<[u8; BUFFER_SIZE]>,
}

impl Kmsg {
    pub fn new() -> Result<Kmsg, io::Error> {
        let kmsg = fs::OpenOptions::new().write(true).open("/dev/kmsg")?;

        Ok(Kmsg {
               fd: RefCell::new(kmsg),
               buffer: RefCell::new([0; BUFFER_SIZE]),
           })
    }
}

fn level_to_kern_level(l: slog::Level) -> u8 {
    match l {
        slog::Level::Critical => 2,
        slog::Level::Error => 3,
        slog::Level::Warning => 4,
        slog::Level::Info => 6,
        slog::Level::Debug => 7,
        slog::Level::Trace => 7,
    }
}

impl Drain for Kmsg {
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let len = {
            let mut buf = self.buffer.borrow_mut();
            let mut cursor = io::Cursor::new(&mut buf[..]);
            let klevel = level_to_kern_level(record.level());
            write!(&mut cursor,
                   "<{}>{}: {}",
                   klevel,
                   record.module(),
                   record.msg())?;
            cursor.position() as usize
        };
        self.fd
            .borrow_mut()
            .write_all(&self.buffer.borrow()[..len])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Kmsg;

    use slog;
    use slog::Drain;

    use slog_async;

    #[test]
    fn it_works() {
        let drain = Kmsg::new().unwrap().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let root = slog::Logger::root(drain, o!());
        error!(root, "test 123");
    }
}
