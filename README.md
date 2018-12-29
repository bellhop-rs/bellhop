Bellhop
=======

**Bellhop is a web application for reserving different assets, like machines or IP addresses**

---

## Introduction

### Purpose

Are you part of a team of ten that shares three testing environments? Do you keep a spreadsheet of
who's using what IP addresses? Need a way to keep track of who's borrowing which virtual machine?

If you answered yes to any of those questions, Bellhop might be able to help. Bellhop is a super
simple asset reservation tool that keeps track of who's using what, until when.

For more information, including a live demo, checkout [bellhop.rs][homepage].

[homepage]: https://bellhop.rs

### Project Structure

 * `bellhop/`               - The core library that implements most of Bellhop's features.
 * `bellhop-bin/`           - A runnable example that demos adding hooks and starting the server.
 * `bellhop-hook-email/`    - A hook that sends an email when your lease is expiring.
 * `bellhop-hook-jenkins/`  - A hook that starts a Jenkins job for each event.
 * `bellhop-auth-dummy/`    - Authentication plugin that only requires an email address.
 * `bellhop-auth-header/`   - Authentication plugin that creates users based on a header.

### Design

Bellhop is designed to be heavily customized and installed internally. Additional functionality
can be added by writing hooks (like `bellhop-hook-jenkins`.) Authentication is customizable as
well, with pluggable modules like `bellhop-auth-header`.

## Usage

At this point, Bellhop is more like a set of crates you assemble yourself into a web application
than a web application itself.

A good starting point is getting `bellhop-bin` running, then using that as an example to build
your own deployment.

### First Time Setup

The following instructions are roughly tailored for Ubuntu, but should be relatively similar
regardless of the platform. Bellhop is regularly compiled for Ubuntu and OS X, and should compile
on Windows as well.

#### Installing Rust

The project requires the latest Rust nightly. You can install that with [rustup][rustup]:

```bash
$ curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
```

[rustup]: https://www.rust-lang.org/tools/install

#### Installing PostgreSQL

Bellhop uses [PostgreSQL][postgresql] as its database. Any version greater than
or equal to 9.5 should work.

```
# Install the server and client libraries.
$ sudo apt install postgresql-9.5 postgresql-client libpq-dev

# Create a user, identified with a password.
$ sudo -u postgres createuser -P bellhop

# Create a new database owned by the same user.
$ sudo -u postgres createdb -O bellhop bellhop
```

[postgresql]: https://www.postgresql.org/


#### Installing Diesel

To perform database migrations, you'll need to install `diesel_cli`:

```bash
$ cargo install diesel_cli --no-default-features --features postgres
```

#### Obtain Bellhop

The easiest way to grab all the components is to clone the Bellhop repository:

```bash
$ git clone https://github.com/bellhop-rs/bellhop
```

#### Perform Migrations

The core application, as well as some hooks, will have database migrations that need to be applied
for the application to function.

```bash
# Make sure to update this line with the password supplied earlier.
$ export DATABASE_URL=postgres://bellhop:bellhop@localhost/bellhop

# Apply the core application's migrations.
$ cd bellhop
$ diesel migration run

# Apply any hook's migrations.
$ cd ../bellhop-hook-jenkins
$ diesel migration run
```

#### Run Bellhop, Finally

To run Bellhop in "demo" mode:

```bash
$ cd bellhop-bin
$ cargo run
```

Note that `bellhop-bin` doesn't require any kind of passwords to log in. It really isn't suitable
for production use without some customization.

## License

Bellhop is licensed under:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE.md) or
   http://www.apache.org/licenses/LICENSE-2.0)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
Bellhop by you, as defined in the Apache-2.0 license, shall be licensed as above, without any
additional terms or conditions.

[//]: # ( vim: set tw=98 : )
