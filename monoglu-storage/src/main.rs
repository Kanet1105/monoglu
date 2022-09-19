use bytes::Bytes;
use object_store::{local::LocalFileSystem, path::Path, ObjectStore};
use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader, Read},
    os::windows::fs::MetadataExt,
    path::PathBuf,
    sync::Arc,
};

pub type Exception = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Exception> {
    let storage_path = path_from_vec(vec![
        "D:/",
        "RustProjects",
        "monoglu",
        "monoglu-storage",
        "data",
    ])?;
    let local = LocalFileSystem::new_with_prefix(&storage_path.as_path())?;
    let storage: Arc<dyn ObjectStore> = Arc::new(local);

    let file_path = path_from_vec(vec!["D:/", "downloads", "mongosh-1.5.4-x64.msi"])?;
    upload(&file_path, storage.clone(), &storage_path).await?;
    Ok(())
}

fn path_from_vec(v: Vec<&str>) -> Result<PathBuf, Exception> {
    let mut path = current_dir()?;
    for crumb in v {
        path.push(crumb);
    }
    Ok(path)
}

async fn upload(
    file_path: &PathBuf,
    storage: Arc<dyn ObjectStore>,
    storage_path: &PathBuf,
) -> Result<(), Exception> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.file_size();

    let index_len: u64 = file_size / (1024 * 255);
    dbg!(index_len + 1);
    let mut buf_reader = BufReader::with_capacity(1024 * 255, file);

    for suffix in 0..(index_len + 1) as usize {
        let file_name = Path::from(suffix.to_string());
        dbg!(&file_name);

        let mut buffer = Vec::with_capacity(1024 * 255);
        buf_reader.read(&mut buffer)?;
        let bytes = Bytes::from(buffer);
        storage.put(&file_name, bytes).await?;
    }
    Ok(())
}

// // Sled usage.
// fn sled_usage() -> Result<(), Exception> {
//     use sled::{Db, IVec};

//     let storage_path = path_from_vec(
//         vec!["D:/", "RustProjects", "monoglu", "monoglu-storage", "data"]
//     )?;
//     let db = sled::open(storage_path)?;

//     let file_path = path_from_vec(
//         // vec!["D:/", "downloads", "All_Amazon_Review.json.gz"]
//         vec!["D:/", "downloads", "mongosh-1.5.4-x64.msi"]
//     )?;
//     upload_file_bytes(file_path, db.clone())?;
//     Ok(())
// }

// fn upload_file_bytes(file_path: PathBuf, db: Db) -> Result<(), Exception> {
//     let file = File::open(&file_path)?;
//     let metadata = file.metadata()?;
//     let file_size = metadata.file_size();

//     let index_len: u64 = file_size / (1024 * 255);
//     dbg!(index_len + 1);
//     let mut buf_reader = BufReader::with_capacity(1024 * 255, file);

//     for index in 0..(index_len + 1) as usize {
//         let key = format!("{}-{}", &file_path.to_str().unwrap(), index);
//         dbg!(&key);
//         let mut buffer = Vec::with_capacity(1024 * 255);
//         buf_reader.read(&mut buffer)?;

//         db.insert(IVec::from(key.as_bytes()), IVec::from(buffer))?;
//     }

//     for index in 0..(index_len + 1) as usize {
//         let key = format!("{}-{}", &file_path.to_str().unwrap(), index);
//         let bytes = db.get(key.as_bytes())?.unwrap();
//         dbg!(bytes);
//     }
//     Ok(())
// }
