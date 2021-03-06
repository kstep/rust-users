# rust-users [![Build Status](https://travis-ci.org/ogham/rust-users.svg?branch=master)](https://travis-ci.org/ogham/rust-users)

This is a library for getting information on Unix users and groups. It
supports getting the system users, and creating your own mock tables.

### [View the Rustdoc](http://bsago.me/doc/users/)

## Beta-compatibility

Unfortunately, **rust-users is not compatible with Rust beta**. You'll have to
use the nightly. We're waiting on the following two things to settle:

- [Alternatives to ToOwned](https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md#alternatives-to-toowned-on-entries) in the collections crate
- `as_ref` on C pointers in the core crate

# Installation

This crate, like all external crates, works very well with
[Cargo](http://crates.io/). Add the following to your `Cargo.toml`:

```toml
[dependencies.users]
git = "https://github.com/ogham/rust-users.git"
```

And the `users` crate should be available to you.


# Usage

In Unix, each user has an individual *user ID*, and each process has an
*effective user ID* that says which user's permissions it is using.
Furthermore, users can be the members of *groups*, which also have names
and IDs. This functionality is exposed in libc, the C standard library,
but as an unsafe Rust interface. This wrapper library provides a safe
interface, using User and Group objects instead of low-level pointers
and strings. It also offers basic caching functionality.

It does not (yet) offer *editing* functionality; the objects returned are
read-only.

## Users

The function `get_current_uid` returns a `uid_t` value representing the user
currently running the program, and the `get_user_by_uid` function scans the
users database and returns a User object with the user's information. This
function returns `None` when there is no user for that ID.

A `User` object has the following public fields:

- **uid:** The user's ID
- **name:** The user's name
- **primary_group:** The ID of this user's primary group

Here is a complete example that prints out the current user's name:

```rust
use users::{get_user_by_uid, get_current_uid};
let user = get_user_by_uid(get_current_uid()).unwrap();
println!("Hello, {}!", user.name);
```

This code assumes (with `unwrap()`) that the user hasn't been deleted
after the program has started running. For arbitrary user IDs, this is
**not** a safe assumption: it's possible to delete a user while it's
running a program, or is the owner of files, or for that user to have
never existed. So always check the return values from `user_to_uid`!

There is also a `get_current_username` function, as it's such a common
operation that it deserves special treatment.

## Caching

Despite the above warning, the users and groups database rarely changes.
While a short program may only need to get user information once, a
long-running one may need to re-query the database many times, and a
medium-length one may get away with caching the values to save on redundant
system calls.

For this reason, this crate offers a caching interface to the database,
which offers the same functionality while holding on to every result,
caching the information so it can be re-used.

To introduce a cache, create a new `OSUsers` object and call the same
methods on it. For example:

```rust
use users::{Users, OSUsers};
let mut cache = OSUsers::empty_cache();
let uid = cache.get_current_uid();
let user = cache.get_user_by_uid(uid).unwrap();
println!("Hello again, {}!", user.name);
```

This cache is **only additive**: it's not possible to drop it, or erase
selected entries, as when the database may have been modified, it's best to
start entirely afresh. So to accomplish this, just start using a new
`OSUsers` object.

## Groups

Finally, it's possible to get groups in a similar manner. A `Group` object
has the following public fields:

- **gid:** The group's ID
- **name:** The group's name
- **members:** Vector of names of the users that belong to this group

And again, a complete example:

```rust
use users::{Users, OSUsers};
let mut cache = OSUsers::empty_cache();
let group = cache.get_group_by_name("admin".to_string()).expect("No such group 'admin'!");
println!("The '{}' group has the ID {}", group.name, group.gid);
for member in group.members.into_iter() {
    println!("{} is a member of the group", member);
}
```

## Caveats

You should be prepared for the users and groups tables to be completely
broken: IDs shouldn't be assumed to map to actual users and groups, and
usernames and group names aren't guaranteed to map either!

Use the mocking module to create custom tables to test your code for these
edge cases.
