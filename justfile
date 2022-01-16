test:
    cargo test

build:
    cargo build

check:
    #!/bin/bash

    branch=$(git rev-parse --abbrev-ref HEAD)
    printf 'Current branch is `%s`.\n' "$branch"


    remote="origin/$branch"
    exists=$(git branch -vv --format '%(upstream:short)' | grep "$remote")
    if [ -z "$exists" ]; then
        printf "Current branch doesn't have a upstream origin!\n"
        exit 1
    fi

    hash=$(git log "$remote" --oneline | awk '{ print $1 }' | head -n 1)


    printf 'Remote is at %s. Checking each commit until the origin.\n' "$remote"
    hist=$(git log --oneline | awk '{print $1 }')
    while IFS= read -r curr; do
        if [[ "$curr" == "$hash" ]]; then
            echo "Sucessfully finished."
            exit 0
        fi
        printf 'Checking `%s`...' "$curr"
        cargo check -q 2> /dev/null || exit 1 && printf " done!\n"
    done <<< "$hist"