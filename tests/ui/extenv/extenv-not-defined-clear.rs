// compile-flags: --env-clear -Zunstable-options
fn main() {
    env!("PATH");
    //~^ ERROR: environment variable `PATH` not defined
}
