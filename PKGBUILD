# Maintainer: Tim Apple <tim@timapple.com>
pkgname=writeapp
pkgver=0.1.0
pkgrel=1
pkgdesc="A distraction-free terminal-based writing app with vim keybindings, flow mode, and markdown support"
arch=('x86_64')
url="https://github.com/timappledotcom/writeapp"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/timappledotcom/writeapp/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
