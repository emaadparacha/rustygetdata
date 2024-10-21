# RustyGetData
GetData - But the Rust version

---------------------

## Introduction
`rustygetdata` is a Rust Wrapper for the GetData Library - used to access, read, write, and work with Dirfiles. You can find more information on [GetData here](https://getdata.sourceforge.net/getdata.html), along with links to other bindings and wrappers. `rustygetdata` takes away the pain of C and instead allows you to use GetData in Rust!

### 🚧 CONSTRUCTION WARNING 🚧 
This library and wrapper is still under construction, with only a few functions ready. Currently you can only do the following:

* Get the number of fields in a Dirfile
* Get the number of frames in a Dirfile
* Get the number of samples per frame for a field in the Dirfile
* Get the data type of the data for a field in the Dirfile
* Get all the data for a field in the Dirfile (as a `Vec<f64>`)

More functions will be added soon as they are written and tested, especially around writing to a Dirfile, getting specific chunks of data, and more. If you have a specific request, please reach out. Or if you'd like to contribute, more on that below!

## 👨‍💻 Usage
To use `rustygetdata`, add it to your `Cargo.toml` file under `dependencies`:
```
[dependencies]
rustygetdata = "0.1.0" # Replace with the latest version
```

Then, in your Rust code, use it as following:
```
use rustygetdata::Dirfile;

fn main() {
    let my_dirfile = Dirfile::new("path/to/your/dirfile");

    println!("Number of fields: {}", my_dirfile.nfields());
    println!("Number of frames: {}", my_dirfile.nframes());
    
    let data = my_dirfile.get_data("your_field");
    println!("Data: {:?}", data);
}
```

## 🤔 Future Work
Like mentioned above, this is still under construction and more functions will be added soon, especially around writing to Dirfiles, getting specific chunks of data, and a lot more that the full library has to offer. Stay tuned for more updates to the library!

## 🐠 Contributing
If you want to join in on the fun and help contribute, or if you have any suggestions you'd like to see in `rustygetdata`, please feel free to reach out, report and create issues, or if you prefer, fork this repository and submit pull requests. Contributions are always appreciated!

## 🤝 License
Like the [GetData](https://github.com/ketiltrout/getdata) Project, this project is licensed under the GNU Lesser General Public License (LGPL-2.1). This means you can use, modify, and distribute the library, but if you distribute it or modified versions of it, you must also provide access to the source code.

## 😄 Enjoy!
Feel free to reach out if you have any questions, requests, want to chat, or more!