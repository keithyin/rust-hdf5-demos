

use hdf5::{File, H5Type, Result};
use ndarray::{arr2, s};

pub fn write_hdf5() -> String {
    let filepath = "test.h5";
    let file = File::create(filepath).unwrap(); // open for writing
    let group = file.create_group("group").unwrap(); // create a group
    // #[cfg(feature = "blosc")]
    // blosc_set_nthreads(2); // set number of blosc threads
    let builder = group.new_dataset_builder();
    // #[cfg(feature = "blosc")]
    // let builder = builder.blosc_zstd(9, true); // zstd + shuffle
    let ds = builder
        .with_data(&[1, 2, 3])
        // finalize and write the dataset
        .create("data").unwrap();
    // create an attr with fixed shape but don't write the data
    let attr = ds.new_attr::<u8>().create("size").unwrap();
    // write the attr data
    attr.write_scalar(&8).unwrap();
    // attr.write(&[8]).unwrap();
    // attr.write(&[8]).unwrap();
    filepath.to_string()
}

pub fn read_hdf5(filepath: String) {
    let file = File::open(&filepath).unwrap(); // open for reading
    let ds = file.dataset("group/data").unwrap();
    
    let data = ds.read_1d::<u8>().unwrap();
    println!("{:?}", data);
}


pub fn modify_h5_dataset(filepath: String) {
    let file = File::open_rw(&filepath).unwrap(); // open for reading
    let ds = file.dataset("group/data").unwrap();
    
    let data = ds.read_1d::<i32>().unwrap();
    // ds.as_writer().conversion(hdf5::Conversion::Hard);
    // ds.as_datatype().unwrap().conv_to::<i32>().expect("dtype conv to error");
    
    ds.as_writer().conversion(hdf5::Conversion::Hard).write(&[4.0, 5.0, 6.0]).expect("write error");
    // ds.as_writer().write_slice(arr, selection)

    // println!("{:?}", data);
}


#[cfg(test)]
mod test {
    use super::{write_hdf5, read_hdf5, modify_h5_dataset};


    #[test]
    fn test_write_h5() {
        let filepath = write_hdf5();
        read_hdf5(filepath.clone());
        // modify_h5_dataset(filepath.clone());
    }
}