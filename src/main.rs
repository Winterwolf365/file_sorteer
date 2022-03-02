extern crate date_time;
use date_time::date_tuple::DateTuple;
use std::{
    env::current_dir,
    fs,
    io::{self, Read},
    path::Path,
    time::SystemTime,
};

fn main() {
    println!("--------------------------");
    println!("Welcome to the File Sorter");
    println!("Made by WinterWolf365");
    println!("--------------------------");
    println!("If you didn't already, close the program and");
    println!("the program andthe folder in the folder that you want to sort.");
    println!("Press 'Enter' to continue");
    println!("--------------------------");

    // wait untail the user presses enter
    let mut input = [0];
    io::stdin()
        .read(&mut input)
        .expect("Coulden't read the input");

    // get the current path
    let current_dir = current_dir().expect("Coulden't find the current directory");
    let read_dir = fs::read_dir(&Path::new(&current_dir)).expect("Coulden't read the path");

    // read every file in the path
    for file in read_dir {
        // get the file details
        let file = file.expect("Coulden't read the file");
        let date = file.metadata().unwrap().modified().unwrap();
        let file_name = String::from(file.file_name().to_str().unwrap());

        // check if the name is valid
        if !file_name.contains("desktop.ini")
            && !file_name.contains("file_sorter.exe")
            && file_name.contains(".")
        {
            // get the date
            let duration = date.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let mut date = DateTuple::new(1970, 01, 01).unwrap();
            date.add_days((duration.as_secs_f64() / 3600. / 24.) as u32);

            // make a string with the date
            let date = format!(
                "{:02}-{:02}-{:04}",
                date.get_date(),
                date.get_month(),
                date.get_year(),
            );

            // create folder with the date
            if !Path::new(&date).exists() {
                fs::create_dir(&date).expect("coulden't make path");
            }

            // move the file
            let path = format!("{}/{}", &date, file.file_name().to_string_lossy());
            fs::copy(file.path(), Path::new(&path)).expect("Coulden't copy the file");
            fs::remove_file(file.path()).expect("Coulden't remove file");
        }
    }
}
