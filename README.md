# Schillernover's Database Management or for Short SNDM

A repo with the server-side backend and the frontend (SNDI, Schillernova's Database Interface) of the Database Application of the Schillerschool for the upcoming project week.

### Test Website

A Test Website can be found under [nils.wrenger.net](http://nils.wrenger.net). A test Login with no changing Permissions is User: alisa.timms with Password: 1. Have fun with testing. Report Bugs using the Issues on this Github repo.

### Download

The latest builds can be downloaded from the [releases page](https://github.com/nwrenger/sndm/releases).

### Usage

Just run the binary/executable file provided in the release. Make sure it's in the same Directory as the dummy data file (benutzer.txt) and admin.env file otherwise it won't start. Run it with sudo/admin permission (because of server port being http://0.0.0.0:80). The path for the Swagger-UI is http://0.0.0.0/swagger-ui/ / http://localhost/swagger-ui/. In addition, using the admin.env file you can define your admin, which can't be deleted. This admin can add other User and their permissions. Without those permissions you are unauthorized and can't interact with the Server/Database.

## Architecture - Including SNDI

This application follows the 3-tier principle.
* **UI Layer:** Getting the Data from Server Calls. It's named SNDI.
* **Application/Server Layer:** This is implemented using Rust and Rocket. Including a Swagger UI integration. It's named SNDM.
* **Database Layer:** The SQLite database that stores the persistent data specific to a project.

### UI Layer - SNDI

Developed by me and a few others (look to contributions). You can see a current state of the development, by visiting a [Test Website](#test-website).

It's developed by using intern Server Calls, JS and Bootstrap (for the UI). I don't use any JS Webdev Framework like React.js (could be a bad idea). The code of [main.js](static/main.js) is really messy and could/should be refactored (won't probably ever done, but when you like to do that Open a Pull Request!).

A Picture of the Main Page:

<img src="images/website.png" alt="Database Schema" width=900/>

### Application/Server Layer - SNDM

This layer is implemented in Rust ([src](src)) and [Rocket](https://rocket.rs) (0.5 rc-3).

It is responsible for the consistency checks and business logic.
This layer also manages the database connection to store and fetch the project data.

Besides that, it also consists the management and logic for a server using [Rocket](https://rocket.rs), including Swagger UI (using the Utoipa Crate).

#### Server

The Server calls are:

- 5 for each data table (user, criminal, absence)
- 2 for logins (create, delete)
- all_roles - to get all roles from user
- stats - getting general statistics/infos

Swagger UI integrated via Utoipa:

<img src="images/server_routes.png" alt="Database Schema" width=500 />

Schemas:

<img src="images/schemas.png" alt="Database Schema" width=500 />

Security:

- User System, an Admin, defined thought the admin.env file
- Admin can add User with Permissions what they can do cannot do like: Reading/Writing for each Data Type (User, Absence, Criminal)
- logging every Server call (excluding Swagger UI) to separate file called 'log.txt' with Information who did what

### Database Layer

The [SQLite](https://sqlite.org/index.html) database has the following schema:

<img src="images/sqlite_dia.png" alt="Database Schema" width=600 />

(The bold printed texts are the primary keys!)

## Current Todo's

- [x] DB Management
- [x] Making a real/openable DB File
- [x] Fetching User Data from IServ
- [x] Server Integration
- [x] Swagger UI Integration
- [x] Testing, Fixing, etc.
- [x] Logging? Why not!
- [x] SideBar -> User
- [x] Searching (including absence, criminal, user)
- [x] Main Input Container -> Stats when nothing and User when one is selected -> include changing and adding them
- [x] Login -> at the current Logout Button make a profile menu
- [x] In profile menu: Logins Creator with permission selection (makes our job a lot easier)
- [x] Absence Management
- [x] Criminal Management
- [x] fix Add Button (currently a little bit buggy)
- [x] Better Criminal/Absence adding process
- [x] Integration for Mobile
- [x] Finished Criminals Extended Data Fields
- [ ] Network test (after we are finished with the UI)
