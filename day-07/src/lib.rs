use std::{
    error::Error,
    path::{Path, PathBuf},
    str::FromStr,
};

use advent_utils::{Part, Solver};
use color_eyre::{eyre::eyre, Report};

type Map<K, V> = fnv::FnvHashMap<K, V>;

#[derive(Debug)]
pub struct Solution {
    entries: Map<PathBuf, FsEntry>,
}

fn parse_command(
    cmd_log: &str,
    current_path: &mut PathBuf,
    fs_entries: &mut Map<PathBuf, FsEntry>,
) -> Result<(), Report> {
    let (cmd_name, rest) = cmd_log
        .split_once(' ')
        .ok_or_else(|| eyre!("invalid cmd format: {cmd_log}"))?;

    match cmd_name {
        "cd" => {
            if rest == ".." {
                current_path.pop();
            } else {
                let dir_name = rest.to_owned();
                current_path.push(dir_name);
                fs_entries.insert(current_path.clone(), FsEntry::Dir(vec![]));
            }
            Ok(())
        }
        "ls" => {
            let current_entry = fs_entries
                .get_mut(current_path)
                .ok_or_else(|| eyre!("listing unknown dir"))?;

            let entries = rest
                .lines()
                .map(|line| {
                    let mut path = current_path.clone();

                    match line.split_once(' ') {
                        Some(("dir", name)) => {
                            let entry = FsEntry::Dir(vec![]);
                            path.push(name);

                            current_entry.push(path.clone())?;
                            Ok((path, entry))
                        }
                        Some((size, name)) => {
                            let entry = FsEntry::File(size.parse()?);
                            path.push(name);

                            current_entry.push(path.clone())?;
                            Ok((path, entry))
                        }
                        None => Err(eyre!("invalid format: {line}")),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            fs_entries.extend(entries);

            Ok(())
        }
        _ => Err(eyre!("unknown cmd: {cmd_name}")),
    }
}

#[derive(Debug, Clone)]
pub enum FsEntry {
    Dir(Vec<PathBuf>),
    File(usize),
}

impl FsEntry {
    fn size(&self, entries_map: &Map<PathBuf, FsEntry>) -> Result<usize, Report> {
        match self {
            Self::File(size) => Ok(*size),
            Self::Dir(entries) => {
                let mut size_sum = 0;

                for entry_path in entries {
                    size_sum += entries_map
                        .get(entry_path)
                        .ok_or_else(|| eyre!("tying to check unknown entry {entry_path:?}"))?
                        .size(entries_map)?;
                }

                Ok(size_sum)
            }
        }
    }

    fn push(&mut self, entry: PathBuf) -> Result<(), Report> {
        match self {
            Self::File(_) => Err(eyre!("trying to push to the file")),
            Self::Dir(entries) => {
                entries.push(entry);

                Ok(())
            }
        }
    }
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = Default::default();
        let mut current_path = PathBuf::new();

        let cmd_logs = s.split("$ ").skip_while(|&s| s.is_empty());
        for cmd_log in cmd_logs {
            parse_command(cmd_log.trim(), &mut current_path, &mut entries)?;
        }

        Ok(Self { entries })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let sizes_sum: usize = self
                    .entries
                    .values()
                    .filter(|entry| matches!(entry, FsEntry::Dir(_)))
                    .filter_map(|entry| {
                        let size = entry.size(&self.entries).unwrap();

                        (size <= 100000).then_some(size)
                    })
                    .sum();

                format!("total file sizes sum is {sizes_sum}")
            }
            Part::Two => {
                const DISK_SIZE: usize = 70_000_000;
                const MUST_BE_FREE_SIZE: usize = 30_000_000;

                let root_size = self
                    .entries
                    .get(Path::new("/"))
                    .expect("no root parsed")
                    .size(&self.entries)
                    .expect("can't get root size");

                let free_size = DISK_SIZE - root_size;
                let need_to_free = match MUST_BE_FREE_SIZE.checked_sub(free_size) {
                    None => return "there is already enough free space :)".to_owned(),
                    Some(need_to_free) => need_to_free,
                };

                let min_dir_to_del_size = self
                    .entries
                    .values()
                    .filter(|entry| matches!(entry, FsEntry::Dir(_)))
                    .filter_map(|entry| {
                        let size = entry.size(&self.entries).unwrap();

                        (size >= need_to_free).then_some(size)
                    })
                    .min()
                    .expect("no suitable dir to delete found :(");

                format!("min dir to delete has size {min_dir_to_del_size}")
            }
        }
    }

    fn day_number() -> u32 {
        7
    }
}
