const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");
const sidebarList = document.getElementById("sidebar-list");
const forename = document.getElementById("forename");
const surname = document.getElementById("surname");
const account = document.getElementById("account");
const absence_account = document.getElementById("absence-account");
const criminal_account = document.getElementById("criminal-account");
const role = document.getElementById("role");
const day = document.getElementById("day");
const time = document.getElementById("time");
const kind = document.getElementById("kind");
const criminal_data = document.getElementById("data");
const user_container = document.getElementById("user-container");
const absence_container = document.getElementById("absence-container");
const criminal_container = document.getElementById("criminal-container");
const stats_container = document.getElementById("stats-container");
var select = "User";
var current_date = "";
var current_kind = "";
var current_data_user = {};

if (!auth || !current_user) {
    window.open("login.html", "_self");
}


// Makes Requests from/to the API
async function request(url, type, json) {
    const response = await fetch(url, {
        method: type,
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
        body: json,
    });

    let data = await response.json();

    if (response.status === 200) {
        return data["Ok"];
    } else {
        error(data["Err"])
    }
}

function showUser() {
    stats_container.hidden = true;
    absence_container.hidden = true;
    criminal_container.hidden = true;
    user_container.hidden = false;
}

function showAbsence() {
    stats_container.hidden = true;
    absence_container.hidden = false;
    criminal_container.hidden = true;
    user_container.hidden = true;
}

function showCriminal() {
    stats_container.hidden = true;
    absence_container.hidden = true;
    criminal_container.hidden = false;
    user_container.hidden = true;
}

// Updates the UI with user data
function updateUserUI(data) {
    showUser();

    forename.value = data.forename;
    surname.value = data.surname;
    account.value = data.account;
    role.value = data.role;
}

// Updates the UI with absence data
function updateAbsenceUI(data) {
    showAbsence();

    absence_account.value = data.account;
    day.value = data.date;
    time.value = data.time;
}

// Updates the UI with criminal data
function updateCriminalUI(data) {
    showCriminal();

    criminal_account.value = data.account;
    kind.value = data.kind;
    criminal_data.value = data.data;
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

    const kinds = await request("/criminal/all_kinds", "GET");

    if (!Array.isArray(kinds) || !kinds.length) {
        if (!sidebarList.textContent) {
            sidebarList.textContent = "No Results!";
        }
        return;
    }

    // Fetch users
    for (const kind of kinds) {
        const node = document.createElement("li");
        const data = document.createTextNode(kind);
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const kind = this.textContent;

            current_kind = kind;

            const criminals = await request(`/criminal/search?text=${kind}`, "GET");
            createUserList(criminals, sidebarList, true);
        });
    }
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

            for (const button of document.getElementsByClassName("btn btn-outline-danger m-3")) {
                button.remove();
            }

            document.getElementById("add").classList.remove("active");
            document.getElementById("edit").classList.remove("active");
            document.getElementById("del").classList.remove("active");

            current_data_user = user;
            if (select === "User") {
                updateUserUI(user);
            } else if (select === "Absence") {
                updateAbsenceUI(user);
            } else if (select === "Criminal") {
                updateCriminalUI(user);
            }
        });
    }
}

function error(error) {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("staticBackdropLabel").textContent = "Error"
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

function profile() {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("staticBackdropLabel").textContent = "Info"
    document.getElementById("modal-body").textContent = "The current user account is " + current_user;
    modal.toggle();
}

function loginCreator() {
    error("Currently not Implemented!");
}

function reset() {
    clearList();
    document.getElementById("add").classList.remove("active");
    document.getElementById("edit").classList.remove("active");
    document.getElementById("del").classList.remove("active");
    document.getElementById("edit").hidden = true;
    document.getElementById("del").hidden = true;
    absence_container.hidden = true;
    criminal_container.hidden = true;
    stats_container.hidden = false;
    user_container.hidden = true;
    document.getElementById("search").value = "";
    if (select === "User") {
        roleUserList().catch(() => window.open("login.html", "_self"));
        stats();
    } else if (select === "Absence") {
        absenceUserList();
    } else if (select === "Criminal") {
        criminalUserList();
    }
}

function changeUser(kind, message) {
    forename.readOnly = false;
    surname.readOnly = false;
    account.readOnly = false;
    role.readOnly = false;
    const button = document.createElement("button")
    const textNode = document.createTextNode(message);
    button.className = "btn btn-outline-danger m-3";
    button.id = "change-button"
    button.appendChild(textNode);
    user_container.appendChild(button);
    button.addEventListener("click", function () {
        forename.readOnly = true;
        surname.readOnly = true;
        account.readOnly = true;
        role.readOnly = true;
        request("user", kind, JSON.stringify({forename: forename.value, surname: surname.value, account: account.value, role: role.value}))
        button.remove();
        reset();
    })
    if (document.getElementsByClassName("btn btn-outline-danger m-3").length > 1) {
        for (const button of document.getElementsByClassName("btn btn-outline-danger m-3")) {
            button.remove();
        }
    }
}

function changeAbsence(kind, message) {
    absence_account.readOnly = false;
    day.readOnly = false;
    time.readOnly = false;
    const button = document.createElement("button")
    const textNode = document.createTextNode(message);
    button.className = "btn btn-outline-danger m-3";
    button.id = "change-button"
    button.appendChild(textNode);
    absence_container.appendChild(button);
    button.addEventListener("click", function () {
        absence_account.readOnly = true;
        day.readOnly = true;
        time.readOnly = true;
        request("absence", kind, JSON.stringify({account: absence_account.value, date: day.value, time: time.value}))
        button.remove();
        reset();
    })
    if (document.getElementsByClassName("btn btn-outline-danger m-3").length > 1) {
        for (const button of document.getElementsByClassName("btn btn-outline-danger m-3")) {
            button.remove();
        }
    }
}

function changeCriminal(otherKind, message) {
    criminal_account.readOnly = false;
    kind.readOnly = false;
    criminal_data.readOnly = false;
    const button = document.createElement("button")
    const textNode = document.createTextNode(message);
    button.className = "btn btn-outline-danger m-3";
    button.id = "change-button"
    button.appendChild(textNode);
    criminal_container.appendChild(button);
    button.addEventListener("click", function () {
        criminal_account.readOnly = true;
        kind.readOnly = true;
        criminal_data.readOnly = true;
        request("criminal", otherKind, JSON.stringify({account: criminal_account.value, kind: kind.value, data: criminal_data.value}))
        button.remove();
        reset();
    })
    if (document.getElementsByClassName("btn btn-outline-danger m-3").length > 1) {
        for (const button of document.getElementsByClassName("btn btn-outline-danger m-3")) {
            button.remove();
        }
    }
}

function add() {
    document.getElementById("add").classList.add("active");
    if (select === "User") {
        showUser();
        forename.value = "";
        surname.value = "";
        account.value = "";
        role.value = "";
        changeUser("POST", "Add", "add");
    } else if (select === "Absence") {
        showAbsence();
        absence_account.value = "";
        day.value = "";
        time.value = "";
        changeAbsence("POST", "Add", "add");
    } else if (select === "Criminal") {
        showCriminal();
        criminal_account.value = "";
        kind.value = "";
        data.value = "";
        changeCriminal("POST", "Add", "add");
    }
}

function edit() {
    document.getElementById("edit").classList.add("active");
    if (select === "User") {
        forename.value = current_data_user.forename;
        surname.value = current_data_user.surname;
        account.value = current_data_user.account;
        role.value = current_data_user.role;
        changeUser("PUT", "Confirm", "edit");
    } else if (select === "Absence") {
        absence_account.value = current_data_user.account;
        day.value = current_data_user.date;
        time.value = current_data_user.time;
        changeAbsence("PUT", "Confirm", "edit");
    } else if (select === "Criminal") {
        criminal_account.value = current_data_user.account;
        kind.value = current_data_user.kind;
        criminal_data.value = current_data_user.data;
        changeCriminal("PUT", "Confirm", "edit");
    }
}

function del() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (select === "User") {
        request("user/" + activeElement.textContent, "DELETE");
    } else if (select === "Absence") {
        request("absence/" + activeElement.textContent + "/" + current_date, "DELETE");
    } else if (select === "Criminal") {
        request("criminal/" + activeElement.textContent + "/" + current_kind, "DELETE");
    }
    reset();
}

async function search() {
    const text = document.getElementById("search").value;
    if (select === "User") {
        const data = await request(`/user/search?name=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "Absence") {
        const data = await request(`/absence/search?text=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "Criminal") {
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

function selectAbsence() {
    selecting("Absence", "absence");
    reset();
}

function selectCriminal() {
    selecting("Criminal", "criminal");
    reset();
}

selectUser();