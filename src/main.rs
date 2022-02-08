
mod domain;
mod infra;
mod utils;

use whoami;

use crate::infra::bash_manager::BashManager;
use crate::infra::os_manager::get_home_directory;
use crate::utils::parser_helper;
use crate::utils::file_helper;

fn read_profile(filename: String) -> Vec<String> {
    let mut home_path: String = get_home_directory();
    home_path.push_str(filename.as_str());
    if home_path.len() == 0 {
        return vec![]
    }
    let content = file_helper::read_file(home_path.clone());
    
    if content.eq("anything") {
        file_helper::create_file(home_path);
        return vec![]
    }
    
    return parser_helper::parse_paths(content);

}

fn main() {
    let paths : Vec<String> = read_profile(String::from("/.unbshrc_profile"));
    let mut bash = BashManager{ username: whoami::username(), history: vec![], paths: paths };
    bash.run();
}
