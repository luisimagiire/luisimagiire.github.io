use std::fs;
use std::io::prelude::*;


fn save_html_file(html: String, file_name: String) {
  let mut file = fs::File::create(file_name).expect("Failed to create file");
  file.write(html.as_bytes()).expect("Failed to write to file");
}

fn build_full_html(inner_html: String, title: String) -> String {
  let html_base = r#"
<html lang=\"en\">
<head>
  <meta charset=\"UTF-8\">
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
  <title>{title}</title>
</head>
<body>
<p>Return to <a href="./index.html">Blog</a></p>
<p>Return to <a href="../index.html">Main</a></p>
      {content}
</body>
</html>
"#;

  let mut ff = String::new();
  html_escape::decode_html_entities_to_string(
    html_base.replace("{content}", inner_html.as_str())
      .replace("{title}", title.as_str())
      .as_str(),
    &mut ff);
  ff
}

fn read_md_files_from_blog_folder() {
  let folder_path = "./blog/"; // Path to your blog folder
  let entries = fs::read_dir(folder_path).expect("Failed to read directory");


  for entry in entries {
    let file_path = entry.expect("Failed to get directory entry").path();
    let str_path = file_path.to_str().unwrap().clone();
    if file_path.is_file() && file_path.extension().unwrap_or_default() == "md" {
      let mut file = fs::File::open(file_path.clone()).expect("Failed to open file");
      let mut contents = String::new();
      file.read_to_string(&mut contents)
        .expect("Failed to read file contents");

      // Now you can parse the contents of the file as Markdown and do whatever you want with it
      println!("File name: {}", str_path);
      let title = str_path.split("/").last().unwrap().replace(".md", "");
      let blog = markdown::to_html(contents.as_str());
      save_html_file(build_full_html(blog, title), str_path.replace("md", "html"));
      // println!("File contents:\n{}", blog);
    }
  }
}

fn main() {
  read_md_files_from_blog_folder();
  println!("Hello World");
}
