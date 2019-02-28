pkg_name=rb-proxy-repo
pkg_origin=guskovd
pkg_version='1.0.0'
pkg_description="Rb-proxy repo"
pkg_maintainer='Guskovd'
pkg_upstream_url="https://github.com/guskovd/rb-proxy"

pkg_hab_shell_interpreter="bash"

RUBY_VERSION=2.5.3

pkg_deps=(
    core/bash
    core/clang
    core/docker
    core/gawk
    core/gcc-libs
    core/gcc
    core/git
    core/grep
    core/hab
    core/libarchive
    core/libsodium
    core/make
    core/openssl
    core/pkg-config
    core/rsync
    core/ruby/$RUBY_VERSION
    core/sshpass
    core/sudo
    core/gdb
    core/jq-static
    guskovd/rust-nightly
    guskovd/rust-racer
)

do_shell() {
    export RUST_SRC_PATH=$HOME/rust/src/
    export CARGO_HOME=$( builtin cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/.cargo
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
