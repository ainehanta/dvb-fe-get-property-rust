use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// static RE_IOWR: Lazy<Regex> = Lazy::new(|| {
//     Regex::new(r"#define\s+(\w+)\s+(_IO[WR]*)\(([^,)]+),\s*([^,)]+),?\s*([^,)]*)\)").unwrap()
// });

// #[derive(Debug)]
// struct Callbacks(PathBuf);

// impl Callbacks {
//     fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
//         let _ = std::fs::File::create(&path)?;
//         Ok(Callbacks(path.as_ref().to_owned()))
//     }

//     fn generate(&self, filename: &str) -> std::io::Result<()> {
//         let input_file = std::fs::read_to_string(filename)?;

//         let mut output_file = OpenOptions::new().create(true).append(true).open(&self.0)?;

//         for caps in RE_IOWR.captures_iter(&input_file) {
//             let define_name = caps[1].to_ascii_lowercase();
//             let iorw = &caps[2];
//             let io_type = &caps[3];
//             let number = &caps[4];

//             let (macro_name, ty) = match iorw {
//                 "_IO" => ("ioctl_none", None),
//                 "_IOR" => ("ioctl_read", Some(&caps[5])),
//                 "_IOW" => {
//                     let ty = &caps[5];
//                     let macro_name = if ty == "long" {
//                         "ioctl_write_int"
//                     } else {
//                         "ioctl_write_ptr"
//                     };
//                     (macro_name, Some(ty))
//                 }
//                 "_IOWR" => ("ioctl_readwrite", Some(&caps[5])),
//                 _ => continue,
//             };

//             writeln!(output_file, "\n// {}", &caps[0])?;
//             match ty {
//                 Some(ty) if macro_name != "ioctl_write_int" => {
//                     let ty = ty.trim_start_matches("struct ");
//                     let ty = if ty == "int" { "libc::c_int" } else { ty };
//                     writeln!(
//                         output_file,
//                         "nix::{}!({}, {}, {}, {});",
//                         macro_name, define_name, io_type, number, ty
//                     )?;
//                 }
//                 _ => {
//                     writeln!(
//                         output_file,
//                         "nix::{}!({}, {}, {});",
//                         macro_name, define_name, io_type, number
//                     )?;
//                 }
//             }
//         }

//         Ok(())
//     }
// }

// impl bindgen::callbacks::ParseCallbacks for Callbacks {
//     fn include_file(&self, filename: &str) {
//         println!("cargo:rerun-if-changed={}", filename);
//         self.generate(filename).expect("failed");
//     }
// }
