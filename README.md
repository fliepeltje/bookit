![Rust](https://github.com/fliepeltje/bookit/workflows/Rust/badge.svg?branch=master&event=push)

## What is this?

A CLI tool for keeping track of how your time is spent, specifically in the context of freelancing.


## Installation
You can install the CLI tool with `cargo` by specifying the git flag and referencing this repository.

## Roadmap
The first priority is having a great cli experience and relevant data structures. At present all data structures are fairly minimal. In order of priority I would say:
1. Improved data structures for `Contractor` and `Alias` that hold relevant information
2. Good error handling that does not rely on using `panic!`
3. Pretty terminal outputs that can be filtered based on relevant parameters.

This would be a good version v1.0.

Building on top of that I would like to be able to generate invoices in PDF. What I would want is control in granularity of how much data is displayed. I would also want custom templates for different contractors. This would be v2.0.

The final set of functionality that is on the horizon is generating static sites that integrate with something like Vercel. I would like to be able to share protected URLs with clients so they can track my activities whenever they want. 
