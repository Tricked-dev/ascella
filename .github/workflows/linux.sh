#/bin/bash

#
# Copyright tricked-dev 2022
#

cd apps/desktop

run() {
    strip -s target/release/ascella
    echo installing cargo $1
    cargo install cargo-$1 &>/dev/null
    echo Running cargo $1
    cargo $1 &>/dev/null
}

run deb
run aur
run appimage
run generate-rpm

mkdir ../../dist &>/dev/null

mv PKGBUILD ../../dist/
mv ascella-*.AppImage ../../dist/
mv ascella-*.tar.gz ../../dist/
mv target/debian/ascella_*.deb ../../dist/
mv target/generate-rpm/ascella-*.rpm ../../dist/
cp LICENSE ../../dist/
