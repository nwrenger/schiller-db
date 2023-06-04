const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");
const permissions = JSON.parse(localStorage.getItem("permissions"));
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
const login_container = document.getElementById("login-container");
const get_user_button = document.getElementsByClassName("get-user");
const addButton = document.getElementById("add");
const editButton = document.getElementById("edit");
const deleteButton = document.getElementById("del");
const cancelButton = document.getElementById("cancel");
const loginCreatorDropdown = document.getElementById("login-creator");
const absenceDropdown = document.getElementById("absence");
const criminalDropdown = document.getElementById("criminal");
var select = "user";
var current_data_user = {};

if (!auth || !current_user || !permissions) {
    window.open("login.html", "_self");
    error("InvalidLocalKeys");
}

function updateDisabling() {
    addButton.disabled = false;
    editButton.disabled = false;
    deleteButton.disabled = false;
    if (permissions.access_absence === "None") {
        absenceDropdown.disabled = true;
    }
    if (permissions.access_criminal === "None") {
        criminalDropdown.disabled = true;
    }
    if (select === "user") {
        if (permissions.access_user === "ReadOnly" || permissions.access_user === "None") {
            addButton.disabled = true;
            editButton.disabled = true;
            deleteButton.disabled = true;
            loginCreatorDropdown.disabled = true;
        }
    } else if (select === "absence") {
        if (permissions.access_absence === "ReadOnly" || permissions.access_absence === "None") {
            addButton.disabled = true;
            editButton.disabled = true;
            deleteButton.disabled = true;
        }
    } else if (select === "criminal") {
        if (permissions.access_criminal === "ReadOnly" || permissions.access_criminal === "None") {
            addButton.disabled = true;
            editButton.disabled = true;
            deleteButton.disabled = true;
        }
    }
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

    if (response.status === 200 && !data["Err"]) {
        return data["Ok"];
    } else {
        error(data["Err"]);
    }
}

/**state[0] = stats_container.
 * state[1] = absence_container.
 * state[2] = criminal_container.
 * state[3] = user_container.
 * state[4] = login_container.
 * state[5] = visibilityGetUser.
 * getUser = enable visibilityGetUser Changer
*/
function show(state, getUser) {
    stats_container.hidden = state[0];
    absence_container.hidden = state[1];
    criminal_container.hidden = state[2];
    user_container.hidden = state[3];
    login_container.hidden = state[4];
    if (getUser) {
        visibilityGetUser(state[5]);
    }
}

// Updates the UI with user data
function updateUserUI(data) {
    show([true, true, true, false, true]);

    forename.value = data.forename;
    surname.value = data.surname;
    account.value = data.account;
    role.value = data.role;
}

// Updates the UI with absence data
function updateAbsenceUI(data) {
    show([true, false, true, true, true, false], true);

    absence_account.value = data.account;
    day.value = data.date;
    time.value = data.time;
}

// Updates the UI with criminal data
function updateCriminalUI(data) {
    show([true, true, false, true, true, false], true);

    criminal_account.value = data.account;
    kind.value = data.kind;
    criminal_data.value = data.data;
}

// Initializes the user list for roles UI
async function roleUserList() {
    const roles = await request("/user/all_roles", "GET");

    clearList();
    for (const role of roles) {
        const node = document.createElement("li");
        const data = document.createTextNode(role);
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const role = this.textContent;
            cancel();

            const users = await request(`/user/search?role=${encodeURIComponent(role)}`, "GET");
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
            sidebarList.textContent = "Keine Ergebnisse!";
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
            cancel();

            const absences = await request(`/absence/search?text=${encodeURIComponent(date)}`, "GET");
            createUserList(absences, sidebarList, true);
        });
    }
}

async function criminalUserList() {
    clearList();

    const kinds = await request("/criminal/all_kinds", "GET");

    if (!Array.isArray(kinds) || !kinds.length) {
        if (!sidebarList.textContent) {
            sidebarList.textContent = "Keine Ergebnisse!";
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
            cancel();

            const criminals = await request(`/criminal/search?text=${encodeURIComponent(kind)}`, "GET");
            createUserList(criminals, sidebarList, true);
        });
    }
}

function createUserList(nestedList, node, back) {
    clearList();

    const backEntry = document.createElement("li");
    if (back) {
        const text = document.createTextNode("Zurück");
        backEntry.className = "list-group-item list-group-item-action list-group-item-danger";
        backEntry.appendChild(text);
        node.appendChild(backEntry);

        backEntry.addEventListener("click", async function () {
            reset();
        })
        document.scrollingElement.scrollTo(0, 0);
    }

    if (!Array.isArray(nestedList) || !nestedList.length) {
        if (back) {
            backEntry.textContent = "Zurück - Keine Ergebnisse!";
        } else {
            sidebarList.textContent = "Keine Ergebnisse!";
        }
        return;
    }

    for (const user of nestedList) {
        const userNode = document.createElement("li");
        const userTextNode = document.createTextNode(user.account);
        userNode.className = "list-group-item list-group-item-action";
        userNode.appendChild(userTextNode);
        node.appendChild(userNode);

        current_data_user = user;

        userNode.addEventListener("click", async function () {
            const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
            if (activeElement !== null) {
                activeElement.classList.remove("active");
            }
            this.classList.add("active");

            allReadOnly(true);

            editButton.hidden = false;
            cancelButton.hidden = false;
            deleteButton.hidden = false;

            hideAllButtons();

            addButton.classList.remove("active");
            editButton.classList.remove("active");

            if (select === "user") {
                updateUserUI(user);
            } else if (select === "absence") {
                updateAbsenceUI(user);
            } else if (select === "criminal") {
                updateCriminalUI(user);
            }
        });
    }
}

function error(error) {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("staticBackdropLabel").textContent = "Fehler"
    document.getElementById("modal-body").textContent = error;
    console.log(error);
    modal.toggle();
    cancel();
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

function currentUser() {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("staticBackdropLabel").textContent = "Info"
    document.getElementById("modal-body").textContent = "Der akutelle Benutzer ist " + current_user;
    modal.toggle();
}

function loginCreator() {
    cancel();
    show([true, true, true, true, false]);
}

async function addLogin() {
    const user = document.getElementById("login-add-user").value;
    const password = document.getElementById("login-add-password").value;
    const user_permissions = document.getElementById("login-add-user-permissions").value;
    const absence_permissions = document.getElementById("login-add-absence-permissions").value;
    const criminal_permissions = document.getElementById("login-add-criminal-permissions").value;
    request("login", "POST", JSON.stringify({ user: user, password: password, access_user: user_permissions, access_absence: absence_permissions, access_criminal: criminal_permissions }))
    cancel();
}

async function deleteLogin() {
    const user = document.getElementById("login-delete-user").value;
    request("login/" + encodeURIComponent(user), "DELETE");
    cancel();
}

function reset() {
    clearList();
    updateDisabling();
    allReadOnly(true);
    hideAllButtons();
    cancel();
    current_data_user = {};
    document.getElementById("search").value = "";
    if (select === "user") {
        roleUserList().catch(() => {
            window.open("login.html", "_self");
            error("InvalidLocalKeys");
        });
        stats().catch(() => {
            window.open("login.html", "_self");
            error("InvalidLocalKeys");
        });
    } else if (select === "absence") {
        absenceUserList();
    } else if (select === "criminal") {
        criminalUserList();
    }
}

function cancel() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (activeElement) {
        activeElement.classList.remove("active");
    }
    addButton.classList.remove("active");
    editButton.classList.remove("active");
    editButton.hidden = true;
    cancelButton.hidden = true;
    deleteButton.hidden = true;
    document.getElementById("criminal-select-button").disabled = true;
    document.getElementById("absence-select-button").disabled = true;
    show([false, true, true, true, true])
}

async function buttonAddUser() {
    userReadOnly(true);
    await request("user", "POST", JSON.stringify({ forename: forename.value, surname: surname.value, account: account.value, role: role.value }))
    reset();
}

async function buttonConfirmUser() {
    userReadOnly(true);
    await request("user/" + encodeURIComponent(current_data_user.account), "PUT", JSON.stringify({ forename: forename.value, surname: surname.value, account: account.value, role: role.value }))
    reset();
}

function formatDate(date) {
    const [year, month, day] = date.split('-');
    return `${year}-${month}-${day}`;
}


async function buttonAddAbsence() {
    document.getElementById("absence-select-button").disabled = true;
    absenceReadOnly(true);
    await request("absence", "POST", JSON.stringify({ account: absence_account.value, date: formatDate(day.value), time: time.value }))
    reset();
}

async function buttonConfirmAbsence() {
    document.getElementById("absence-select-button").disabled = true;
    absenceReadOnly(true);
    await request("absence/" + encodeURIComponent(current_data_user.account) + "/" + encodeURIComponent(current_data_user.date), "PUT", JSON.stringify({ account: absence_account.value, date: formatDate(day.value), time: time.value }))
    reset();
}


async function buttonAddCriminal() {
    document.getElementById("criminal-select-button").disabled = true;
    criminalReadOnly(true);
    await request("criminal", "POST", JSON.stringify({ account: criminal_account.value, kind: kind.value, data: criminal_data.value }))
    reset();
}

async function buttonConfirmCriminal() {
    document.getElementById("criminal-select-button").disabled = true;
    criminalReadOnly(true);
    await request("criminal/" + encodeURIComponent(current_data_user.account) + "/" + encodeURIComponent(current_data_user.kind), "PUT", JSON.stringify({ account: criminal_account.value, kind: kind.value, data: criminal_data.value }))
    reset();
}

function showChange(otherKind, selectId, addId, confirmId) {
    visibilityGetUser(true);
    allReadOnly(false);
    if (selectId) {
        document.getElementById(selectId).disabled = false;
    }
    const buttonAdd = document.getElementById(addId);
    const buttonConfirm = document.getElementById(confirmId);
    if (otherKind === "PUT") {
        buttonAdd.hidden = true;
        buttonConfirm.hidden = false;
    } else if (otherKind === "POST") {
        buttonAdd.hidden = false;
        buttonConfirm.hidden = true;
    }
}

function visibilityGetUser(bool) {
    for (const button of get_user_button) {
        button.hidden = bool;
    }
}

async function getUser() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    activeElement.classList.remove("active");
    const user = await request("user/fetch/" + encodeURIComponent(activeElement.textContent, "GET"));
    cancelButton.hidden = true;
    editButton.hidden = true;
    deleteButton.hidden = true;
    updateUserUI(user);
}

function add() {
    addButton.classList.add("active");
    editButton.classList.remove("active");
    if (select === "user") {
        show([true, true, true, false, true]);
        forename.value = "";
        surname.value = "";
        account.value = "";
        if (!current_data_user.role) {
            role.value = "";
        } else {
            role.value = current_data_user.role;
        }
        showChange("POST", "", "user-add-button", "user-confirm-button");
    } else if (select === "absence") {
        show([true, false, true, true, true, false], true);
        absence_account.value = "";
        if (!current_data_user.date) {
            day.value = "";
        } else {
            day.value = current_data_user.date;
        }
        time.value = "";
        showChange("POST", "absence-select-button", "absence-add-button", "absence-confirm-button");
    } else if (select === "criminal") {
        show([true, true, false, true, true, false], true);
        criminal_account.value = "";
        if (!current_data_user.kind) {
            kind.value = "";
        } else {
            kind.value = current_data_user.kind;
        }
        data.value = "";
        showChange("POST", "criminal-select-button", "criminal-add-button", "criminal-confirm-button");
    }
}

function edit() {
    editButton.classList.add("active");
    addButton.classList.remove("active");
    if (select === "user") {
        forename.value = current_data_user.forename;
        surname.value = current_data_user.surname;
        account.value = current_data_user.account;
        role.value = current_data_user.role;
        showChange("PUT", "", "user-add-button", "user-confirm-button");
    } else if (select === "absence") {
        absence_account.value = current_data_user.account;
        day.value = current_data_user.date;
        time.value = current_data_user.time;
        showChange("PUT", "absence-select-button", "absence-add-button", "absence-confirm-button");
    } else if (select === "criminal") {
        criminal_account.value = current_data_user.account;
        kind.value = current_data_user.kind;
        criminal_data.value = current_data_user.data;
        showChange("PUT", "criminal-select-button", "criminal-add-button", "criminal-confirm-button");
    }
}

async function del() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (select === "user") {
        await request("user/" + encodeURIComponent(activeElement.textContent), "DELETE");
    } else if (select === "absence") {
        await request("absence/" + encodeURIComponent(activeElement.textContent) + "/" + encodeURIComponent(current_data_user.date), "DELETE");
    } else if (select === "criminal") {
        await request("criminal/" + encodeURIComponent(activeElement.textContent) + "/" + encodeURIComponent(current_data_user.kind), "DELETE");
    }
    reset();
}

function allReadOnly(value) {
    userReadOnly(value);
    absenceReadOnly(value);
    criminalReadOnly(value);
}

function userReadOnly(value) {
    forename.readOnly = value;
    surname.readOnly = value;
    account.readOnly = value;
    role.readOnly = value;
}

function absenceReadOnly(value) {
    absence_account.readOnly = value;
    day.readOnly = value;
    time.readOnly = value;
}

function criminalReadOnly(value) {
    criminal_account.readOnly = value;
    kind.readOnly = value;
    criminal_data.readOnly = value;
}

function hideAllButtons() {
    document.getElementById("user-add-button").hidden = true;
    document.getElementById("absence-add-button").hidden = true;
    document.getElementById("criminal-add-button").hidden = true;
    document.getElementById("user-confirm-button").hidden = true;
    document.getElementById("absence-confirm-button").hidden = true;
    document.getElementById("criminal-confirm-button").hidden = true;
}


async function search() {
    const text = encodeURIComponent(document.getElementById("search").value);
    if (select === "user") {
        const data = await request(`/user/search?name=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "absence") {
        const data = await request(`/absence/search?text=${text}`, "GET");
        createUserList(data, sidebarList, true);
    } else if (select === "criminal") {
        const data = await request(`/criminal/search?text=${text}`, "GET");
        createUserList(data, sidebarList, true);
    }
}

async function createSelectList(node, text_field) {
    const data = await request(`/user/search?name=${encodeURIComponent(text_field.value)}`, "GET")

    if (!Array.isArray(data) || !data.length) {
        node.textContent = "No Results!";
        return;
    }
    clearSelect(node);
    for (const user of data.slice(0, 10)) {
        const aUser = document.createElement("a");
        const userTextNode = document.createTextNode(user.account);
        aUser.className = "dropdown-item";
        aUser.appendChild(userTextNode);
        const userNode = document.createElement("li")
        userNode.className = "parent-dropdown-item"
        userNode.appendChild(aUser);
        node.appendChild(userNode);

        userNode.addEventListener("click", async function () {
            text_field.value = this.textContent;
            clearSelect(node);
        })
    }
}

function clearSelect(node) {
    node.textContent = "";
    const items = node.querySelectorAll(".parent-dropdown-item");
    items.forEach(item => item.remove());
}

function nodeSelect(parentId, inputId) {
    const parent = document.getElementById(parentId);
    const input = document.getElementById(inputId);
    clearSelect(parent);
    createSelectList(parent, input);
}

async function stats() {
    const statsData = await request("/stats", "GET");

    const devs = statsData.developer.split(":");

    document.getElementById("name").textContent = statsData.name;
    document.getElementById("version").textContent = statsData.version;
    document.getElementById("devs").textContent = "Programmer/Project Lead " + devs[0] + " und Assistant Lead " + devs[1];
    document.getElementById("repo").textContent = statsData.repo;
    document.getElementById("repo").href = statsData.repo;
    document.getElementById("description").textContent = statsData.description;
    document.getElementById("users").textContent = statsData.users;
}

function selecting(message, which) {
    select = which;
    document.getElementById("select-button").textContent = message;
    const activeElement = document.querySelector(".dropdown-item.active");
    if (activeElement !== null) {
        activeElement.classList.remove("active");
    }
    document.getElementById(which).classList.add("active");
    reset();
}

selecting("Bürger", "user");