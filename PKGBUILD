# Maintainer: Jim Verheijde <jimver1@hotmail.com>
pkgname=catsay
pkgver=0.2.0.9
pkgrel=1
pkgdesc="This is a useless cli tool of a cat echoing what you say."
url=https://github.com/Jimver/catsay
license=('MIT')
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    cargo build --release
}

package() {
    echo "$pkgdir"
    mkdir -p "$pkgdir/usr/bin/"
    cp "$startdir/target/release/catsay" "$pkgdir/usr/bin/"
}
