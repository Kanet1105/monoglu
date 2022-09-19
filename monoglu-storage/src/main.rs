use std::{
    env::current_dir,
    fs::File,
    io::{BufReader, Read},
    os::windows::fs::MetadataExt,
    path::PathBuf,
};

pub type Exception = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Exception> {
    let storage_path = path_from_vec(
        vec!["D:/", "RustProjects", "monoglu", "monoglu-storage", "data"]
    )?;
    let tree = sled::open(storage_path)?;

    let file_path = path_from_vec(
        // vec!["D:/", "downloads", "All_Amazon_Review.json.gz"]
        vec!["D:/", "downloads", "mongosh-1.5.4-x64.msi"]
    )?;
    read_file_bytes(file_path)?;
    Ok(())
}

fn path_from_vec(v: Vec<&str>) -> Result<PathBuf, Exception> {
    let mut path = current_dir()?;
    for crumb in v {
        path.push(crumb);
    }
    Ok(path)
}

fn read_file_bytes(file_path: PathBuf) -> Result<(), Exception> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.file_size();
    
    let index_len: u64 = file_size / (1024 * 255);
    dbg!(index_len + 1);
    let mut buf_reader = BufReader::with_capacity(1024 * 255, file);

    for index in 0..(index_len + 1) as usize {
        dbg!(index);
        let mut buffer = [0; 1024 * 255];
        buf_reader.read(&mut buffer)?;
    }
    Ok(())
}

fn write_bytes(storage_path: PathBuf) {
    
}

// pub struct Object {
//     capacity: usize,
// }

// impl Object {
//     fn new() -> Self {
//         Self { capacity: 255 * 1024 }
//     }

//     fn read<T: Read>(&self, reader: &mut T) {
//         let mut buffer = Vec::with_capacity(255 * 1024);
//         let part = reader.take(self.capacity as u64);
//     }
// }
