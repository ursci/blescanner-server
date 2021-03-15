# BLE Scanner Server
[![CI](https://github.com/ursci/blescanner-server/actions/workflows/main.yml/badge.svg)](https://github.com/ursci/blescanner-server/actions/workflows/main.yml)

Server application of the "BLE Scanner" project.

This is a very first version of "BLE Scanner", a project lead by [Urban Sciences Lab](https://urbansciences.jp/).

## How to start the server
### Requirements
[rustc](https://www.rust-lang.org/tools/install) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) need to be installed. Cargo is expected to build and test with the current stable, beta, and nightly [releases](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html).

First, you have to prepare environment variables via .env file. This repository has a .env.example file. You can create the .env file copying this file.

FIY, you can modify each values in .env file, especially DB_PASS.

```bash
$ cp example.env .env
$ vim .env
```

Then, you build a docker image and start containers (API server and postgreSQL).

```bash
$ docker-compose build
$ docker-compose up -d
```

And you can check the process with command below.

```bash
$ docker-compose logs -f
```

Once you have confirmed each startup process, setup the migration tool in the container with this command.

And make sure you need setup test database for testings.

```bash
$ docker-compose run api /bin/bash
$ diesel setup
$ diesel setup --database_url <TEST_DATABASE_URL>
```

## How to develop the API server
See above section for the requirement and confirm that the Docker build is finished and processes are running, you can start the development.

Start and shutdown the docker process with these commands below.

```bash
docker-compose up
```

```bash
docker-compose down
```

## License

This program is free software. See [LICENSE](LICENSE) for more information.
