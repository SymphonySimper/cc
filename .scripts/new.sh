#!/usr/bin/env bash

# Exit if no arg is passed
[[ -z "$1" ]] && exit 1

# set path array
# $1 format is `<platform>:<x>:<y>:...`
# ex: leet:medium:5175
read -ra path < <(echo "$1" | tr ':' ' ')

template_txt=$(<./template.txt)

function add_to_mod() {
  if [[ -z "$2" ]] && [[ ! -f "$2" ]]; then
    echo "Either \`${2}\` does not exist or was not passed"
    exit 1
  fi
  echo "pub mod $1" >>"$2"
}

function echo_success() {
  echo "File created at $1"
}

function new_aoc() {
  # TODO: add aoc
  dir="./src/aoc"
}

function new_leet() {
  difficulty=""

  case "${path[1]}" in
  easy | medium | hard) difficulty="${path[1]}" ;;
  *)
    echo "Supported difficulty for leet are <easy | medium | hard>"
    exit 1
    ;;
  esac

  if [[ -z "${path[2]}" ]]; then
    echo "Challenge number not mentioned!"
    exit 1
  fi

  challenge_name="lt_${path[2]}"
  dir="./src/leet/${difficulty}"
  file_mod="${dir}/mod.rs"
  file="${dir}/${challenge_name}.rs"

  if [[ -f "$file" ]]; then
    echo "${file} already exists"
    exit 1
  fi

  echo "$template_txt" >"$file"
  add_to_mod "$challenge_name" "$file_mod"

  echo_success "$file"
}

case "${path[0]}" in
aoc) new_aoc "$1" ;;
leet) new_leet "$1" ;;
*) echo "Supported platforms leet|aoc" ;;
esac
