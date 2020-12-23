use std::{fs, env};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "layouts/layout.html", escape = false, delimiter = '?')]
struct Layout {
  nested_template: String
}

fn concat_files(paths: Vec<Path>, output: Path) {
  
}

fn build_stylesheet() {
  let mut entries = fs::read_dir("./assets/stylesheets")
    .expect("read from assets/stylesheets")
    .map(|entry| entry.expect("read entry").path())
    .collect::<Vec<_>>();
  
  entries.sort();
  concat_files(entries, Path::new("./_build/stylesheets/app.css"));
}

fn generate_template_files_using_layout() {
  let current_dir = env::current_dir()
    .expect("read current direcory");
  
  let template_dir = current_dir.join("templates");
  
  for entry in fs::read_dir(template_dir).expect("read template directory") {
      let entry = entry.expect("read template file system entry");
      let path = entry.path();
      
      println!("cargo:rerun-if-changed={}", path.to_string_lossy());
            
      let content = String::from_utf8(fs::read(path.clone()).expect("reading template")).expect("template UTF-8 check");
      
      let template_with_layout = Layout {
        nested_template: content
      }.render_once().expect("render layout template");
      
      let out_path = current_dir
        .join("_build").join("templates")
        .join(path.file_name().expect("template must have file name"));
      fs::write(out_path, template_with_layout).expect("write wrapped template");
  }
}

fn main() {
  build_stylesheet();
  generate_template_files_using_layout();
}