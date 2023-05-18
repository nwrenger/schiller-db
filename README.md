# Schillernova's Database Managment or for short SNDM

A repo with the Server-side Backend for the SNDI (Schillernova's Database Interface).

### Download

The latest builds can be downloaded from the [releases page](https://github.com/nwrenger/sndm/releases).

### Usage

Just run the binary/exceutable file provided in the release. Make sure it's in the same Diractory as the dummy data file (benutzer.txt) and admin.env file otherwise it won't start. Run it with sudo/admin permission (because of server port being http://0.0.0.0:80). The path for the Swagger-UI is http://0.0.0.0/swagger-ui/ / http://localhost/swagger-ui/. In addition, using the admin.env file you can define your admin, which can't be deleted. This admin can add other User and their permissions. Without those permissions you are unauthorised and can't interact with the Server/Database.

## Architecture - Including SNDI

This application follows the 3-tier principle.
* **UI Layer:** Developed by my friend. Getting the Data from Server Calls. It's named SNDI.
* **Application/Server Layer:** This is implemented using Rust and Rocket. Including a Swagger UI integration. It's named SNDM.
* **Database Layer:** The SQLite database that stores the persistent data specific to a project.

### UI Layer - SNDI

Developed by [BoettcherDasOriginal](https://github.com/BoettcherDasOriginal). Look out there for further explainations.

### Application/Server Layer - SNDM

This layer is implemented in Rust ([src](src)) and [Rocket](https://rocket.rs) (0.5 rc-3).

It is responsible for the consistency checks and business logic.
This layer also manages the database connection to store and fetch the project data.

Besides that, it also consists the managment and logic for a server using [Rocket](https://rocket.rs), including Swagger UI (using the Utoipa Crate).

#### Server

The Server calls are:

- 5 for each data table (user, criminal, absence)
- stats - getting general statistics
- info

Swagger UI integrated via Utoipa:

<img src="images/server_routes.png" alt="Database Schema" width=500 />

Schemas:

<img src="images/schemas.png" alt="Database Schema" width=500 />

Security:

- User System, an Admin, definded throught the admin.env file
- Admin can add User with Permissions what they can do cannot do like: Reading/Writing for each Data Type (User, Absence, Criminal)
- logging every Server call (excluding Swagger UI) to seperate file called 'log.txt' with Information who did what

### Database Layer

The [SQLite](https://sqlite.org/index.html) database has the following schema:

<img src="images/sqlite_dia.png" alt="Database Schema" width=600 />

(The bold printed texts are the primary keys!)

## Current Todos

- [x] DB Managment
- [x] Making a real/openable DB File
- [x] Fetching User Data from IServ
- [x] Server Integration
- [x] Swagger UI Integration
- [x] Testing, Fixing, etc.
- [x] Logging? Why not!
- [ ] Network test (After [BoettcherDasOriginal](https://github.com/BoettcherDasOriginal) is finished with the UI)
