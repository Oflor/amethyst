#!/bin/bash

# Publish the book and the API documentation to the `gh-pages' branch.

cargo install mdbook
if [ $? -ne 0 ]; then
    exit
fi

mdbook build book

mkdir web
cp -r book/html/* book/images/ web/
cp -r target/doc/ web/doc/

sudo pip install ghp-import
ghp-import -n web/

git config user.name "Eyal Kalderon"
git config user.email "ebkalderon@gmail.com"
git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
