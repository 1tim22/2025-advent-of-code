#!/usr/bin/env bash

# https://adventofcode.com/2025
# cURL both the instructions and the unique input for a day's puzzle
# ./setup-puzzle.sh 2 `cat .cookie`
# Where "2" is the day number and ".cookie" is the file where the session cookie is stored for puzzle input
# Hint: snag the session cookie from your web browser!

day=$1
cookie=$2

if [ -n "$day" ]; then
    # Create a Git worktree with branch of "day-$day"
    if [ ! -d "day-$day" ]; then
        git worktree add day-$day
    fi

    cd day-$day

    # Snag the puzzle and convert to Markdown
    curl "https://adventofcode.com/2025/day/$day"\
        -s\
        --globoff\
        -H 'Accept: application/json'\
        | pandoc -f html -t markdown\
        > README.md

    # Snag the unique input for a user specified by a session cookie
    if [ -n "$cookie" ]; then
    curl "https://adventofcode.com/2025/day/$day/input"\
        --compressed\
        -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8'\
        -H 'Accept-Language: en-US,en;q=0.5'\
        -H "Referer: https://adventofcode.com/2025/day/$day"\
        -H "Cookie: session=$2"\
        -s\
        > input.txt
    else
        echo "Please include the session cookie for puzzle input."
    fi

    # Add the puzzle
    git add README.md input.txt
    git -c core.hookspath=/dev/null commit --no-verify -m "Adds day $day puzzle

Adds README.md puzzle
Adds input.txt puzzle input

- https://adventofcode.com/2025/day/$day

To the King of the ages, immortal, invisible, the only God, be honor and glory forever and ever. Amen."
else
    echo "Please specify a day."
fi
