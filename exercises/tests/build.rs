//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable called `TEST_FOO`.
    // Print the correct command to let Cargo set the environment variable.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // This command tells Cargo to set an environment variable `TEST_FOO`
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we should enable the "pass" feature to make the testcase return early.
    // This command enables the "pass" feature for tests8.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
