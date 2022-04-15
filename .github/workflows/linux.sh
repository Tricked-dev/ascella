#/bin/bash

#
# Copyright tricked-dev 2022
#

wget "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
chmod a+x appimagetool-x86_64.AppImage
sudo mv appimagetool-x86_64.AppImage /bin/appimagetool

cd desktop

run() {
    strip -s target/release/ascella-desktop
    echo installing cargo $1
    cargo install cargo-$1 &>/dev/null
    echo Running cargo $1
    cargo $1 &>/dev/null
}

run deb
run aur
run appimage
run generate-rpm

mkdir ../dist &>/dev/null

mv PKGBUILD ../dist/
mv ascella-desktop-*.AppImage ../dist/
mv ascella-desktop-*.tar.gz ../dist/
mv target/debian/ascella-desktop*.deb ../dist/
mv target/generate-rpm/ascella-desktop-*.rpm ../dist/
cp LICENSE ../dist/
