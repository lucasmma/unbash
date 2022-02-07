
mod domain;
mod infra;

use whoami;

use crate::infra::bash_manager::BashManager;

fn main() {
    let bash = BashManager{ username: whoami::username() };
    bash.run();
}
