set -euxo pipefail

main() {
    local T=$TARGET

    cross check --target $T

    cross run --target $T --example hello
    cross run --target $T --example math

    set +e
    cross run --target $T --example stderr
    [ $? = 1 ] || exit 1

    cross run --target $T --example panic
    [ $? = 134 ] || exit 1
}

main
