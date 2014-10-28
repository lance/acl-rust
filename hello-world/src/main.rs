extern crate semver;

use semver::Version;

fn main() {
    println!("Hello, world!")
    assert!(Version::parse("1.2.3") == Ok(Version {
        major: 1u,
        minor: 2u,
        patch: 3u,
        pre: vec!(),
        build: vec!(),
    }));

    println!("Versions compared successfully!");
}
