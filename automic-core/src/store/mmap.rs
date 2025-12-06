use std::{fs::OpenOptions, path::PathBuf};

use memmap2::MmapMut;
use rand::RngCore;

use crate::store::Store;

pub struct MmapVec<'a, V> {
    buff: &'a mut [V],
    _mmap: MmapMut,
}

impl<'a, V: Clone> Store<V> for MmapVec<'a, V> {
    fn fill(size: usize, fill: V) -> Option<Self> {
        let mem_size = size_of::<V>() * size;
        let mut name = [0u8; 16];
        rand::rng().fill_bytes(&mut name);
        // Path to backing store
        let path: PathBuf = ["/var/tmp", &hex::encode(name)].iter().collect();
        if let Ok(file) = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
        {
            file.set_len(mem_size as u64)
                .expect("Could not set file length");
            if let Ok(mut mmap) = unsafe { MmapMut::map_mut(&file) } {
                let ptr = mmap.as_mut_ptr() as *mut V;
                let buff = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
                for i in 0..size {
                    buff[i] = fill.clone();
                }
                Some(Self { buff, _mmap: mmap })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get(&mut self, position: usize) -> &V {
        &self.buff[position]
    }

    fn get_mut(&mut self, position: usize) -> &mut V {
        &mut self.buff[position]
    }
}
