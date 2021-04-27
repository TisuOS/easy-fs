use std::{fs::OpenOptions, io::{Read, Seek, SeekFrom, Write}, path::Path};

pub struct File {
    file : std::fs::File,
}

impl File {
    pub fn create(path : &String)->Self {
        Self {
            file : OpenOptions::new().
                read(true).write(true).create(true).open(path).unwrap()
        }
    }
    pub fn open(path : &String, option : Option)->Self {
        Self {
            file : OpenOptions::new().
                read(option.read()).write(option.write()).open(path).unwrap(),
        }
    }

    pub fn exists(path : &String)->bool {
        Path::new(&path).exists()
    }

    pub fn is_file(path : &String)->bool {
        Path::new(&path).is_file()
    }
}

impl File {
    pub fn size(&mut self)->usize {
        let old = self.file.stream_position().unwrap();
        let rt = self.file.seek(SeekFrom::End(0)).unwrap();
        self.file.seek(SeekFrom::Start(old)).unwrap();
        rt as usize
    }

    pub fn position(&mut self)->usize {
        self.file.stream_position().unwrap() as usize
    }

    pub fn seek(&mut self, pos : SeekFrom)->usize {
        self.file.seek(pos).unwrap() as usize
    }

    pub fn read(&mut self, buf : &mut [u8])->usize {
        let n = self.file.read(buf).unwrap() as usize;
        assert_ne!(n, 0);
        n
    }

    pub fn write(&mut self, buf : &[u8])->usize {
        let n = self.file.write(buf).unwrap() as usize;
        assert_ne!(n, 0);
        n
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Option {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl Option {
    pub fn read(&self)->bool {
        *self == Self::ReadOnly || *self == Self::ReadWrite
    }

    pub fn write(&self)->bool {
        *self == Self::WriteOnly || *self == Self::ReadWrite
    }
}