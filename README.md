# BLE Scanner Server

Server application of the "BLE Scanner" project.

This is a very first version of "BLE Scanner", a project lead by [Urban Sciences Lab](https://urbansciences.jp/).

## How to run the server
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

Once you have confirmed each startup process, run the migration in the container.

```bash
$ diesel setup
```

### Requirements
[TBD]

## How to develop the server
[TBD]

## License

This program is free software. See [LICENSE](LICENSE) for more information.
