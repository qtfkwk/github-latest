# About

Get latest release tag(s) for GitHub repositories

This utility pulls the GitHub repository tags webpage via the [`reqwest`] asynchronous API.
It does not pull subsequent pages, so it only presents the latest tags appearing on the first page.
By default tags containing `rc`, `pre`, or `canary` are ignored, and only the first / latest tag is
shown, but these behaviors can be modified via the `-e` and `-a` options, respectively.

If you need a more general / flexible utility, please try [`git ls-remote -t REPO`], which can query
remote git repositories at GitHub and elsewhere via any supported protocol, and will print all tags. 

[`reqwest`]: https://crates.io/crates/reqwest
[`git ls-remote -t REPO`]: https://git-scm.com/docs/git-ls-remote.html

# Usage

~~~text
$ github-latest -V
!run:../target/release/github-latest -V 2>&1
~~~

~~~text
$ github-latest -h
!run:../target/release/github-latest -h 2>&1
~~~

# Examples

~~~text
$ github-latest qtfkwk/github-latest
!run:../target/release/github-latest qtfkwk/github-latest 2>&1
~~~

~~~text
$ github-latest qtfkwk/github-latest -a
!run:../target/release/github-latest qtfkwk/github-latest -a 2>&1
~~~

!inc:../CHANGELOG.md

