const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");
const sidebarList = document.getElementById("sidebar-list");
var select = "User";
var current_date = "";

if (!auth || !current_user) {
    window.open("login.html", "_self");
}


// Makes Requests from/to the API
async function request(url, type) {
    const response = await fetch(url, {
        method: type,
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
    });

    let data = await response.json();

    if (response.status === 200) {
        return data["Ok"];
    } else {
        error(data["Err"])
    }
}

// Updates the UI with user data
function updateUserUI(data) {
    document.getElementById("stats-container").hidden = true;
    document.getElementById("user-container").hidden = false;

    document.getElementById("forename").value = data.forename;
    document.getElementById("surname").value = data.surname;
    document.getElementById("account").value = data.account;
    document.getElementById("group").value = data.role;
}

// Initializes the user list for roles UI
async function roleUserList() {
    clearList();

    const roles = await request("/user/all_roles", "GET");
    for (const role of roles) {
        const node = document.createElement("li");
        const data = document.createTextNode(role);
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const role = this.textContent;
            // document.getElementById("back-button").hidden = false;

            const users = await request(`/user/search?role=${role}`, "GET");
            createUserList(users, sidebarList, true);
        });
    }
}

// Initializes the user list for the dates
async function absenceUserList() {
    clearList();

    const dates = await request("/absence/all_dates", "GET");

    if (!Array.isArray(dates) || !dates.length) {
        if (!sidebarList.textContent) {
            sidebarList.textContent = "No Results!";
        }
        return;
    }

    // Fetch users
    for (const date of dates) {
        const node = document.createElement("li");
        const data = document.createTextNode(date);
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const date = this.textContent;

            current_date = date;

            const absences = await request(`/absence/search?text=${date}`, "GET");
            createUserList(absences, sidebarList, true);
        });
    }
}

async function criminalUserList() {
    clearList();
    const criminals = await request("/criminal/search", "GET");
    createUserList(criminals, sidebarList, false);
}

function createUserList(list, node, back) {
    clearList();

    const backEntry = document.createElement("li");
    if (back) {
        const text = document.createTextNode("Back");
        backEntry.className = "list-group-item list-group-item-action list-group-item-danger";
        backEntry.appendChild(text);
        node.appendChild(backEntry);

        backEntry.addEventListener("click", async function () {
            reset();
        })
        document.scrollingElement.scrollTo(0,0);
    }

    if (!Array.isArray(list) || !list.length) {
        if (back) {
            backEntry.textContent = "Back - No Results!";
        } else {
            sidebarList.textContent = "No Results!";
        }
        return;
    }
    

    for (const user of list) {
        const userNode = document.createElement("li");
        const userTextNode = document.createTextNode(user.account);
        userNode.className = "list-group-item list-group-item-action";
        userNode.appendChild(userTextNode);
        node.appendChild(userNode);
        
        userNode.addEventListener("click", async function () {
            const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
            if (activeElement !== null) {
                activeElement.classList.remove("active");
            }
            this.classList.add("active");

            document.getElementById("edit").hidden = false;
            document.getElementById("del").hidden = false;

            if (user.role) {
                updateUserUI(user);
            } else {
                const current_user = await request("user/fetch/" + user.account, "GET");
                updateUserUI(current_user);
            }
        });
    }
}

function error(error) {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("modal-body").textContent = error;
    console.log(error);
    modal.toggle();
    throw error;
}

// Clears the user list UI
function clearList() {
    while (sidebarList.firstChild) {
        sidebarList.firstChild.remove();
    }
}

// Event handlers
function logout() {
    localStorage.clear();
    window.open("login.html", "_self");
}

function absenceButton() {
    error("Currently not Implemented!");
}

function criminalsButton() {
    error("Currently not Implemented!");
}

function profile() {
    error("Currently not Implemented!");
}

function loginCreator() {
    error("Currently not Implemented!");
}

function reset() {
    clearList();
    document.getElementById("edit").hidden = true;
    document.getElementById("del").hidden = true;
    document.getElementById("stats-container").hidden = false;
    document.getElementById("user-container").hidden = true;
    document.getElementById("search").value = "";
    if (select === "User") {
        roleUserList().catch(() => window.open("login.html", "_self"));
        stats();
    } else if (select === "Absences") {
        absenceUserList();
    } else if (select === "Criminals") {
        criminalUserList();
    }
}

function add() {
    if (select === "User") {
        error("Currently not Implemented!")
    } else if (select === "Absences") {
        error("Currently not Implemented!")
    } else if (select === "Criminals") {
        error("Currently not Implemented!")
    }
}

function edit() {
    if (select === "User") {
        error("Currently not Implemented!")
    } else if (select === "Absences") {
        error("Currently not Implemented!")
    } else if (select === "Criminals") {
        error("Currently not Implemented!")
    }
}

function del() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (select === "User") {
        request("user/" + activeElement.textContent, "DELETE");
    } else if (select === "Absences") {
        request("absence/" + activeElement.textContent + "/" + current_date, "DELETE");
    } else if (select === "Criminals") {
        request("criminal/" + activeElement.textContent, "DELETE");
    }
    reset();
}

async function search() {
    const text = document.getElementById("search").value;
    if (select === "User") {
        const data = await request(`/user/search?name=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "Absences") {
        const data = await request(`/absence/search?text=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "Criminals") {
        const data = await request(`/criminal/search?text=${text}`, "GET");
        createUserList(data, sidebarList, true);
    }
}

async function stats() {
    const statsData = await request("/stats", "GET");

    const devs = statsData.developer.split(":");

    document.getElementById("name").textContent = statsData.name;
    document.getElementById("version").textContent = statsData.version;
    document.getElementById("devs").textContent = devs[0] + " and " + devs[1];
    document.getElementById("repo").textContent = statsData.repo;
    document.getElementById("repo").href = statsData.repo;
    document.getElementById("description").textContent = statsData.description;
    document.getElementById("users").textContent = statsData.users;
}

function selecting(message, which) {
    select = message;
    document.getElementById("select-button").textContent = select;
    const activeElement = document.querySelector(".dropdown-item.active");
    if (activeElement !== null) {
        activeElement.classList.remove("active");
    }
    document.getElementById(which).classList.add("active");
}

function selectUser() {
    selecting("User", "user");
    reset();
}

function selectAbsences() {
    selecting("Absences", "absences");
    reset();
}

function selectCriminals() {
    selecting("Criminals", "criminals");
    reset();
}

selectUser();