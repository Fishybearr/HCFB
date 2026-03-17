# Highly Customizable File Browser (HCFB)

A file browser with high customizablity that is efficient and works on both Windows and Linux

## Must haves
- Able to display a large number of files and folders at once
    - Needs virtual scrolling to dynamically load GUI elements
- small program size
- FAST
- customizable default programs for launching any file type
_ ASYNC file operations
    - tokio is supposedly good for async file operations
- Tuari does rust gui with css
- Really really good documentation
    - especially for CSS styling

## Wants
- Security features like showing if a file is chmod 777
- CSS styling system for GUI
- please please please split windows and tabs
- Built in text editor
    - in a beautiful perfect world it would have syntax highlighting


## Things to remember
This is my first time using Rust for a real project so don't give up like a dumbass when you do something wrong or don't understand a package you stupid idiot
- run app with npm run tauri dev
-IMPORTANT
    - when getting read for a release build, change the empty array for targets in tauri.conf.json to "all" (quotes in included).
    This will give us a nice packaged build, but for arch, linuxdeploy doesn't work so we don't use it for testing

## TODO
- For CSS styling, create a config file at ~/.config/HCFB/style.css
    - Then read from that dir on program startup to populate css

- Make each list entry a button that calls the function to refresh the page contents with the new from the button

- Add a back button on top of the standard ../ to handle
if a user went to a path from search as opposed to just going down another level

- Need to deal with perm errors by something like letting user move up to root or something

- [X] Update FileObjects to display local path in a directory instead of absolute path

- Need to push to history stack any time you enter directory


## Snippits
```html
{#each fileList as file}
    <li><button onclick={ () => { //TODO: update this section to validate a path before adding to history
    //Currently validates the next path while adding the current path to history regardless of it's filetype
      if(file.file_type){history.push(path)} console.log(history); path = file.path }}>{file.path}, {file.file_type}</button></li>

  {/each}
  ```