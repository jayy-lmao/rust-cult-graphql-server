### Without Docker compose

You will need to create a `.env` file with the following. 

```sh
DATABASE_URL=
```
This will need to be placed in the project root.
Make sure to migrate the `db/init.sql` file into the postgres database you are using if you arent using docker.

Then `cargo run` to run! 

Lint: `cargo fmt --all`

### With Docker Compose

`docker-compose up`
