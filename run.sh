#!/usr/bin/env bash

builds_dir="builds"

if [ -n "$1" ]; then
  file_path="$1"
else
  file_path=$(find . -type f -name "*.rs" -not -name "mod.rs" | fzf)
fi

file_name=$(basename -- "$file_path")
file_name_without_ext="${file_name%.*}"
binary_name="$builds_dir/$file_name_without_ext"
input_file_name="$builds_dir/${file_name_without_ext}.txt"

echo "Compiling $file_path"
rustc --out-dir "builds" "$file_path"

if [ ! -f "$input_file_name" ]; then
  echo "Input for $file_path"
  wl-paste | sed '/^$/d' >"$input_file_name"
fi

echo "Running $binary_name"
"./$binary_name" <"$input_file_name"
