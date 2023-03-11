# Rust Streaming Server

## How to use?

* First, create a resource directory inside **/src** and add video (mp4 format) to it.  
  
Then execute the following command:  
  
```bash
cargo install cargo-edit
cargo install cargo-watch
cargo watch -x run
```
  
When the server is running, open index.html inside the **static/** directory with a browser.  
  
Note: The project is currently in development and has many features that have not been implemented.

## API

* `/video` : Loads the video file in the resource directory and returns it to the client with status code 403 as many as Chunks
* `/list` : Reads and returns a file within the current resource directory as a list
* `/preview_image`: incomplete
