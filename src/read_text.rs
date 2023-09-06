use std::fs;
use glob::glob;
use polars::prelude::*;

pub fn read_single_file(filename: &str) -> Vec<String>{
    let content = fs::read_to_string(filename).expect("read file err");
    let rows = content.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    return rows;
}


pub fn read_text(filenames: &[&str]) -> Vec<Vec<String>>{
    let a = vec![1, 2, 3];
    filenames.iter()
        .flat_map(|filename| read_single_file(*&filename))
        .filter(|row| row.contains("hello"))
        .map(|row| row.split(" ").map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn to_data_frame(data: Vec<Vec<String>>) {

    let res = data.iter().map(|row| row[1].clone()).collect::<Vec<String>>();

     let df = df!(
         "first" => &res
    );

    println!("{:?}", df);

}

#[cfg(test)]
mod test {
    use glob::glob;

    use crate::read_text::{read_text, to_data_frame};

    #[test]
    fn test_read_text() {
        let filenames = ["./tmp_files/f1.txt", "./tmp_files/f2.txt"];
        let filenames2 = glob("./tmp_files/*.txt")
            .expect("no matched file found")
            .map(|p| p.unwrap().to_str().unwrap().to_string())
            .collect::<Vec<String>>();
        println!("{:?}", read_text(&filenames));
        let datas = read_text(&filenames);
        to_data_frame(datas);
    }
}