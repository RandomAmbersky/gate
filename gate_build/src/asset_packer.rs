// Copyright 2017 Matthew D. Michelotti
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write};
use std::ffi::OsStr;

use atlas::form_atlas;
use ::rerun_print;

// TODO have more careful checks on input
// TODO add unit test for rect_packer efficiency
// TODO add unit test for unsafe code in asset_id.template.rs

/// Packs image atlases and other assets as part of the build script for a Gate application.
///
/// Contains methods to pack all assets needed for a game
/// and generate code to refer to these assets.
pub struct AssetPacker {
    assets_dir: PathBuf,
    check_rerun: bool,
    sprites: Option<Vec<String>>,
    tiles: Option<Vec<String>>,
    music: Option<Vec<String>>,
    sounds: Option<Vec<String>>,
}

impl AssetPacker {
    /// Constructs a new `AssetPacker` where packed asset files will be written to `assets_dir`.
    ///
    /// The `assets_dir` must be a directory named "assets" relative to
    /// the current directory used when the Gate application is run.
    /// This is usually in the base directory of the Rust project.
    pub fn new(assets_dir: &Path) -> AssetPacker {
        assert!(assets_dir.file_name() == Some(OsStr::new("assets")), "assets_dir.file_name() must be 'assets'");
        fs::create_dir_all(assets_dir).expect("failed to create assets directory");
        AssetPacker {
            assets_dir: assets_dir.to_path_buf(),
            sprites: None,
            tiles: None,
            check_rerun: false,
            music: None,
            sounds: None,
        }
    }

    /// Invoke this method to print "cargo:rerun-if-changed" statements for all
    /// files read and written by `self`.
    ///
    /// This will retrigger a build script if any of these files are changed.
    /// Refer to Cargo documentation for more details.
    ///
    /// Panics if called after calling methods to pack assets.
    pub fn cargo_rerun_if_changed(&mut self) {
        assert!(self.sprites.is_none() && self.tiles.is_none(), "cannot add rerun checks after asset packing has already started");
        self.check_rerun = true;
    }

    /// Packs sprite images into an atlas, to be rendered by Gate renderer in "sprite" mode.
    ///
    /// Image ".png" files are read from `in_dir`,
    /// generating enum handles with the same names as the image files.
    /// Returns the list of these handles indexed by ID,
    /// in the same order that they appear in the generated enum code.
    ///
    /// The center of the image is used as the anchor point.
    /// If the width or height is odd, then the center of the image is between pixels.
    /// Transparent pixels padded around the image rectangle will be stripped before packing,
    /// so there is no need to worry about efficiency in regards to padded pixels.
    ///
    /// If any image filename ends with "_t#", where # is a number,
    /// then it will interpret that image as a collection of tiles
    /// with width and height equal to #.
    /// The names generated to refer to each of these tiles are suffixed with "R#C#",
    /// referencing the row and column number.
    /// Empty tiles will be omitted.
    ///
    /// Note: in the current implementation of `gate` there is only one atlas for sprites,
    /// but in the future there are plans to allow for larger games that may need
    /// multiple atlases loaded at different times.
    pub fn sprites(&mut self, in_dir: &Path) -> &[String] {
        assert!(self.sprites.is_none(), "self.sprites(...) was already invoked");
        let output = &self.assets_dir.join("sprites");
        self.sprites = Some(form_atlas(in_dir, output, 1, self.check_rerun));
        self.sprites.as_ref().unwrap()
    }

    /// Packs tile images into an atlas, to be rendered by Gate renderer in "tiled" mode.
    ///
    /// Follows the same packing conventions as `self.sprites(in_dir)`.
    /// See that method for more details.
    ///
    /// Note: in the current implementation of `gate` there is only one atlas for tiles,
    /// but in the future there are plans to allow for larger games that may need
    /// multiple atlases loaded at different times.
    pub fn tiles(&mut self, in_dir: &Path) -> &[String] {
        assert!(self.tiles.is_none(), "self.tiles(...) was already invoked");
        let output = &self.assets_dir.join("tiles");
        self.tiles = Some(form_atlas(in_dir, output, 0, self.check_rerun));
        self.tiles.as_ref().unwrap()
    }

    /// Creates handles for and moves music files from `in_dir` to the assets directory.
    ///
    /// Music files are expected to be in `.ogg` format,
    /// generating enum handles with the same names as the audio files.
    /// Returns the list of these handles indexed by ID,
    /// in the same order that they appear in the generated enum code.
    pub fn music(&mut self, in_dir: &Path) -> &[String] {
        assert!(self.music.is_none(), "self.music(...) was already invoked");
        self.music = Some(enumerate_files(in_dir, &self.assets_dir, "music", "ogg", self.check_rerun));
        self.music.as_ref().unwrap()
    }

    /// Creates handles for and moves sound files from `in_dir` to the assets directory.
    ///
    /// Music files are expected to be in `.ogg` format,
    /// generating enum handles with the same names as the audio files.
    /// Returns the list of these handles indexed by ID,
    /// in the same order that they appear in the generated enum code.
    pub fn sounds(&mut self, in_dir: &Path) -> &[String] {
        assert!(self.sounds.is_none(), "self.sounds(...) was already invoked");
        self.sounds = Some(enumerate_files(in_dir, &self.assets_dir, "sound", "ogg", self.check_rerun));
        self.sounds.as_ref().unwrap()
    }

    /// Generates Rust enums to use as handles for all of the packed assets.
    ///
    /// The generated code will consist of four enums:
    /// `SpriteId`, `TileId`, `MusicId`, and `SoundId`.
    /// These types are collected together in the type `AssetId`,
    /// which implements `gate::asset_id::AppAssetId`.
    /// Constructing a `gate::App` instance with this as the Asset ID type
    /// will allow you to use the generated handles to refer to assets.
    ///
    /// This method should be called after packing all of the assets.
    /// The `sprites` and `tiles` methods must be called before this,
    /// but `music` and `sounds` may be omitted if there is no audio.
    ///
    /// The generated Rust code is written to `out`.
    /// This will typically be a file in the `env::var("OUT_DIR")` directory.
    /// Refer to the Cargo build script documentation for more details.
    pub fn gen_asset_id_code(self, out: &Path) {
        self.gen_asset_id_code_checked(out).expect("error generating AssetId code")
    }

    fn gen_asset_id_code_checked(self, out: &Path) -> io::Result<()> {
        let sprites_enum = gen_asset_enum("SpriteId", &self.sprites.expect("self.sprites(...) was not invoked"));
        let tiles_enum = gen_asset_enum("TileId", &self.tiles.expect("self.tiles(...) was not invoked"));
        let music_enum = gen_asset_enum("MusicId", &self.music.unwrap_or(vec![]));
        let sounds_enum = gen_asset_enum("SoundId", &self.sounds.unwrap_or(vec![]));

        let code = format!(include_str!("asset_id.template.rs"),
                           sprites_enum, tiles_enum, music_enum, sounds_enum);
        if let Some(out_dir) = out.parent() {
            fs::create_dir_all(out_dir)?;
        }
        let mut file = File::create(out)?;
        file.write_all(code.as_bytes())?;
        Ok(())
    }
}

fn enumerate_files(in_dir: &Path, out_dir: &Path, prefix: &str, extension: &str, check_rerun: bool) -> Vec<String> {
    let mut paths: Vec<_> = in_dir.read_dir().unwrap()
                                  .filter_map(|p| p.ok())
                                  .map(|p| p.path())
                                  .filter(|p| p.extension() == Some(OsStr::new(extension)))
                                  .collect();
    paths.sort_unstable();
    for (id, path) in paths.iter().enumerate() {
        rerun_print(check_rerun, &path);
        let out_path = out_dir.join(format!("{}{}.{}", prefix, id, extension));
        rerun_print(check_rerun, &out_path);
        fs::copy(path, out_path).unwrap();
    }
    paths.iter().map(|p| p.file_stem().unwrap().to_str().unwrap().to_owned()).collect()
}

fn gen_asset_enum(name: &str, ids: &[String]) -> String {
    let mut ids_str = String::new();
    for id in ids {
        ids_str.push_str("    ");
        ids_str.push_str(id);
        ids_str.push_str(",\n");
    }

    if ids.len() == 0 {
        format!(include_str!("asset_id_0.template.rs"), name)
    } else if ids.len() <= 256 {
        format!(include_str!("asset_id_u8.template.rs"), name, ids_str, ids.len())
    } else if ids.len() <= 65536 {
        format!(include_str!("asset_id_u16.template.rs"), name, ids_str, ids.len())
    } else {
        panic!("too many {} assets", name);
    }
}