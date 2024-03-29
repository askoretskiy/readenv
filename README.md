# ReadEnv

Simple program that reads `.env` file and use it to run given command.

Never load environment variables manually or pollute your interpreter profile again. 

## How it works

1. Read current environment variables
2. Read `.env` file in current or parent directory
3. Extend current environment variables with ones read from `.env` file
4. Spawn `<COMMAND>` with generated environment variables
5. Replace current process with spawned one

## Installation

```
cargo install readenv
```

## Usage

1. Create `.env` file in with `<KEY>=<VALUE>` structure in current or parent directory
2. Run the app:

   ```bash
   renv <COMMAND>
   ```
During the run, the `<COMMAND>` acts exactly as executed directly, including environment variables, stdin, stdout, stderr, pipelines support and signals handling.

## .env format

Support of `.env` file is provided by [dotenv](https://github.com/dotenv-rs/dotenv) library. See its [documentation](https://crates.io/crates/dotenv) for the format.

## Recipes

### Django

To run a [Django](https://www.djangoproject.com/) project, a settings file is needed. One approach is to have
different settings file per enviroment (e.g. for development and production).

The easiest way to do that is to define environment variable `DJANGO_SETTINGS_MODULE` with name of the settings module.

Let's create `.env` file (given settings module is `local_settings`):

```bash
echo 'DJANGO_SETTINGS_MODULE=local_settings' >> .env
```

Now run Django server with one command:

```bash
renv django-admin.py runserver
```

### Virtualenv

[Virtualenv](https://virtualenv.pypa.io/en/latest/) is a nice tool to isolate Python dependencies for a project.

To switch to given virtualenv one has to use command `. <VENV>/bin/activate`.

Let's create `.env` file (given virtualenv directory is `.venv`):

```bash
echo "VIRTUAL_ENV=$PWD/.venv" >> .env
echo "PATH=$PWD/.venv/bin:\${PATH}" >> .env
```

Now check Python interpreter:

```bash
renv which python
```

Result should be:

```bash
$PWD/.venv/bin/python
```

Try pip:

```bash
renv pip freeze
```

Result should be the list of dependencies installed in your virtualenv.

## Design considerations

[12-factor App methodology](https://en.wikipedia.org/wiki/Twelve-Factor_App_methodology) is great but could be boring.
A simple tool that automates work with environment variables would be helpful.

That tool should be a drop-in replacement for any app, so it:

* Must support stdin, stdout and stderr
* Could be used in pipelines
* Must handle signals (e.g. `SIGTERM` or `SIGKILL`) identically

Also, it would be nice to:

* Have small application size
* Have low RAM footprint
* Do not depend on shell used
* Be safe

Solution:

* Produce binary program
* Use [Rust](https://www.rust-lang.org/)
* Replace its process with executed command (like `exec` in BASH)

## Kudos

Thanks team of [dotenv](https://github.com/dotenv-rs/dotenv) and [dotenv](https://github.com/allan2/dotenvy) library for the most of work ;-)
