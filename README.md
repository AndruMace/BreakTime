# Break Time!

Break time is a CLI application that helps you keep track of and visit your favorite digital destinations. By default it creates a file with links to reddit, hackernews, youtube, and the rust programming language website. Feel free to add to those or clear them and start anew with the below commands. 


### Commands

Primary
- ```breaktime``` selects random url from your collection and opens it in browser 
- ```breaktime add <url>``` adds a url to your collection (must be FULL url)
- ```breaktime list``` lists your url collection
- ```breaktime rm``` removes the most recently added url.
- ```breaktime clear``` clears your url collection so you can start from scratch.

Other
- ```breaktime length``` randomly generates how long your break will be, between 10-30 minutes. 


### Instructions
If you are familiar with cargo or the rust toolchain then you can simply clone this repo and run it via cargo. However the intended usage is to put the executable in your path which can be done a variety of ways. I personally have a directory ```~/bin``` in my path that I throw any executables I want to run directly from the CLI. See link below for more information.

https://zwbetz.com/how-to-add-a-binary-to-your-path-on-macos-linux-windows/

The executable is located in ```src/target/release/breaktime```
