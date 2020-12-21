use std::{fs, env};

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "layouts/layout.html", escape = false, delimiter = '?')]
struct Layout {
  nested_template: String
}

fn main() {
  let current_dir = env::current_dir()
    .expect("read current direcory");
    
  let mut template_dir = current_dir.clone().to_path_buf();
  template_dir.push("templates");
  
  for entry in fs::read_dir(template_dir).expect("read template directory") {
      let entry = entry.expect("read template file system entry");
      let path = entry.path();
      
      println!("cargo:rerun-if-changed={}", path.to_string_lossy());
      
      let file_name = path.file_name().expect("template must have file name");
      if file_name != "layout.html" {
      }
      
      let content = String::from_utf8(fs::read(path.clone()).expect("reading template")).expect("template UTF-8 check");
      
      let template_with_layout = Layout {
        nested_template: content
      }.render_once().expect("render layout template");
      
      let mut out_path = current_dir.clone().to_path_buf();
      out_path.push("_build");
      out_path.push("templates");
      out_path.push(path.file_name().expect("template must have file name"));
      fs::write(out_path, template_with_layout).expect("write wrapped template");
  }
}