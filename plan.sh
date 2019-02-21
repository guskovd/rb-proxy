pkg_name=rb-proxy-repo
pkg_origin=guskovd
pkg_version='1.0.0'
pkg_description="Rb-proxy repo"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/rb-proxy"

pkg_hab_shell_interpreter="bash"

RUBY_VERSION=2.5.1

pkg_deps=(
    core/bash/4.4.19/20180608092913
    core/clang/7.0.0/20181212192223
    core/docker/18.03.0/20180608150948
    core/gawk/4.2.0/20180608093856
    core/gcc-libs/7.3.0/20180608091701
    core/gcc/7.3.0/20180608051919
    core/git/2.18.0/20181218161804
    core/grep/3.1/20180608092809
    core/hab/0.73.0/20190115004751
    core/libarchive/3.3.2/20181214200119
    core/libsodium/1.0.13/20180703181056
    core/make/4.2.1/20180608100733
    core/openssl/1.0.2q/20181212183918
    core/pkg-config/0.29.2/20180608091734
    core/rsync/3.1.2/20180608145950
    core/ruby/2.5.1/20181212185250
    core/sshpass/1.06/20180608151129
    core/sudo/1.8.18p1/20181219210923
    guskovd/rust-nightly/1.32.0-2018-12-13/20181214174458
)

do_shell() {
    export PKG_CONFIG_PATH="$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"

    ruby_bundle_path=$HOME/.hab-shell/ruby/bundle/$RUBY_VERSION

    mkdir -p $ruby_bundle_path
    export BUNDLE_PATH=$ruby_bundle_path

    pushd "$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" > /dev/null
    bundle install --binstubs > /dev/null
    popd > /dev/null

    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.hab-shell/bin:$PATH"
    export PATH="$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/bin:$PATH"
    
    . ~/.bashrc
}

do_build() {
    return 0
}

do_install() {
    return 0
}

do_setup_environment() {
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/gcc-libs)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/libsodium)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/libarchive)/lib"
    push_runtime_env LD_LIBRARY_PATH "$(pkg_path_for core/openssl)/lib"
}
