#!/usr/bin/env bash

builds_dir="builds"
use_cargo=false

if [ "$1" == "-c" ]; then
  use_cargo=true
  shift
fi

if [ -n "$1" ]; then
  file_path="${1#./}"
else
  file_path=$(find . -type f -name "*.rs" -not -name "mod.rs" | fzf)
fi

function create_input_file() {
  if [ ! -f "$2" ]; then
    echo "Input for $1"
    wl-paste | sed '/^$/d' >"$2"
  fi

}

file_name_without_ext=$(echo "${file_path%.*}" | tr '/' '-')
input_file_name="${file_name_without_ext}.txt"
if $use_cargo; then
  temp_dir="/tmp/$file_name_without_ext"
  if [ ! -d "$temp_dir" ]; then
    mkdir -p "$temp_dir/src"
  fi

  cp ./Cargo.toml "$temp_dir"
  cp ./Cargo.lock "$temp_dir"
  cp "$file_path" "$temp_dir/src/main.rs"

  create_input_file "$file_path" "$temp_dir/$input_file_name"

  cd "$temp_dir" || exit 1

  cargo run <"$temp_dir/$input_file_name"
else
  binary_name="$builds_dir/$file_name_without_ext"
  input_file_name="$builds_dir/$input_file_name"

  echo "Compiling $file_path"
  rustc -o "$binary_name" "$file_path"

  create_input_file "$file_path" "$input_file_name"

  echo "Running $binary_name"
  "./$binary_name" <"$input_file_name"
fi
