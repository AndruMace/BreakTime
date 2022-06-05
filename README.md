# Break Time!

Break time is a CLI application that help you keep track of and visit your favorite digital destinations. By default it creates a file with links to reddit, hackernews, youtube, and the rust programming language website. Feel free to add to those or clear them and start anew with the below commands. 


### Commands

- ```breaktime``` selects random url from your collection and opens it in browser 
- ```breaktime add <url>``` add a url to your collection (must be FULL url)
- ```breaktime list``` list your url collection
- ```breaktime clear``` start with a blank slate and clear your url collection.


### Instructions
If you are familiar with cargo or the rust toolchain then you can simply clone this repo and run it via cargo. However the intended usage is to put the executable in your path which can be done a variety of ways. I personally have a directory ```~/bin``` in my path that I throw any executables I want to run directly from the CLI. See link below for more information.

https://zwbetz.com/how-to-add-a-binary-to-your-path-on-macos-linux-windows/

The executable is located in ```src/target/debug/breaktime```
