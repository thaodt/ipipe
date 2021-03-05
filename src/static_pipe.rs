use crate::{Pipe, OnCleanup};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, MutexGuard};
use std::io::Write;
use dashmap::{DashMap, ReadOnlyView};
use std::path::PathBuf;

lazy_static! 
{
    static ref PIPES: DashMap<String, Mutex<Pipe>> = DashMap::new();
}

#[macro_export]
macro_rules! pprint 
{
    ($name:tt, $($arg:tt)*) => (crate::static_pipe::print_impl($name, format!($($arg)*).as_str()));
}

#[macro_export]
macro_rules! pprintln 
{
    ($name:tt) => (print_impl($name, "\n"));
    ($name:tt, $($arg:tt)*) => ({ crate::static_pipe::print_impl($name, format!($($arg)*).as_str()); })
}

pub fn init(name: &str) -> crate::Result<()>
{
    PIPES.insert(name.to_string(), Mutex::from(Pipe::with_name(name, OnCleanup::Delete)?));
    Ok(())
}

pub fn reader(name: &str) -> Option<Pipe>
{
    PIPES.get(name).map(|pipe| pipe.lock().unwrap().clone())
}

#[inline]
pub(super) fn print_impl(name: &str, s: &str)
{
    match PIPES.get(name)
    {
        None => panic!("Pipe not initialized"),
        Some(pipe) => match pipe.lock().as_mut().unwrap().write_string(s)
        {
            Ok(_) => {}
            Err(e) => panic!(e.to_string())
        }
    }
}
