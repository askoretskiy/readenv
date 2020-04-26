# ReadEnv

Simple program that reads `.env` file and use it to run given command.

Never load environment variables manually or pollute your interpreter profile again. 

## How it works

1. Read current environment variables
2. Read `.env` file in current directory
3. Extend current environment variables with ones read from `.env` file:
   * Environment variables take precedence over ones defined in `.env`
4. Spawn `<COMMAND>` with generated environment variables
5. Replace current process with spawned one

## Installation

```
cargo install readenv
```

## Usage

1. Create `.env` file in with `<KEY>=<VALUE>` structure.
2. Run the app:

   ```bash
   renv <COMMAND>
   ```
During the run, the `<COMMAND>` acts exactly as executed directly, including ENV variables, stdin, stdout, stderr, pipelines support and signals handling.

## Recipes

### Django

To run [Django](https://www.djangoproject.com/) project, a settings file is needed. One approach is to have
different settings file per enviroment (e.g. for development and production).

The easiest way to define that is to define environment variable `DJANGO_SETTINGS_MODULE` with name of the settings module.

Let's create `.env` file (given module name is `local_settings`):

```bash
echo 'DJANGO_SETTINGS_MODULE=local_settings' >> .env
```

Now run Django server with one command:

```bash
renv django-admin.py runserver
```

## Design considerations

[12-factor App methodology](https://en.wikipedia.org/wiki/Twelve-Factor_App_methodology) is great but could be boring.
A simple tool that automates work with environment variables would be helpful.

That tool should be a drop-in replacement for any app, so it:

* Must support stdin, stdout and stderr
* Could be used in pipelines
* Must handle signals (e.g. `SIGTERM` or `SIGKILL`) identically

Also, it would be nice to:

* Have application size
* Have low RAM footprint
* Do not depend on shell used
* Be safe

Solution:

* Produce binary program
* Use [Rust](https://www.rust-lang.org/)
* Replace its process with executed command (like `exec` in BASH)
