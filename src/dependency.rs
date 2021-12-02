use std::io::Write;

pub trait DependencyProvider {
    fn download(&self) -> bool;
}

#[derive(serde::Deserialize, Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub repo: String,
    pub notes: Option<String>,
}

impl Dependency {
    pub fn parse(b: &ftd::p2::Document) -> Vec<Dependency> {
        b.to_owned().instances("fpm#dependency").unwrap()
    }

    pub async fn download_zip(&self) -> Result<(), std::io::Error> {
        let download_url = match self.repo.as_str() {
            "github" => {
                format!(
                    "https://github.com/{}/archive/refs/heads/main.zip",
                    self.name
                )
            }
            k => k.to_string(),
        };

        let response = reqwest::get(download_url).await.expect("");

        std::fs::create_dir_all("./.packages/.cache").expect("failed to create build folder");

        let download_path = format!("./.packages/.cache/{}.zip", self.name.replace("/", "__"));
        let path = std::path::Path::new(download_path.as_str());
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let content = response.bytes().await.expect("");

        file.write_all(&content).expect("");
        // Ok(file)
        let file = std::fs::File::open(&path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let mut c_file = archive.by_index(i).unwrap();
            let outpath = match c_file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };
            let outpath_without_folder = outpath.to_str().unwrap().split_once("/").unwrap().1;
            let new_outpath = format!("{}/{}", self.name, outpath_without_folder);
            let file_extract_path = std::path::Path::new(new_outpath.as_str());
            if (&*c_file.name()).ends_with('/') {
                println!("File {} extracted to \"{}\"", i, outpath.display());
                std::fs::create_dir_all(
                    format!("./.packages/{}", file_extract_path.to_str().unwrap()).as_str(),
                )
                .unwrap();
            } else {
                if let Some(p) = file_extract_path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(
                            format!("./.packages/{}", &p.to_str().expect("")).as_str(),
                        )
                        .unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(format!(
                    "./.packages/{}",
                    &file_extract_path.to_str().expect("")
                ))
                .unwrap();
                std::io::copy(&mut c_file, &mut outfile).unwrap();
            }
        }
        Ok(())
    }

    pub async fn process(&self) -> bool {
        self.download_zip().await.is_ok()
    }
}
