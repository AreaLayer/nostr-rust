# Contributing to rust-nostr

## Contribution Workflow

Before writing code, please read [our code style](./CODE_STYLE.md).

To contribute a patch:

- Fork Repository 
- Create topic branch 
- Commit patches (PR, emails, ...)

In general commits should be atomic and diffs **easy to read**.

## Commit Style

The commit **must** be formatted as following:

```
<context>: <short descriptrion>

<description explaining reasons for the changes>
```

If applicable, link the `issue`/`PR` to be closed with:

- Closes <url>
- Fixes <url>

The `context` **must be**:

- `nostr` for changes to the `nostr` crate
- `sdk`, `cli`, `relay-pool`, `connect`, `nwc` and so on for the others crates (remote the `nostr-` prefix)
- `test` for changes to the unit tests
- `doc` for changes to the documentation
- `contrib` for changes to the scripts and tools
- `ci` for changes to the CI code
- `refactor` for structural changes that do not change behavior

### Examples

```
nostr: add NIP32 support

Added kinds, tags and EventBuilder constructors to support NIP32.

Closes https://<domain>.com/rust-nostr/nostr/issue/1234
```

```
pool: fix connection issues

Long description...

Fixes https://<domain>.com/rust-nostr/nostr/issue/5612
```

```
nwc: add `pay_multiple_invoices` support

Closes https://<domain>.com/rust-nostr/nostr/issue/2222
```

## Deprecation policy

Where possible, breaking existing APIs should be avoided.
Instead, add new APIs and use 
[`#[deprecated]`](https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md)
to discourage use of the old one.

Deprecated APIs are typically maintained for one release cycle.
In other words, an API that has been deprecated with the 0.10 release 
can be expected to be removed in the 0.11 release.
This allows for smoother upgrades without incurring too much technical debt inside this library.

If you deprecated an API as part of a contribution, we encourage you to "own" that API
and send a follow-up to remove it as part of the next release cycle.

## Coding Conventions

Install https://github.com/casey/just and use `just precommit` or `just check` 
to format and check the code before committing.
This is also enforced by the CI.

### Terminology

Concept ACK - Agree with the idea and overall direction, but haven't reviewed the code changes or tested them.

utACK (untested ACK) - Reviewed and agree with the code changes but haven't actually tested them.

tACK (tested ACK) - Reviewed the code changes and have verified the functionality or bug fix.

ACK - A loose ACK can be confusing. It's best to avoid them unless it's a documentation/comment only change in which case there is nothing to test/verify; therefore the tested/untested distinction is not there.

NACK - Disagree with the code changes/concept. Should be accompanied by an explanation.
