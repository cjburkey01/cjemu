#![feature(with_options)]

// App data directory relative to the user's home directory
const APP_DIR_REL: &str = "cjemu";
// The font directory relative to the `cjemu` directory
const FONT_DIR_REL: &str = "font";
// Font location relative to the `cjemu` font directory
const FONT_REL: &str = "main_font.ttf";

use directories::UserDirs;
use fltk::app::App;
use fltk::group::PackType;
use fltk::text::{TextBuffer, TextEditor};
use fltk::{app, enums::Font, group::Pack, prelude::*, window::DoubleWindow, window::Window};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
struct CJEmu {
    files: CJEmuFiles,
    app: App,
    window: Option<DoubleWindow>,

    terminal_font: Font,

    memory_map_tmp: Option<TextEditor>,
    console_tmp: Option<TextEditor>,
}

#[derive(Debug)]
struct CJEmuFiles {
    user_dirs: UserDirs,
    home_dir: PathBuf,
    cjemu_dir: PathBuf,
    extracted_font_path: PathBuf,
}

fn main() {
    println!(
        "starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Get file locations and directories
    let files = load_files();
    println!("important file locations: {:#?}", files);

    // Create the wrapper application
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    println!("initialized FLTK");

    // Load the font (and extract FiraCode from the binary as the default if
    // necessary)
    let default_font = include_bytes!("../font/FiraCode-Regular.ttf");
    let terminal_font = load_font(&app, default_font, &files.extracted_font_path);

    // Wrap everything in a neat wrapper
    let mut cjemu = CJEmu {
        files,
        app,
        window: None,

        terminal_font,

        memory_map_tmp: None,
        console_tmp: None,
    };

    // Create the window
    create_window(
        &mut cjemu,
        concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")),
    );
    println!("created window");

    // Show the window and start the app
    cjemu.window.expect("failed to load window").show();
    println!("displayed window");

    // Start the event loop, blocking execution in this thread until the app
    // exits
    app.run().unwrap();

    println!("exiting");
}

fn load_files() -> CJEmuFiles {
    let user_dirs = UserDirs::new().expect("failed to get user-specific directories");
    let home_dir = PathBuf::from(user_dirs.home_dir());
    // `cjemu` directory within the user's home folder
    let cjemu_dir = {
        let mut cjed = PathBuf::from(user_dirs.home_dir());
        cjed.push(APP_DIR_REL);
        cjed
    };
    // Terminal font file location (within `cjemu` directory)
    let extracted_font_path = {
        let mut efp = PathBuf::from(&cjemu_dir);
        efp.push(FONT_DIR_REL);
        efp.push(FONT_REL);
        efp
    };

    // Wrap the files into a neat little struct
    CJEmuFiles {
        user_dirs,
        home_dir,
        cjemu_dir,
        extracted_font_path,
    }
}

fn load_font(app: &app::App, default_font: &[u8], font_file_loc: &Path) -> Font {
    let font_name = &{
        // Write the font from the binary into the output file if it
        // doesn't exist
        if !font_file_loc.exists() {
            println!(
                "font not found at {:?}, extracting the default packaged font (FiraCode)",
                font_file_loc
            );
            {
                {
                    let mut font_path = PathBuf::from(font_file_loc);
                    // Remove file name, leaving only the directory
                    font_path.pop();
                    std::fs::create_dir_all(&font_path).unwrap_or_else(|_| {
                        panic!("failed to create font directory at {:?}", font_path)
                    });
                }

                let mut output_file = File::with_options()
                    .write(true)
                    .create(true)
                    .open(font_file_loc)
                    .unwrap_or_else(|_| panic!("failed to create file at {:?}", font_file_loc));
                output_file
                    .write_all(default_font)
                    .unwrap_or_else(|_| panic!("failed to write font file at {:?}", font_file_loc));
            }
            println!("extracted font");
        }

        app.load_font(font_file_loc)
            .unwrap_or_else(|_| panic!("failed to load font at {:?}", font_file_loc))
    };
    println!("loaded font by name {}", font_name);

    Font::by_name(font_name)
}

fn create_window(cjemu: &mut CJEmu, title: &'static str) {
    // Create the window
    let mut wind = Window::new(0, 0, 150, 100, title).center_screen();
    wind.make_resizable(true);

    create_outer_pack(cjemu, &wind);

    // Finish window creation
    wind.end();
    cjemu.window = Some(wind);
}

fn create_outer_pack(cjemu: &mut CJEmu, window: &Window) -> Pack {
    // Create the master horizontal pack
    let mut outer_pack = Pack::default().with_size(150, 100).center_of(window);
    outer_pack.set_spacing(10);
    outer_pack.set_type(PackType::Horizontal);
    window.resizable(&outer_pack);

    // Create children
    create_left_pack(cjemu);
    create_right_pack(cjemu);

    // Finish the outer pack
    outer_pack.end();
    outer_pack
}

fn create_left_pack(cjemu: &mut CJEmu) -> Pack {
    // Create the vertical left pack
    let mut left_pack = Pack::default().with_size(75, 100);
    left_pack.set_type(PackType::Vertical);

    cjemu.memory_map_tmp = Some({
        let mut editor = TextEditor::new(0, 0, 75, 100, "");
        editor.set_buffer(Some({
            let mut buf = TextBuffer::default();
            buf.append("memory map!");
            buf
        }));
        editor.set_text_font(cjemu.terminal_font);
        left_pack.resizable(&editor);
        editor
    });

    // Finish left pack
    left_pack.end();
    left_pack
}

fn create_right_pack(cjemu: &mut CJEmu) -> Pack {
    // Create the vertical right pack
    let mut right_pack = Pack::default().with_size(75, 100);
    right_pack.set_type(PackType::Vertical);

    cjemu.console_tmp = Some({
        let mut editor = TextEditor::new(0, 0, 75, 100, "");
        editor.set_buffer(Some({
            let mut buf = TextBuffer::default();
            buf.append("console!");
            buf
        }));
        editor.set_text_font(cjemu.terminal_font);
        right_pack.resizable(&editor);
        editor
    });

    // Finish left pack
    right_pack.end();
    right_pack
}
