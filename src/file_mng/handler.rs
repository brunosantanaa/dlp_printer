use std::{fs, io, path::{Path, PathBuf}};
use zip;


pub fn open_gcode(path: &str) -> Option<String>{
    if let Some(bytes) = fs::read(path).ok() {
        if let Some(s) = String::from_utf8(bytes).ok() {
            return Some(s);
        }
    }
    
    None
}

pub fn unzip(fname: &str, outdir: &str) {
    let file = fs::File::open(fname).expect("File not found");
    let mut archive = zip::ZipArchive::new(file).unwrap();

    if PathBuf::from(outdir).exists() {
        fs::remove_dir_all(&outdir).expect("Falied to clean output dir");
    }
    
    fs::create_dir_all(outdir).expect("Failed to create ouput dir");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(outdir).join(path),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}