// compile-flags: --env-remove PATH -Zunstable-options
fn main() {
    env!("PATH");
    //~^ ERROR: environment variable `PATH` not defined
}
