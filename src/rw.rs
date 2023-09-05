

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
        .with_data(&[1, 2, 3, 4, 5, 6])
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

fn read_hdf5(filepath: String) {
    let file = File::open(&filepath).unwrap(); // open for reading
    let ds = file.dataset("group/data").unwrap();
}

#[cfg(test)]
mod test {
    use super::write_hdf5;


    #[test]
    fn test_write_h5() {
        write_hdf5();
    }
}