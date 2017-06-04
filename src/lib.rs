#[macro_use]
extern crate slog;

use std::io;

use slog::{Drain, Record, OwnedKVList};

pub struct Kmsg
{
}

impl Drain for Kmsg
{
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record, values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
