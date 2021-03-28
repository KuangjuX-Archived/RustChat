use std::path::Path;

// solve duplicate filename problem
pub fn duplicate_filename(filename: &mut String, mut num: usize){
    while Path::new(filename).exists(){
        *filename = format!(
            "{}{}",
            filename.as_str(), num
        );
        num += 1;
    }
}