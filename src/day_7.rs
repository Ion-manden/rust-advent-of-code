pub mod day_7 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::reader::reader::create_reader_from_file;

    #[derive(Clone, Debug)]
    struct ElfFolder {
        name: String,
        parent_folder: Option<String>,
        path: String,
    }

    #[derive(Clone, Debug)]
    struct ElfFile {
        size: usize,
        folder_path: String,
        path: String,
    }

    fn get_parent_folder(path: &String) -> String {
        let last_part_of_path = path.split("/").last().unwrap();

        let mut part_with_slash = "/".to_owned();
        part_with_slash.push_str(last_part_of_path);

        path.replace(&part_with_slash, "")
    }

    #[test]
    fn test_get_parent_folder() {
        let expect = "/aaa/bbb/ccc";
        let got = get_parent_folder(&"/aaa/bbb/ccc/ddd".to_owned());
        assert_eq!(got, expect);
    }

    fn create_folders_and_files_from_reader(
        reader: BufReader<File>,
    ) -> (HashMap<String, ElfFolder>, HashMap<String, ElfFile>) {
        let mut current_folder = ElfFolder {
            name: "/".to_owned(),
            parent_folder: None,
            path: "/".to_owned(),
        };

        let mut folders: HashMap<String, ElfFolder> = HashMap::new();
        let mut files: HashMap<String, ElfFile> = HashMap::new();

        folders.insert(current_folder.path.clone(), current_folder.clone());

        for l in reader.lines() {
            let line = l.unwrap();

            // command
            if line == "$ cd .." {
                let f = folders.get(&current_folder.parent_folder.unwrap()).unwrap();

                current_folder = f.to_owned();
                continue;
            }
            if line.starts_with("$ cd ") {
                let (_, dir) = line.split_at("$ cd ".len());
                let mut path = current_folder.clone().path;
                if path != "/" {
                    path.push_str("/");
                }
                if dir != "/" {
                    path.push_str(dir);
                }

                let f = folders.get(&path);

                match f {
                    Some(f) => {
                        current_folder = f.to_owned();
                    }
                    None => {
                        let folder = ElfFolder {
                            name: dir.to_owned(),
                            parent_folder: Some(current_folder.clone().path),
                            path,
                        };

                        folders.insert(folder.path.clone(), folder.clone());
                        current_folder = folder;
                    }
                }
                continue;
            }

            if line.starts_with("$ ls") {
                // Do i need to do anythin here?
                // -- No
                continue;
            }

            // ls output
            if line.starts_with("dir ") {
                let (_, dir) = line.split_at("dir ".len());
                let mut path = current_folder.clone().path;
                if path != "/" {
                    path.push_str("/");
                }
                path.push_str(dir);

                let folder = ElfFolder {
                    name: dir.to_owned(),
                    parent_folder: Some(current_folder.clone().path),
                    path,
                };

                folders.insert(folder.path.clone(), folder.clone());
                continue;
            }

            // Last posibilty a file outputed from ls
            let (size, file_name) = line.split_once(" ").unwrap();
            let mut path = current_folder.clone().path;
            path.push_str("/");
            path.push_str(file_name);
            let file = ElfFile {
                size: size.parse::<_>().unwrap(),
                folder_path: current_folder.clone().path,
                path,
            };

            files.insert(file.path.clone(), file);
        }

        (folders, files)
    }

    fn get_folder_size(
        path: String,
        folders: &HashMap<String, ElfFolder>,
        files: &HashMap<String, ElfFile>,
    ) -> usize {
        let folder_size_sum: usize = folders
            .iter()
            .map(|(_, f)| f)
            // Repove top level folder
            .filter(|f| f.path != "/")
            // Filter all not sub folders of path
            .filter(|f| f.parent_folder.clone().unwrap() == path)
            .map(|f| get_folder_size(f.to_owned().path, folders, files))
            .sum();

        let file_size_sum: usize = files
            .iter()
            .map(|(_, f)| f)
            .filter(|f| f.folder_path == path)
            .map(|f| f.size)
            .sum();

        folder_size_sum + file_size_sum
    }

    pub fn day_7_part_1(reader: BufReader<File>) -> usize {
        let (folders, files) = create_folders_and_files_from_reader(reader);

        folders
            .iter()
            .map(|(_, f)| f)
            .map(|f| -> (ElfFolder, usize) {
                (
                    f.to_owned(),
                    get_folder_size(f.clone().path, &folders, &files),
                )
            })
            // .inspect(|(f, s)| println!("{}: {}", f.path, s))
            .filter(|(_, s)| s.to_owned() <= 100_000)
            .map(|(_, s)| s)
            .sum::<usize>()
    }

    #[test]
    fn test_day_7_part_1_example() {
        let expect = 95_437;
        let got = day_7_part_1(create_reader_from_file(
            "./src/day_7/example.txt".to_owned(),
        ));
        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_7_part_1_input() {
        let expect = 1_367_870;
        let got = day_7_part_1(create_reader_from_file("./src/day_7/input.txt".to_owned()));
        assert_eq!(got, expect);
    }

    pub fn day_7_part_2(reader: BufReader<File>) -> usize {
        let (folders, files) = create_folders_and_files_from_reader(reader);

        let total_space_used: i32 = get_folder_size(String::from("/"), &folders, &files) as i32;
        let free_space: i32 = 70_000_000 - total_space_used;
        let space_needed: i32 = 30_000_000 - free_space;

        let mut smallest_seen = total_space_used;

        for (_, f) in folders.iter() {
            let size = get_folder_size(f.clone().path, &folders, &files) as i32;

            if size < space_needed || size > smallest_seen {
                continue;
            }

            smallest_seen = size
        }

        smallest_seen as usize
    }

    #[test]
    fn test_day_7_part_2_example() {
        let expect = 24_933_642;

        let got = day_7_part_2(create_reader_from_file(
            "./src/day_7/example.txt".to_owned(),
        ));

        assert_eq!(got, expect);
    }

    #[test]
    fn test_day_7_part_2_input() {
        let expect = 549_173;

        let got = day_7_part_2(create_reader_from_file("./src/day_7/input.txt".to_owned()));

        assert_eq!(got, expect);
    }
}
