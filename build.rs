use sailfish::TemplateOnce;
use std::path::{Path, PathBuf};
use std::{
    env, fs,
    io::{self, Write},
};

#[derive(TemplateOnce)]
#[template(path = "layouts/layout.html", escape = false, delimiter = '?')]
struct Layout {
    nested_template: String,
}

fn open_temp_file() -> io::Result<(PathBuf, fs::File)> {
    let mut temp_file_number = 0;
    let tmp_dir = fs::canonicalize(env::temp_dir()).expect("get canonical tmp path");
    loop {
        let path = tmp_dir.join(format!("doxia-tmp-{}", temp_file_number));
        let output_file_result = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path);
        match output_file_result {
            Ok(file) => {
                return Ok((path, file));
            }
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                temp_file_number += 1;
                continue;
            }
            Err(other_err) => {
                return Err(other_err);
            }
        }
    }
    // TODO: check more things? Could the file have been moved?
    // On Linux can use O_TMPFILE / linkat, but not Darwin :<
}

fn concat_files_and_overwrite_if_exists(paths: Vec<PathBuf>, output_path: &Path) {
    let (temp_path, mut temp_file) = open_temp_file().expect("open tmp file for concat");
    for path in paths {
        let mut file = fs::File::open(path).expect("open source file for concat");
        io::copy(&mut file, &mut temp_file).expect("copy to new file");
    }
    temp_file.flush().expect("flush temp file");
    fs::rename(&temp_path, &output_path).expect("rename concat tmp file");
}

fn build_stylesheet() {
    let mut entries = fs::read_dir("./assets/stylesheets")
        .expect("read from assets/stylesheets")
        .map(|entry| entry.map(|e| e.path()))
        .collect::<io::Result<Vec<PathBuf>>>()
        .expect("read stylesheets");

    entries.sort();
    concat_files_and_overwrite_if_exists(entries, Path::new("./_build/assets/app.css"));
}

fn generate_template_files_using_layout() -> io::Result<()> {
    let current_dir = env::current_dir().expect("read current direcory");

    let template_dir = current_dir.join("templates");

    for entry in fs::read_dir(template_dir).expect("read template directory") {
        let entry = entry.expect("read template file system entry");
        let path = entry.path();

        println!("cargo:rerun-if-changed={}", path.to_string_lossy());

        let content = String::from_utf8(fs::read(path.clone()).expect("reading template"))
            .expect("template UTF-8 check");

        let template_with_layout = Layout {
            nested_template: content,
        }
        .render_once()
        .expect("render layout template");

        let out_path = current_dir
            .join("_build")
            .join("templates")
            .join(path.file_name().expect("template must have file name"));
        fs::write(out_path, template_with_layout).expect("write wrapped template");
    }

    Ok(())
}

fn create_build_directories() {
    fs::create_dir_all(Path::new("./_build/assets"))
        .and_then(|_| fs::create_dir_all(Path::new("./_build/templates")))
        .expect("create _build directories");
}

fn main() -> io::Result<()> {
    create_build_directories();
    build_stylesheet();
    generate_template_files_using_layout()?;
    Ok(())
}
