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
github-latest 0.4.1
~~~

~~~text
$ github-latest -h
Get latest tag(s) for GitHub repositories

Usage: github-latest [OPTIONS] [REPO]...

Arguments:
  [REPO]...  One or more GitHub repositories (`qtfkwk/github-latest`)

Options:
  -e <EXCLUDE>      Exclude tags with [default: rc,pre,canary]
  -a                Show all tags (on the first tags page)
  -q                Quiet mode; just show the latest tag(s)
  -h, --help        Print help
  -V, --version     Print version
~~~

# Examples

~~~text
$ github-latest qtfkwk/github-latest
| Repository           | Latest |
|----------------------|--------|
| qtfkwk/github-latest | 0.4.0  |

~~~

~~~text
$ github-latest qtfkwk/github-latest -a
| Repository           | Latest                                                               |
|----------------------|----------------------------------------------------------------------|
| qtfkwk/github-latest | 0.4.0, 0.3.2, 0.3.1, 0.3.0, 0.2.4, 0.2.3, 0.2.2, 0.2.1, 0.2.0, 0.1.1 |

~~~

~~~text
$ github-latest qtfkwk/github-latest -q
0.4.0
~~~

# Changelog

* 0.1.0 (2024-01-18): Initial release
    * 0.1.1 (2024-01-18): Remove `scraper` dependency
* 0.2.0 (2024-01-24): Pull the `/tags` page instead of the main repo page; URL decoding via
  [`urlencoding`]; `-e` and `-a` options; improve doc; update dependencies
    * 0.2.1 (2024-03-11): Update dependencies
    * 0.2.2 (2024-07-26): Fix makefile; update dependencies
    * 0.2.3 (2024-10-18): Update dependencies
    * 0.2.4 (2024-12-04): Update dependencies; add commit target to makefile
* 0.3.0 (2024-12-09): Add `-q` option; update dependencies
    * 0.3.1 (2025-02-21): Update dependencies
    * 0.3.2 (2025-04-16): Update dependencies
* 0.4.0 (2025-08-28): Update dependencies; 2024 edition
    * 0.4.1 (2025-10-27): Update dependencies

[`urlencoding`]: https://crates.io/crates/urlencoding

