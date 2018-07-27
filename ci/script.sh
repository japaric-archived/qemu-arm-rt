set -euxo pipefail

main() {
    local T=$TARGET

    cargo check --target $T

    cargo run --target $T --example hello
    cargo run --target $T --example math

    set +e
    cargo run --target $T --example stderr
    [ $? = 1 ] || exit 1

    cargo run --target $T --example panic
    [ $? = 134 ] || exit 1
}

main
