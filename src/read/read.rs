use std::{error::Error, fs, rc::Rc, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn read_dir_no_parallel(read_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let read_path = read_path.chars().rev().skip_while(|c| *c == '/').collect::<String>().chars().rev().collect::<String>();

    let dir = fs::read_dir(&read_path)?;

    let mut vec_name_frames = vec![];

    for file in dir {
        let file = file?;

        let name = file.file_name().to_string_lossy().to_string();
        if name.contains("_frame") {
            if name.split("_frame").collect::<Vec<_>>()[1].split(".").collect::<Vec<_>>()[0]
                .chars()
                .all(|c| c.is_ascii_digit())
            {
                vec_name_frames.push(name);
            }
        }
    }

    let read_path = Rc::new(read_path);

    vec_name_frames.frame_sort();
    let vec_strings = vec_name_frames
        .into_iter()
        .map(|path| fs::read_to_string(format!("{}/{}", Rc::clone(&read_path), path)).unwrap()).collect::<Vec<String>>();

    Ok(vec_strings)
}

pub fn read_dir_parallel(read_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let read_path = read_path.chars().rev().skip_while(|c| *c == '/').collect::<String>().chars().rev().collect::<String>();
    let dir = fs::read_dir(&read_path)?;

    let x = dir.collect::<Vec<_>>();

    let mut vec_name_frames = x.into_par_iter().filter_map(|file| {
        let file = file.unwrap();

        let name = file.file_name().to_string_lossy().to_string();
        if name.contains("_frame") {
            if name.split("_frame").collect::<Vec<_>>()[1].split(".").collect::<Vec<_>>()[0]
                .chars()
                .all(|c| c.is_ascii_digit())
            {
                Some(name)
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<_>>();

    let read_path = Arc::new(read_path);

    vec_name_frames.frame_sort();
    let vec_strings = vec_name_frames
        .into_par_iter()
        .map(|path| fs::read_to_string(format!("{}/{}", Arc::clone(&read_path), path)).unwrap()).collect::<Vec<String>>();

    Ok(vec_strings)
}

trait FrameSort {
    fn frame_sort(&mut self);
}

impl FrameSort for Vec<String> {
    fn frame_sort(&mut self) {
        self.sort_by(|a, b| {
            let a = a.split("_frame").collect::<Vec<_>>()[1].split(".").collect::<Vec<_>>()[0]
                .parse::<u32>()
                .unwrap();
            let b = b.split("_frame").collect::<Vec<_>>()[1].split(".").collect::<Vec<_>>()[0]
                .parse::<u32>()
                .unwrap();

            a.cmp(&b)
        });
    }
}

#[test]
fn test() {
    read_dir_no_parallel("gender-bender-frames".to_string()).unwrap();
}