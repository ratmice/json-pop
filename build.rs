extern crate lalrpop;

fn main() {
    let mut lalrpop = lalrpop::Configuration::new();
    lalrpop
        .use_cargo_dir_conventions()
        .emit_rerun_directives(true)
        .process()
        .unwrap();
}
