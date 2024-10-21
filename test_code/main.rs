// getdata library
mod getdata;
use getdata::Dirfile;
mod getdata_bindings; // Declare the bindings module

fn main() {
    // New Code

    let dirfile = Dirfile::new("test_file");

    // Get the number of fields in the dirfile
    let nfields = dirfile.nfields();
    println!("nfields: {:?}", nfields);

    // Get the number of frames in the dirfile
    let total_frames = dirfile.nframes();
    println!("Total frames: {:?}", total_frames);

    // Get samples per frame for "lon"
    let samples_per_frame = dirfile.spf("lon");
    println!("Samples per frame for lon: {:?}", samples_per_frame);

    // Get the type of the field "lon"
    let lon_type = dirfile.field_type("lon");
    println!("lon_type: {:?}", lon_type);

    // Get the data of the field "lon"
    let lon_data = dirfile.get_data("lon");
    // Print the size
    println!("lon_data size: {:?}", lon_data.len());

    // Do the same for the field "test_fsc2"
    let test_fsc2_data = dirfile.get_data("test_fsc2");
    // Print the size
    println!("test_fsc2_data size: {:?}", test_fsc2_data.len());

}
