# This script takes care of building your crate and packaging it for release

set -ex

mk_deb() {
    if [ ! -z $MAKE_DEB ]; then
        dtd=$(mktempd)
        mkdir -p $dtd/debian/usr/bin

        dobin target/$TARGET/release/pickpocket-*

        mkdir -p $dtd/debian/DEBIAN
        cat >$dtd/debian/DEBIAN/control <<EOF
Package: $PROJECT_NAME
Version: ${TRAVIS_TAG#v}
Architecture: $(architecture $TARGET)
Maintainer: $DEB_MAINTAINER
Description: $DEB_DESCRIPTION
EOF

        fakeroot dpkg-deb --build $dtd/debian
        mv $dtd/debian.deb $PROJECT_NAME-$TRAVIS_TAG-$TARGET.deb
        rm -r $dtd
    fi
}

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --target $TARGET --release -- -C lto

    cp target/$TARGET/release/pickpocket $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    if [ $TRAVIS_OS_NAME = linux ]; then
        mk_deb
    fi

    rm -rf $stage
}

main
