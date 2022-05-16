# rusty_picture_namer_gui
This is picture_namer written in Rust which also includes a GUI written in Rust. I used the egui package to build the GUI. This prepends the directory name to each specified filetype along with a 5 digit number starting at "00000" eg: `<directory_name>_<xxxxx>_<filename>`. If there are already files with the `<directory_name>_<xxxxx>_<filename>` naming format in the directory then the script will count these files to prevent simultaneous use of the same numbers in the new filename. If there are more than 99999 files then the script will add more leading zeros. Spaces are changed into "_". Specified filetypes can be read from a text file or specified via the command line. 

I've only tested this using Windows 11, cargo 1.60.0, and Rust 1.60.0.
## Dependencies:
walkdir = "2.3.2"
time = "0.3.7"
eframe = "0.18.0"
rfd = "0.8.2"

## main.rs
This constructs the window.

## gui.rs
This file contains the code for the GUI.

## picture_namer.rs
This file contains the code that names the files.

## gui/_list_of_filetypes.txt
A list of filetypes supported by the program.

## build.bat
This builds the debug and release binaries along with the documentation.
