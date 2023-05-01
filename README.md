<div align="center">
  <h1>penguin</h1>

  <img src="https://img.freepik.com/free-vector/cute-baby-penguin-cartoon-vector-icon-illustration-animal-nature-icon-concept-isolated-premium-vector-flat-cartoon-style_138676-3770.jpg?w=2000" wight="200" height="200" />

  <p>
    <strong> Triage bot built to be used inside the Rust Async working group and also be an extensible triage bot written in Async Rust </strong>
  </p>

  <p>
    <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/vincenzopalazzo/penguin/Sanity%20Check%20codebase?style=flat-square">
  </p>

  <h4>
    <a href="https://github.com/vincenzopalazzo/penguin">Project Homepage</a>
  </h4>
</div>

## Table of Content

- Introduction
- How to use


## Introduction

Developed as part of the Rust Async WG while @vincenzopalazzo was onboarding in the async programming in rust concept. The tool is developed as a 
proof of concept for the Async Runtime [Rio](https://github.com/vincenzopalazzo/rio) that is a tiny async runtime developed during the Async WG onboarding.

Why the penguin name?

>The answer to every problem involved penguins. – Rick Riordan

## How to use

There are two simple steps

1. Create your `triage.json` file from the following template:

``` json
{
    "team": "async-wg",
    "git": {
        "owner": "rust-lang",
        "repo": "rust",
        "labels": ["A-async-await"],
        "exclude": [
          "AsyncAwait-Triaged"
        ],
      "since": "2023-04-01T16:14:37.222558823+00:00"
    },
    "hackmd": {
        "token": "<TOKEN>",
        "team": false
    }
}
```
You can generate the hackmd token from the [following docs](https://hackmd.io/@hackmd-api/developer-portal/https:%2F%2Fhackmd.io%2F@hackmd-api%2Fhow-to-issue-an-api-token)


2. Run the following command and start your triage meeting

```
➜  penguin git:(main) ✗ rustup default nightly && make install
➜  penguin git:(macros/fixs) ✗ penguin --config /home/vincent/async-triage.json          
 The last issue triage was 2023-04-01T16:14:37.222558823+00:00
✓ Triage Hackmd available at: https://hackmd.io/@vincenzopalazzo/SkBLw_aQh
```
