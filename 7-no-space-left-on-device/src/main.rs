use anyhow::{anyhow, Result};
use std::io::{stdin, BufRead, BufReader};

struct FileSystem(Vec<FileSystemNode>);

#[derive(Debug)]
enum FileSystemNode {
    File(String, usize),
    Dir(String, Vec<usize>, usize),
}

impl FileSystemNode {
    fn is_dir(&self) -> bool {
        matches!(self, FileSystemNode::Dir(_, _, _))
    }

    fn contents(&self) -> Option<&Vec<usize>> {
        if let FileSystemNode::Dir(_, contents, _) = self {
            Some(contents)
        } else {
            None
        }
    }

    fn contents_mut(&mut self) -> Option<&mut Vec<usize>> {
        if let FileSystemNode::Dir(_, contents, _) = self {
            Some(contents)
        } else {
            None
        }
    }
}

enum Command<'line> {
    ChangeDir(&'line str),
    List,
}

fn read_command(line: &str) -> Option<Command> {
    use Command::*;

    if line.starts_with("cd") {
        Some(ChangeDir(&line[3..]))
    } else if line == "ls" {
        Some(List)
    } else {
        None
    }
}

struct FileSystemBuilder {
    fs: FileSystem,
    cwd: usize,
}

impl FileSystemBuilder {
    fn new() -> Self {
        use FileSystemNode::*;

        Self {
            fs: FileSystem(vec![Dir("".into(), vec![], 0)]),
            cwd: 0,
        }
    }

    fn cwd_dir(&self) -> Result<&FileSystemNode> {
        self.fs
            .0
            .get(self.cwd)
            .ok_or_else(|| anyhow!("cwd is wrong, invalid index"))
            .and_then(|node| {
                node.is_dir()
                    .then_some(node)
                    .ok_or_else(|| anyhow!("cwd is wrong, not a directory"))
            })
    }

    fn cwd_dir_mut(&mut self) -> Result<&mut FileSystemNode> {
        self.fs
            .0
            .get_mut(self.cwd)
            .ok_or_else(|| anyhow!("cwd is wrong, invalid index"))
            .and_then(|node| {
                node.is_dir()
                    .then_some(node)
                    .ok_or_else(|| anyhow!("cwd is wrong, not a directory"))
            })
    }

    fn cwd_parent(&self) -> Result<usize> {
        if let FileSystemNode::Dir(_, _, parent) = self.cwd_dir()? {
            Ok(*parent)
        } else {
            panic!("cwd_dir logic is wrong and it returned a non-dir node");
        }
    }

    fn find_or_create_dir(&mut self, inner: &str) -> Result<usize> {
        match self.find_dir(inner)? {
            Some(inode) => Ok(inode),
            None => {
                let new_inode = self.fs.0.len();
                self.fs
                    .0
                    .push(FileSystemNode::Dir(inner.to_owned(), vec![], self.cwd));
                self.cwd_dir_mut()?
                    .contents_mut()
                    .into_iter()
                    .for_each(|contents| contents.push(new_inode));
                Ok(new_inode)
            }
        }
    }

    fn change_dir(&mut self, target: &str) -> Result<()> {
        match target {
            ".." => {
                self.cwd = self.cwd_parent()?;
            }
            "/" => {
                self.cwd = 0;
            }
            inner => {
                self.cwd = self.find_or_create_dir(inner)?;
            }
        }

        Ok(())
    }

    fn find_dir(&self, name: &str) -> Result<Option<usize>> {
        Ok(self
            .cwd_dir()?
            .contents()
            .into_iter()
            .flat_map(|contents| {
                contents
                    .iter()
                    .copied()
                    .find(|inode| match &self.fs.0[*inode] {
                        FileSystemNode::Dir(dname, ..) => dname == name,
                        _ => false,
                    })
            })
            .next())
    }

    fn find_file(&self, name: &str) -> Result<Option<usize>> {
        Ok(self
            .cwd_dir()?
            .contents()
            .into_iter()
            .flat_map(|contents| {
                contents
                    .iter()
                    .copied()
                    .find(|inode| match &self.fs.0[*inode] {
                        FileSystemNode::File(fname, ..) => fname == name,
                        _ => false,
                    })
            })
            .next())
    }

    fn append_file(&mut self, name: String, size: usize) -> Result<usize> {
        match self.find_file(&name)? {
            Some(inode) => Ok(inode),
            None => {
                let new_inode = self.fs.0.len();
                self.fs.0.push(FileSystemNode::File(name, size));
                self.cwd_dir_mut()?
                    .contents_mut()
                    .into_iter()
                    .for_each(|contents| contents.push(new_inode));
                Ok(new_inode)
            }
        }
    }

    fn build(self) -> FileSystem {
        self.fs
    }
}

fn read(reader: impl BufRead) -> Result<FileSystem> {
    let mut fs = FileSystemBuilder::new();

    let mut is_listing = false;
    for line in reader.lines() {
        let line = line?;

        if line.starts_with('$') {
            is_listing = false;
            let command = read_command(&line[2..]).ok_or_else(|| anyhow!("wrong input"))?;

            use Command::*;
            match command {
                ChangeDir(target) => {
                    fs.change_dir(target)?;
                }
                List => {
                    is_listing = true;
                }
            }
        } else if is_listing {
            if line.starts_with("dir") {
                fs.find_or_create_dir(&line[4..])?;
            } else {
                let mut parts = line.split_whitespace();
                let size = parts
                    .next()
                    .ok_or_else(|| anyhow!("invalid input - wrong file format"))
                    .and_then(|size| size.parse::<usize>().map_err(Into::into))?;
                let name = parts
                    .next()
                    .ok_or_else(|| anyhow!("invalid input - wrong file format (name not found)"))?
                    .to_owned();
                fs.append_file(name, size)?;
            }
        } else {
            return Err(anyhow!(
                "wrong input - not listing, but input starts without $"
            ));
        }
    }

    Ok(fs.build())
}

fn dir_sizes(fs: &FileSystem) -> Vec<usize> {
    let mut stack = vec![(0, 0, 0)];
    let mut result = vec![];
    let mut last_parent = usize::MAX;

    while !stack.is_empty() {
        let idx = stack.len() - 1;
        let (current, _, parent_idx) = stack.last().copied().unwrap();

        if last_parent != current {
            if let Some(contents) = fs.0[current].contents() {
                use FileSystemNode::*;

                for inode in contents.iter().copied() {
                    match &fs.0[inode] {
                        File(_, size) => {
                            stack[idx].1 += *size;
                        }
                        Dir(..) => {
                            stack.push((inode, 0, idx));
                        }
                    }
                }
            }
        }

        // We've finished processing the node.
        if stack.last().unwrap().0 == current {
            result.push(stack[idx].1);
            stack[parent_idx].1 += stack[idx].1;
            stack.pop();
            if !stack.is_empty() {
                last_parent = stack[parent_idx].0;
            }
        }
    }

    result
}

fn main() -> Result<()> {
    let fs = read(BufReader::new(stdin()))?;

    let sizes = dir_sizes(&fs);

    let at_most_100000_sum = sizes
        .iter()
        .copied()
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    let total_used = sizes.iter().copied().max().unwrap();
    let minimum_directory = sizes
        .iter()
        .copied()
        .filter(|size| total_used.saturating_sub(*size) <= 40000000)
        .min()
        .unwrap();

    println!(
        "Total sum of directories with more than 100000 bytes is {}",
        at_most_100000_sum,
    );

    println!(
        "Minimum directory that needs to be deleted to free up space has {} bytes",
        minimum_directory
    );

    Ok(())
}
