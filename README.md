# Stagnant

Stagnant is a hackable static site generator. It allows you to create templates in the way that websites work by default: through HTML! The posts are written in markdown and can all be stored in a `posts` directory.

## Info
Stagnant is currently under heavy development. Expect a lot of breaking changes and nightly [builds](https://github.com/legoraft/stagnant/releases/tag/nightly) that aren't working correctly. Currently stagnant only works very barebones. You can use only html and add a posts list to the index. Adding [CSS](https://github.com/legoraft/stagnant/issues/4) and adding more modularity are all features that are being worked on to get to a full release as fast as possible.

## Usage
You can publish your site through github pages by making a repository that contains a `posts` and `template` repository. Just add the [workflow](https://github.com/legoraft/legoraft.github.io/blob/main/.github/workflows/stagnant.yml) and start writing. You can also run stagnant locally and just push the `site` build directory to a repository or host the html on your own.
