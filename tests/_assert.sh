assert_eq() {
    if [[ "$1" = "$2" ]]
    then
        echo "$(tput setaf 2)$(tput bold)Assertion ok$(tput sgr0)"
    else
        echo "$(tput setaf 1)$(tput bold)Assertion failed$(tput sgr0)"
        echo "Expected: $(tput setaf 2)$1$(tput sgr0)"
        echo "Actual: $(tput setaf 1)$2$(tput sgr0)"
        exit 1
    fi
}