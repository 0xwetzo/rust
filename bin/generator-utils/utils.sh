#!/usr/bin/env bash

# shellcheck source=/dev/null
source ./bin/generator-utils/colors.sh

function message() {
    local flag=$1
    local message=$2

    case "$flag" in
    "success")
        printf "${GREEN}%s${RESET}\n" "[success]: $message"
        ;;
    "info")
        printf "${BLUE}%s${RESET}\n" "[info]: $message"
        ;;
    "error")
        printf "${RED}%s${RESET}\n" "[error]: $message"
        ;;
    "done")
        echo
        cols=$(tput cols)
        printf "%*s\n" "$cols" "" | tr " " "-"
        echo
        printf "${BOLD_GREEN}%s${RESET}\n" "[done]: $message"
        ;;
    *)
        echo "Invalid flag: $flag"
        ;;
    esac
}

function dash_to_underscore() {
    # shellcheck disable=SC2001
    echo "$1" | sed 's/-/_/g'
}

function check_exercise_existence() {
    message "info" "Looking for exercise"
    slug="$1"
    unimplemented_exercises=$(bin/configlet info | sed -n '/With canonical data:/,/Track summary:/p' | sed -e '/\(With\(out\)\? canonical data:\|Track summary:\)/d' -e '/^$/d')
    if echo "$unimplemented_exercises" | grep -q "^$slug$"; then
        message "success" "Found exercise successfully"
    else
        message "error" "Exercise is either implemented or doesn't exist"
        message "info" "These are the unimplemented practice exercises:
${unimplemented_exercises}"
        exit 1
    fi
}
