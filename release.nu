#!/usr/bin/env nu
if not (jj log --revision @ --ignore-working-copy | str contains "(empty)") {
   print --stderr "Please start the release process from an empty change. Exiting."
   exit 1
}

let current_version = open Cargo.toml | get package.version
print $"Current version:   ($current_version)"
let new_version = input "Enter new version: "

if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+$') {
   print --stderr "Error: Version must follow semantic versioning. Exiting."
   exit 1
}

print $"Updating version from ($current_version) to ($new_version)"

open Cargo.toml
| upsert package.version $new_version
| save -f Cargo.toml
cargo check --quiet

let tag = $"v($new_version)"

print "Creating release commit..."
jj commit --message $"release: `($tag)`."
jj git export
git tag --force $tag --annotate --message $tag
jj bookmark set main --revision $tag

print $"Ready to release '($tag)'."
print $"Remember to push the tag `($tag)` and changes."
