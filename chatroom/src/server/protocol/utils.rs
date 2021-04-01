
use std::path::Path;


// solve duplicate filename problem
pub fn duplicate_filename(filename: &mut String, mut num: usize){
    // println!("filename:{}", filename);
    while Path::new(filename).exists(){
        if filename.find(".") != None {
            num += 1;
            let s:Vec<&str> = filename.as_str().split(".").collect(); 
            // println!("s: {:?}", s);
            let file = s[0];
            let extension = s[1];
            let mut file = String::from(file);
            file.push_str(num.to_string().as_str());
            file.push_str(".");
            file.push_str(extension);
            *filename = file;
            // println!("filename: {}", filename);
        }
    }
}