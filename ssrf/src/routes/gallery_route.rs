use rocket::{response::content::RawHtml, tokio::fs};

// get url for file and download it to local storage
#[get("/gallery")]
pub async fn gallery() -> RawHtml<String> {
    // get all files in content folder
    let mut paths = fs::read_dir("content").await.unwrap();

    let mut html = String::from("<html><body>");
    // create simple html list of imgs
    while let Some(path) = paths.next_entry().await.unwrap() {
        // let path = path.unwrap().path();
        let path = path.file_name();
        html.push_str(&format!(
            "<img src=/content/{} width=\"200\" height=\"200\">",
            path.to_str().unwrap()
        ));
    }
    html.push_str(
        r#"<body>
        <form id="myForm">
          <label for="link">Enter URL:</label>
          <input type="text" id="link" name="link" pattern="https?://.*" title="Please enter a valid URL" placeholder="http://blah.com" required>
          <input type="submit" value="Submit">
        </form>
      
        <script>
          document.getElementById('myForm').addEventListener('submit', function(e) {
            e.preventDefault();
            const link = document.getElementById('link').value;
            const data = { "url": link };
            fetch('/api/photo_upload', {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json'
              },
              body: JSON.stringify(data)
            }).then(response => {
                location.reload()
            }).catch(error => {
            });
          });
        </script>"#,
    );

    html.push_str("</body></html>");
    RawHtml(html)
}
