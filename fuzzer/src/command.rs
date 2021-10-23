use crate::{check_dep, search, tmpfs};
use angora_common::defs;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

static TMP_DIR: &str = "tmp";
static INPUT_FILE: &str = "cur_input";
static FORKSRV_SOCKET_FILE: &str = "forksrv_socket";
static TRACK_FILE: &str = "track";
static PIN_ROOT_VAR: &str = "PIN_ROOT";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstrumentationMode {
    LLVM,
    Pin,
}

impl InstrumentationMode {
    pub fn from(mode: &str) -> Self {
        match mode {
            "llvm" => InstrumentationMode::LLVM,
            "pin" => InstrumentationMode::Pin,
            _ => unreachable!(),
        }
    }

    pub fn is_pin_mode(&self) -> bool {
        self == &InstrumentationMode::Pin
    }
}

#[derive(Debug, Clone)]
pub struct CommandOpt {
    pub id: usize,
    pub mode: InstrumentationMode,
    pub main: (String, Vec<String>),
    pub track: (String, Vec<String>),
    pub tmp_dir: PathBuf,
    pub input_path: String,
    pub track_path: String,
    pub forksrv_socket_path: String,
    pub search_method: search::SearchMethod,
    pub mem_limit: u64,
    pub time_limit: u64,
    pub is_stdin: bool,
    pub is_raw: bool,
    pub uses_asan: bool,
    pub ld_library_path: String,
    pub enable_afl: bool,
    pub enable_exploitation: bool,
}

impl CommandOpt {
    pub fn new(
        mode: &str,
        track_target: &str,
        pargs: Vec<String>,
        out_dir: &Path,
        search_method: &str,
        mut mem_limit: u64,
        time_limit: u64,
        enable_afl: bool,
        enable_exploitation: bool,
    ) -> Self {
        let mode = InstrumentationMode::from(mode);
        let (tmp_dir, input_path, track_path, forksrv_socket_path) = create_tmp_dir_and_path(out_dir);
        let (main, track) = prepare_target_and_args(track_target, &pargs, &mode);
        let uses_asan = check_dep::check_asan(&main.0);
        if uses_asan && mem_limit != 0 {
            warn!("The program compiled with ASAN, set MEM_LIMIT to 0 (unlimited)");
            mem_limit = 0;
        }
       
        Self {
            mode,
            id: 0,
            main,
            track,
            tmp_dir,
            input_path,
            forksrv_socket_path,
            track_path,
            search_method: search::parse_search_method(search_method),
            mem_limit,
            time_limit,
            uses_asan,
            is_stdin: !pargs.contains(&"@@".to_string()),
            is_raw: true,
            ld_library_path: fetch_env_ld_library_path(),
            enable_afl,
            enable_exploitation,
        }
    }
  
    /// specifcy variables (paths and args) for corresponding job
    pub fn specify(&self, id: usize) -> Self {
        let mut cmd_opt = self.clone();
        cmd_opt.id = id;
        cmd_opt.is_raw = false;
 
        // update paths
        let new_input_path = format!("{}_{}", &cmd_opt.input_path, id);
        let new_track_path = format!("{}_{}", &cmd_opt.track_path, id);
        let new_forksrv_socket_path = format!("{}_{}", &cmd_opt.forksrv_socket_path, id);
        cmd_opt.input_path = new_input_path.to_owned();
        cmd_opt.forksrv_socket_path = new_forksrv_socket_path.to_owned();
        cmd_opt.track_path = new_track_path.to_owned();

        // update args 
        if !self.is_stdin {
            for arg in &mut cmd_opt.main.1 {
                if arg == "@@" {
                    *arg = new_input_path.clone();
                }
            }
            for arg in &mut cmd_opt.track.1 {
                if arg == "@@" {
                    *arg = new_input_path.clone();
                }
            }
        }

        cmd_opt
    }
}

fn create_tmp_dir_and_path(out_dir: &Path) -> (PathBuf, String, String, String) {
    let tmp_dir = out_dir.join(TMP_DIR);
    tmpfs::create_tmpfs_dir(&tmp_dir);

    let input_path = tmp_dir.join(INPUT_FILE).to_str().unwrap().to_owned();
    let forksrv_socket_path = tmp_dir
        .join(FORKSRV_SOCKET_FILE)
        .to_str()
        .unwrap()
        .to_owned();

    let track_path = tmp_dir.join(TRACK_FILE).to_str().unwrap().to_owned();
    
    (tmp_dir, input_path, track_path, forksrv_socket_path)
}


fn prepare_target_and_args(track_target: &str, pargs: &Vec<String>, mode: &InstrumentationMode) -> ((String, Vec<String>), (String, Vec<String>)) {
    let mut tmp_args = pargs.clone();
    let main_bin = tmp_args[0].clone();
    let main_args: Vec<String> = tmp_args.drain(1..).collect();

    assert_ne!(
        track_target, "-",
        "You should set track target with -t PROM in LLVM mode!"
    );
    let track_bin;
    let mut track_args = Vec::<String>::new();
    if mode.is_pin_mode() {
        let project_bin_dir = env::var(defs::ANGORA_BIN_DIR).expect("Please set ANGORA_PROJ_DIR");
        
        let pin_root =
            env::var(PIN_ROOT_VAR).expect("You should set the environment of PIN_ROOT!");
        let pin_bin = format!("{}/{}", pin_root, "pin");
        track_bin = pin_bin.to_string();
        let pin_tool = Path::new(&project_bin_dir)
            .join("lib")
            .join("pin_track.so")
            .to_str()
            .unwrap()
            .to_owned();

        track_args.push(String::from("-t"));
        track_args.push(pin_tool);
        track_args.push(String::from("--"));
        track_args.push(track_target.to_string());
        track_args.extend(main_args.clone());
    } else {
        track_bin = track_target.to_string();
        track_args = main_args.clone();
    }

    ((main_bin, main_args), (track_bin, track_args))
}

 fn fetch_env_ld_library_path() -> String {
    let clang_lib = Command::new("llvm-config")
        .arg("--libdir")
        .output()
        .expect("Can't find llvm-config")
        .stdout;
    let clang_lib = String::from_utf8(clang_lib).unwrap();
    let ld_library_path = "$LD_LIBRARY_PATH:".to_string() + clang_lib.trim();
    
    ld_library_path
}

impl Drop for CommandOpt {
    fn drop(&mut self) {
        if self.is_raw {
            tmpfs::clear_tmpfs_dir(&self.tmp_dir);
        }
    }
}
