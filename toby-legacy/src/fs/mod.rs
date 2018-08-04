use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{self, Seek, SeekFrom};
use std::path::{Path, PathBuf};

mod flock;

use self::flock::FileLock;

const LOG_PATH: &str = env!("TOBY_LOG_PATH");
const RUNTIME_PATH: &str = env!("TOBY_RUNTIME_PATH");

fn ensure_parent(path: &Path) -> io::Result<()> {
    // ensure that directory exists
    if let Some(dir) = path.parent() {
        DirBuilder::new().recursive(true).create(dir)?;
    }

    Ok(())
}

fn get_job_id_path(project_name: &str) -> PathBuf {
    let mut path = PathBuf::from(RUNTIME_PATH);

    path.push("jobs");
    path.push(project_name);
    path.push("next_id");

    path
}

pub fn job_log_path(project_name: &str, job_id: u64) -> PathBuf {
    let mut path = PathBuf::from(LOG_PATH);

    path.push("jobs");
    path.push(format!("{}-{}", project_name, job_id));

    path.set_extension("log");

    path
}

fn job_archive_path(project_name: &str, job_id: u64) -> PathBuf {
    let mut path = PathBuf::from(RUNTIME_PATH);

    path.push("jobs");
    path.push(project_name);
    path.push(job_id.to_string());

    path.set_extension("toml");

    path
}

pub(crate) fn get_telegram_chat_id_path() -> PathBuf {
    let mut path = PathBuf::from(RUNTIME_PATH);

    path.push("telegram_chat_id");

    path
}

pub(crate) fn get_job_archive_file(project_name: &str, job_id: u64) -> io::Result<File> {
    let path = job_archive_path(project_name, job_id);

    ensure_parent(&path)?;

    OpenOptions::new().create(true).write(true).open(path)
}

///
/// Determines and creates the log file for a job.
///
pub(crate) fn get_job_log(project_name: &str, job_id: u64) -> io::Result<File> {
    let path = job_log_path(project_name, job_id);

    ensure_parent(&path)?;

    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
}

///
/// Determines the next job id for a project and increments the counter.
///
pub(crate) fn next_job_id(project_name: &str) -> io::Result<u64> {
    let path = get_job_id_path(project_name);

    // ensure that directory exists
    ensure_parent(&path)?;

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut file = FileLock::exclusive(file)?;

    // read current id (if file is empty fall back to 1)
    let next_id = file.read_u64::<NativeEndian>().unwrap_or(1);

    // move cursor back to beginning
    file.seek(SeekFrom::Start(0))?;

    // write next id
    file.write_u64::<NativeEndian>(next_id + 1)?;

    // make sure file is truncated to a u64
    file.file().set_len(8)?;

    Ok(next_id)
}

pub(crate) fn get_telegram_chat_id() -> io::Result<Option<i64>> {
    let path = get_telegram_chat_id_path();

    if !path.exists() {
        return Ok(None);
    }

    let mut file = File::open(path)?;

    let chat_id = file.read_i64::<NativeEndian>()?;

    Ok(Some(chat_id))
}

pub(crate) fn write_telegram_chat_id(chat_id: i64) -> io::Result<()> {
    let path = get_telegram_chat_id_path();

    ensure_parent(&path)?;

    let mut file = OpenOptions::new().create(true).write(true).open(path)?;

    file.write_i64::<NativeEndian>(chat_id)?;

    Ok(())
}
