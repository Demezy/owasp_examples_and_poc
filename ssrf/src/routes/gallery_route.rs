use rocket::{response::content::RawHtml, tokio::fs};

// get url for file and download it to local storage
#[get("/gallery")]
pub async fn gallery() -> RawHtml<String> {
    // get all files in content folder
    let mut paths = fs::read_dir("content").await.unwrap();

    let mut html = String::from("<html><head>");
    html.push_str(r#"<link rel="stylesheet" type="text/css" href="/static/styles.css"></head>"#);
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
      <div id="errorContainer" style="color: red;"></div>
      <form id="galleryForm">
          <label for="link">Enter Image URL:</label>
          <input type="url" id="link" name="link" placeholder="http://example.com/image.jpg" required>
          <button type="submit">Add Image</button>
      </form>

      <script>
          document.getElementById('galleryForm').addEventListener('submit', function (e) {
              e.preventDefault();
              const link = document.getElementById('link').value;
              const data = { "url": link };
              fetch('/api/photo_upload', {
                  method: 'POST',
                  headers: {
                      'Content-Type': 'application/json'
                  },
                  body: JSON.stringify(data)
              })
              .then(response => {
                  if (!response.ok) {
                      throw new Error('Failed to upload image.');
                  }
                  location.reload();
              })
              .catch(error => {
                  document.getElementById('errorContainer').innerText = error.message;
              });
          });
      </script>"#,
  );

    html.push_str("</body></html>");
    RawHtml(html)
}
