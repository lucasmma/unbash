
mod domain;
mod infra;

use whoami;

use crate::infra::bash_manager::BashManager;

fn main() {
    let mut bash = BashManager{ username: whoami::username(), history: vec![] };
    bash.run();
}
