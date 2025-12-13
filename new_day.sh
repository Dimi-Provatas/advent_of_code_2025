#!/usr/bin/env sh

set -e

WD=$(dirname $0) || exit 1

if [ "${#}" -ne 1 ]; then
  echo "Usage: $0 <uint>"
  exit 1
fi

# Sed fallback, because of course BSD sed != GNU sed
SED_CMD=(sed)
if [ "$(uname)" = "Darwin" ]; then
  if ! [ -f $(type -p gsed) ];then
    echo "Install gnu-sed with homebrew or get a propper GNU/Linux OS"
    exit 1
  fi
  SED_CMD=(gsed)
fi

DAY_NUM="$1"

case "$DAY_NUM" in
'' | [!0-9]*)
  echo "Argument must be an integer."
  exit 1
  ;;
*) ;;
esac

DAY="day$DAY_NUM"
INPUTS_DIR="$WD/inputs/$DAY"
CODE_DIR="$WD/src/$DAY"

# Directories
mkdir $INPUTS_DIR
mkdir $CODE_DIR

# Files
touch "$INPUTS_DIR/input.txt"
touch "$INPUTS_DIR/test.txt"
cp "$WD/src/day_template.rs" "$CODE_DIR/mod.rs"

# Add mod and macro call to main.rs
MAIN="$WD/src/main.rs"
"${SED_CMD}" -i '$!N;/^mod /!b;:a;n;/^mod /ba;i mod '"$DAY"';' "$MAIN"
"${SED_CMD}" -i '$!N; :loop; /\n[[:space:]]*solve_day\!/!{ P; D }; s/\([[:space:]]*\)\(solve_day[^[:space:]]*\)/\1\/\/ \2\n\1solve_day!\('"$DAY"');/; P; n; /$/!{ b loop }; n; P; D' "$MAIN"
# NOTE: I spent too much time on the line above. Could not figure out why there was an extra empty line. This solves it
"${SED_CMD}" -i '/^$/ {N; /^\n$/s/^\n$//}' "$MAIN"
