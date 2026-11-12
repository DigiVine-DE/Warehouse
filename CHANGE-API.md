# How to change/adapt the API
In this guide, the steps needed to modify the API are described.

# 1 - Change Specification
The specification is the core of the API. It defines the data model and the endpoints.
Any change has to be initiated by changing the specification.
It is located in its own repository on [GitHub](https://github.com/DigiVine-DE/API-specification).

Either check it out locally or create a branch in the repository.
Then adapt the split yaml configuration.
If you are using a local copy, you also have to bundle the configuration files into a single monolithic openAPI yaml.
The GitHub repository will do this automatically.

# 2 - Generate Stubs
With the openAPI specification, it is possible to generate client and server stubs for many languages.
This project uses the generated Rust server stubs to create a server library containing the models as structs and the endpoints as functions of a single server interface.

To regenerate the stubs with the current specification, use the `libs/generate_server.sh` script.
It pulls the current specification from the GitHub main branch and creates the stubs using the `openapi-generator-cli` docker image.
An installation of Docker is required for this step. 
If no docker is available, the generator can also be installed locally as a java application.

During testing/development, it may be desired to use a different specification file.
This can be achieved by changing the `specification` variable at the top of the generator script to point to a local copy or bundles branch file.
**Important**: Do not forget to undo the change of the `specification` variable after testing/development is finished.

# 3 - Implement Stubs
Now that the server stubs are generated, they can be implemented in the server library.
For every endpoint a function has to be added to the `Server` struct in the `src/api.rs` file.

In our design, these functions are only responsible for locking the corresponding backend, handling any errors, and returning the result of another function within any of the submodules.
These submodules are also located in the `src` directory (e.g. `src/field`).

The function in the submodules is responsible for communicating with the backend.
For most cases, we use the relational `postgres` backend, which is a sub-library located in `libs`.
It is a PostgreSQL database adapter with simple retrieve and update functions.
If an endpoint should use data that is not yet stored or the backend needs to be changed, we have to modify the backend library.

## 4 - (Optional) Changing the Postgres Backend

Changing the PostgreSQL backend consists of two or three steps.
First, a migration has to be written, which describes the (revertible) changes to the database.
Then we can optionally add mock data to the database, which will be initialized at each restart of the test-server.
Finally, we implement the database communication.

### 4.1 - Migration
The migration is a directory in the `migrations` directory.
Here a `up.sql` file describes the changes to the database that has to be performed to bring the database into the state that we want.
Here tables, views, functions, and indices can be created.
The `down.sql` file, on the other hand, describes the steps needed to revert the changes to the database.
Every action of the `up.sql` file has to be undone here.
**Note**: Add any newly created tables to the `diesel.toml` filter.

To manage migrations and make communications with the database more convenient, we use the [diesel](https://diesel.rs) library.
We use the diesel CLI tool to create and manage migrations.
To install it, run:
```bash
cargo install diesel_cli
```

To create a new migration, we have to go into the `libs/postgres` directory and execute:
```bash
cd libs/postgres
diesel migration generate <name_of_migration>
```

The migration has to be run on the development system with a running database, to create a valid schema file, with the following command:

```bash
diesel migration run --database-url postgres://digivine:digivine@localhost:5432/digivine
```
The database should be running at the given connection string.
If no database is yet running on your system, install a PostgreSQL database on your PC or pull a Docker image.
Then create a new database with a development user.
By default, we use `digivine` as database, user, and password.
If different user, password, or database values are used, replace them in the URL string.
You can reset and initialize this database by running the `reset_database.sh` script in the `libs` directory.

### 4.2 - Mock Data

Similarly, a new migration can be added in the `mockdata` subdirectory.
Here only insertion and deletion of mock data should be performed.
This data will be automatically loaded each time the test server is restarted.

### 4.3 - Implementation of Communication
For each new topic/data type in the relational database library, a new submodule has to be added to the src directory.
A `trait` should be implemented that contains all new database communication functions for the given topic.

This trait is then implemented for the `RelationalBackend` struct.
Example implementations can be found in the `quality_measurement` and `field` submodules.

