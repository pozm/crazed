use std::path::Path;
pub async fn download_url(url: &str, filename: &str) {
    let pa = std::env::temp_dir();
    // let mut file = std::fs::File::create(pa.join(format!("./{}", self.downloaded))).expect("unable to make file");
    let mut dest =
        std::fs::File::create(filename).expect("unable to make file");
    println!("{:#?}", dest);
    let src = reqwest::get(url)
        .await
        .expect("unable to make request")
        .bytes()
        .await
        .expect("unable to get bytes");
    std::io::copy(&mut src.as_ref(), &mut dest)
        .expect(&*format!("unable to write to file for {}", filename));
}
