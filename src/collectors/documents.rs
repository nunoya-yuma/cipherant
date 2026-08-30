use log::warn;
use std::fs;

/// Trait for filesystem abstraction (enables mocking in tests)
trait FileSystem {
    fn read_dir(&self, dir: &str) -> Result<DirContents, std::io::Error>;
}

struct StdFileSystem;

impl StdFileSystem {
    fn new() -> Self {
        Self
    }
}

impl FileSystem for StdFileSystem {
    fn read_dir(&self, dir: &str) -> Result<DirContents, std::io::Error> {
        let mut files = Vec::new();
        let mut subdirs = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let full_path = entry.path();
            if entry.file_type()?.is_dir() {
                subdirs.push(full_path.to_string_lossy().to_string());
            } else {
                files.push(full_path.to_string_lossy().to_string());
            }
        }

        Ok(DirContents { files, subdirs })
    }
}

/// Direct children of one directory, split into files and subdirectories.
#[derive(Clone)]
struct DirContents {
    files: Vec<String>,
    subdirs: Vec<String>,
}

fn list_documents_with_fs<C: FileSystem>(
    file_system: &C,
    dir: &str,
) -> Result<Vec<String>, std::io::Error> {
    match file_system.read_dir(dir) {
        Ok(contents) => {
            let mut files_including_subdirs = contents.files;
            for subdir in &contents.subdirs {
                match list_documents_with_fs(file_system, subdir) {
                    Ok(mut files) => files_including_subdirs.append(&mut files),
                    Err(e) => {
                        warn!("Failed to read subdirectory {}: {}", subdir, e);
                    }
                }
            }

            Ok(files_including_subdirs)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                warn!("Documents directory {} not found, returning empty list", dir);
                return Ok(Vec::new());
            }
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    struct MockFileSystem {
        dirs: HashMap<String, DirContents>,
    }

    impl MockFileSystem {
        fn with_dir(mut self, dir: &str, contents: DirContents) -> Self {
            let file_paths = contents
                .files
                .into_iter()
                .map(|file| format!("{}/{}", dir, file))
                .collect::<Vec<String>>();

            let subdir_paths = contents
                .subdirs
                .into_iter()
                .map(|subdir| format!("{}/{}", dir, subdir))
                .collect::<Vec<String>>();

            self.dirs.insert(
                dir.to_string(),
                DirContents {
                    files: file_paths,
                    subdirs: subdir_paths,
                },
            );

            self
        }
    }

    impl MockFileSystem {
        fn new() -> Self {
            Self {
                dirs: HashMap::new(),
            }
        }
    }

    impl FileSystem for MockFileSystem {
        fn read_dir(&self, dir: &str) -> Result<DirContents, std::io::Error> {
            self.dirs.get(dir).cloned().ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Directory not found",
            ))
        }
    }

    #[test]
    fn test_list_documents_missing_dir_returns_empty() {
        let mock_file = MockFileSystem::new();

        let result = list_documents_with_fs(&mock_file, "non_existent_dir");

        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_list_documents_includes_file_paths() {
        let mock_file = MockFileSystem::new().with_dir(
            "test_dir",
            DirContents {
                files: vec!["file1.txt".to_string(), "file2.txt".to_string()],
                subdirs: vec![],
            },
        );

        let result = list_documents_with_fs(&mock_file, "test_dir");

        assert_eq!(
            result.unwrap(),
            vec![
                "test_dir/file1.txt".to_string(),
                "test_dir/file2.txt".to_string()
            ]
        );
    }

    #[test]
    fn test_list_documents_includes_files_in_subdirectories() {
        let mock_file = MockFileSystem::new()
            .with_dir(
                "root",
                DirContents {
                    files: vec!["file1.txt".to_string(), "file2.txt".to_string()],
                    subdirs: vec!["subdir".to_string()],
                },
            )
            .with_dir(
                "root/subdir",
                DirContents {
                    files: vec!["file3.txt".to_string(), "file4.txt".to_string()],
                    subdirs: vec![],
                },
            );

        let result = list_documents_with_fs(&mock_file, "root");

        assert_eq!(
            result.unwrap(),
            vec![
                "root/file1.txt".to_string(),
                "root/file2.txt".to_string(),
                "root/subdir/file3.txt".to_string(),
                "root/subdir/file4.txt".to_string()
            ]
        );
    }

    #[test]
    fn test_list_documents_empty_dir_returns_empty() {
        let mock_file = MockFileSystem::new().with_dir(
            "empty_dir",
            DirContents {
                files: vec![],
                subdirs: vec![],
            },
        );

        let result = list_documents_with_fs(&mock_file, "empty_dir");

        assert_eq!(result.unwrap(), Vec::<String>::new());
    }
}
