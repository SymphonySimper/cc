#!/usr/bin/env bash

# Exit if no arg is passed
[[ -z "$1" ]] && exit 1

# set path array
# $1 format is `<platform>:<x>:<y>:...`
# ex: leet:medium:5175
read -ra path < <(echo "$1" | tr ':' ' ')

template_dir="./.templates"
function template_to_file() {
  if [[ -z "${1}" ]] || [[ -f "${1}" ]]; then
    echo "Either \`${1}\` does already exists or was not passed"
    exit 1
  fi

  if [[ "${path[0]}" == "leet" ]]; then
    template_txt=$(<"${template_dir}/leet.rs")
  else
    template_txt=$(<${template_dir}/stdio.rs)
  fi

  echo "$template_txt" >"$1"
}

function add_to_mod() {
  mod_file="${2}/mod.rs"
  if [[ -z "$2" ]] || [[ ! -f "${mod_file}" ]]; then
    echo "Either \`${2}\` does not exist or was not passed"
    exit 1
  fi

  echo "pub mod ${1};" >>"${mod_file}"
}

function echo_success() {
  echo "File created at $1"
  just format
}

function new_aoc() {
  # TODO: add aoc
  base_dir="./src/aoc"

  year="${path[1]}"
  if [[ ! "$year" =~ ^[0-9]{4}$ ]]; then
    echo "\`$year\` is not a valid year!"
    exit 1
  fi

  year_name="y${year}"
  year_dir="${base_dir}/${year_name}"
  if [[ ! -d "$year_dir" ]]; then
    mkdir "$year_dir"
    add_to_mod "$year_name" "$base_dir"
  fi

  day="${path[2]}"
  if [[ ! "$day" =~ ^[0-9]{1,2}$ ]]; then
    echo "\`$day\` is not a valid day!"
    exit 1
  fi

  day_name="day${day}"
  day_dir="${year_dir}/${day_name}"
  if [[ ! -d "$day_dir" ]]; then
    mkdir "$day_dir"
    add_to_mod "$day_name" "$year_dir"
  fi

  part1_name="part1"
  part1_file="${day_dir}/${part1_name}.rs"
  if [[ ! -f "$part1_file" ]]; then
    template_to_file "$part1_file"
    add_to_mod "$part1_name" "$day_dir"
    echo_success "$part1_file"
  fi

  part2_name="part2"
  part2_file="${day_dir}/${part2_name}.rs"
  if [[ ! -f "$part2_file" ]]; then
    template_to_file "$part2_file"
    add_to_mod "$part2_name" "$day_dir"
    echo_success "$part2_file"
  fi
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
  file="${dir}/${challenge_name}.rs"

  template_to_file "$file"
  add_to_mod "${challenge_name}" "${dir}"
  echo_success "${file}"
}

function new_forces() {
  base_dir="./src/forces"

  div="${path[1]}"
  div_dir="${base_dir}/${div}"
  if [[ ! "$div" =~ ^[a-g]{1}$ ]]; then
    echo "\`${div}\` is not in the range of [a-g]"
    exit 1
  fi

  challenge="${path[2]}"
  challenge_name="cf_${div}${challenge}"
  if [[ -z "$challenge" ]]; then
    echo "Challenge name is not passed"
    exit 1
  fi

  file="${div_dir}/${challenge_name}.rs"
  template_to_file "$file"
  add_to_mod "$challenge_name" "$div_dir"
  echo_success "$file"
}

function new_timus() {
  base_dir="./src/timus"

  challenge="${path[1]}"
  if [[ -z "$challenge" ]]; then
    echo "Challenge number not mentioned"
    exit 1
  fi

  challenge_name="ts_${challenge}"
  file="${base_dir}/${challenge_name}.rs"

  template_to_file "$file"
  add_to_mod "${challenge_name}" "${base_dir}"
  echo_success "${file}"
}

case "${path[0]}" in
aoc) new_aoc ;;
leet) new_leet ;;
timus) new_timus ;;
forces) new_forces ;;
*) echo "Supported platforms aoc | leet | timus | forces" ;;
esac
