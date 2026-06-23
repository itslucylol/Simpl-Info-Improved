use crate::wd;
use crate::patches;

const SIMPLINFO_PATH: &str = "../vendor/simplinfo/simplinfo LE";

pub fn patch() {
    wd::cp(SIMPLINFO_PATH, Some("sim"));
    patches::header::main();
}