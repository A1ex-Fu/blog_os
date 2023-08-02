#!/bin/bash
echo "gitting it"

if [ "$#" -ne 1 ]; then
   echo "Usage: ./gitit.sh <commit message>"
   exit 1
fi

COMMIT_MESSAGE="$1"

git add *
git status
git commit -m "$COMMIT_MESSAGE"
git push
