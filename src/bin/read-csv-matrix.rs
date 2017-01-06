extern crate csv;
use std::fs::File;
use std::io::Read;

fn read_csv_matrix(f: &str){
    let mut f = File::open(f).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let mut rdr = csv::Reader::from_string(s).has_headers(true).flexible(true).delimiter(b' ');
    let elements = rdr.decode().collect::<csv::Result<Vec<Vec<f64>>>>().unwrap()
        .into_iter()
        .flat_map(|x|x.into_iter())
        .collect::<Vec<f64>>();
    let dim = rdr.headers().unwrap().iter()
        .map(|x| x.parse::<usize>())
        .filter(|x|x.is_ok()).map(|x| x.unwrap()).collect::<Vec<usize>>();
    assert_eq!(dim.len(), 2);
    assert_eq!(elements.len(), dim[0] * dim[1]);

    println!("{:?}", &dim);
    println!("{:?}", &elements);
}

fn main() {
    read_csv_matrix("./test.csv");
}
