#!/bin/bash

# used in travis to deploy documentation to gh-pages branch

(
cd target/doc
git init
git config user.name "Travis CI"
git config user.email "cholcombe973@users.noreply.github.com"
echo "<meta http-equiv=refresh content=0;url=qemu-rust/index.html>" > index.html
git add .
git commit --quiet -m "Deploy to GH pages."
git push --force --quiet "https://${GH_TOKEN}@github.com/cholcombe973/qemu-rust.git" master:gh-pages &> /dev/null
)
