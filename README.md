# An async API built with [rocket](https://rocket.rs)
Exploratory project that exposes a `GET` endpoint which acts as an intermediary layer for fetching a `Postgres` database.

# Testing
The `docker-compose.yml` set up the development environment for testing.
It creates the table `rocket` containing the following attributes: `id`, `name`, `launch_date`.
The project uses [Flyway](https://flywaydb.org) for migrations.

# Request
The `/launch/<launch_date>` endpoint queries the `rocket` table and returns the rocket `name`s
which were launched later than the `launch_date` in the request form.

# Response
The endpoint outputs a list of objects as key value pairs, e.g.:

```json
[
  {
    "name": "Falcon 9"
  },
  {
    "name": "H-3"
  }
]
```