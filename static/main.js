var auth = localStorage.getItem("auth");
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
const accuser = document.getElementById("accuser");
const police_consultant = document.getElementById("police-consultant");
const lawyer_culprit = document.getElementById("lawyer-culprit");
const lawyer_accuser = document.getElementById("lawyer-accuser");
const facts = document.getElementById("facts");
const time_of_crime = document.getElementById("time-of-crime");
const location_of_crime = document.getElementById("location-of-crime");
const note = document.getElementById("note");
const verdict = document.getElementById("verdict");
const user_container = document.getElementById("user-container");
const absence_container = document.getElementById("absence-container");
const criminal_container = document.getElementById("criminal-container");
const stats_container = document.getElementById("stats-container");
const login_creator_container = document.getElementById("login-container");
const password_changer_container = document.getElementById("password-changer-container")
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
var current_criminal = "%";
var current_date = "%";

if (!auth || !current_user || !permissions) {
    window.open("/login", "_self");
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
 * state[4] = login_creator_container.
 * state[5] = visibilityGetUser.
 * getUser = visibilityGetUser
*/
function show(state, getUser) {
    stats_container.hidden = state[0];
    absence_container.hidden = state[1];
    criminal_container.hidden = state[2];
    user_container.hidden = state[3];
    login_creator_container.hidden = state[4];
    password_changer_container.hidden = state[5];
    if (getUser) {
        visibilityGetUser(!getUser);
    }
}

// Updates the UI with user data
function updateUserUI(data) {
    show([true, true, true, false, true, true]);

    forename.value = data.forename;
    surname.value = data.surname;
    account.value = data.account;
    role.value = data.role;
}

// Updates the UI with absence data
function updateAbsenceUI(data) {
    show([true, false, true, true, true, true], true);

    absence_account.value = data.account;
    day.value = data.date;
    time.value = data.time;
}

// Updates the UI with criminal data
function updateCriminalUI(data) {
    show([true, true, false, true, true, true], true);

    criminal_account.value = data.account;
    kind.value = data.kind;
    accuser.value = data.accuser;
    police_consultant.value = data.police_consultant;
    lawyer_culprit.value = data.lawyer_culprit;
    lawyer_accuser.value = data.lawyer_accuser;
    facts.value = data.facts;
    time_of_crime.value = data.time_of_crime;
    location_of_crime.value = data.location_of_crime;
    note.value = data.note;
    verdict.value = data.verdict;
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
            document.getElementById("advanced").disabled = true;
            document.getElementById("group-select-dropdown").hidden = true;
            cancel();

            const users = await request(`/user/search?role=${encodeURIComponent(role)}`, "GET");
            createUserList(role, users, sidebarList, true);
        });
    }
}

function decodeFormatDate(date) {
    const [day, month, year] = date.split('.');
    return `${year}-${month}-${day}`;
}

function encodeFormatDate(date) {
    const [year, month, day] = date.split('-');
    return `${day}.${month}.${year}`;
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
        const data = document.createTextNode(encodeFormatDate(date));
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const date = this.textContent;
            current_date = decodeFormatDate(date);
            cancel();

            const absences = await request(`/absence/search?date=${encodeURIComponent(decodeFormatDate(date))}`, "GET");
            createUserList(date, absences, sidebarList, true);
        });
    }
}

async function criminalUserList() {
    clearList();

    const accounts = await request("/criminal/all_accounts", "GET");

    if (!Array.isArray(accounts) || !accounts.length) {
        if (!sidebarList.textContent) {
            sidebarList.textContent = "Keine Ergebnisse!";
        }
        return;
    }

    // Fetch users
    for (const account of accounts) {
        const node = document.createElement("li");
        const data = document.createTextNode(account);
        node.className = "list-group-item list-group-item-action";
        node.appendChild(data);
        sidebarList.appendChild(node);

        node.addEventListener("click", async function () {
            const account = this.textContent;
            document.getElementById("advanced").disabled = true;
            document.getElementById("group-select-dropdown").hidden = true;
            current_criminal = account;
            cancel();

            const criminals = await request(`/criminal/search?name=${encodeURIComponent(account)}`, "GET");
            createUserList(account, criminals, sidebarList, true, true);
        });
    }
}

function createUserList(param, nestedList, node, back, swappedKind) {
    clearList();

    const backEntry = document.createElement("li");
    if (back) {
        const text = document.createTextNode("Zurück - " + param);
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
            backEntry.textContent = "Zurück - " + param + " - Keine Ergebnisse!";
        } else {
            sidebarList.textContent = "Keine Ergebnisse!";
        }
        return;
    }

    for (const user of nestedList) {
        const userNode = document.createElement("li");
        let data = {};
        if (swappedKind) {
            data = user.kind;
        } else {
            data = user.account;
        }
        const userTextNode = document.createTextNode(data);
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

            current_data_user = user;

            editButton.hidden = false;
            cancelButton.hidden = false;
            deleteButton.hidden = false;

            resetAllButtons();

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
    window.open("/login", "_self");
}

function currentUser() {
    const modal = new bootstrap.Modal(document.getElementById("dialog"));
    document.getElementById("staticBackdropLabel").textContent = "Info"
    document.getElementById("modal-body").textContent = "Der akutelle Benutzer ist " + current_user;
    modal.toggle();
}

function loginChanger() {
    cancel();
    show([true, true, true, true, true, false]);
}

async function changePassword() {
    const old_password = document.getElementById("old-password").value;
    const new_password = document.getElementById("new-password").value;
    request("login", "PUT", JSON.stringify({ previous_user: current_user, previous_password: old_password, new_user: current_user, new_password })).then(() => {
        auth = btoa(current_user + ":" + new_password);
        localStorage.removeItem("auth");
        localStorage.setItem("auth", auth);
    });
    cancel();
}

function loginCreator() {
    cancel();
    show([true, true, true, true, false, true]);
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

async function deleteAllLogins() {
    request("all_logins", "DELETE");
    cancel();
}

function reset() {
    clearList();
    updateDisabling();
    allReadOnly(true);
    resetAllButtons();
    cancel();
    current_data_user = {};
    current_date = "%";
    current_criminal = "%";
    document.getElementById("advanced").disabled = false;
    document.getElementById("group-select-dropdown").hidden = false;
    document.getElementById("search").value = "";
    if (select === "user") {
        roleUserList().catch(() => {
            window.open("/login", "_self");
            error("InvalidLocalKeys");
        });
        stats().catch(() => {
            window.open("/login", "_self");
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
    document.getElementById("accuser-select-button").disabled = true;
    document.getElementById("police-consultant-select-button").disabled = true;
    document.getElementById("lawyer-culprit-select-button").disabled = true;
    document.getElementById("lawyer-accuser-select-button").disabled = true;
    document.getElementById("verdict-select-button").disabled = true;
    document.getElementById("absence-select-button").disabled = true;
    show([false, true, true, true, true, true])
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

function buttonAbortUser() {
    userReadOnly(true);
    addButton.classList.remove("active");
    editButton.classList.remove("active");
    resetAllButtons();
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (activeElement === null) {
        cancel();
    } else {
        forename.value = current_data_user.forename;
        surname.value = current_data_user.surname;
        account.value = current_data_user.account;
        role.value = current_data_user.role;
    }
}

async function buttonAddAbsence() {
    absenceReadOnly(true);
    await request("absence", "POST", JSON.stringify({ account: absence_account.value, date: day.value, time: time.value }))
    reset();
}

async function buttonConfirmAbsence() {
    absenceReadOnly(true);
    await request("absence/" + encodeURIComponent(current_data_user.account) + "/" + encodeURIComponent(current_data_user.date), "PUT", JSON.stringify({ account: absence_account.value, date: day.value, time: time.value }))
    reset();
}

function buttonAbortAbsence() {
    absenceReadOnly(true);
    addButton.classList.remove("active");
    editButton.classList.remove("active");
    resetAllButtons();
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (activeElement === null) {
        cancel();
    } else {
        show([true, false, true, true, true, true], true);
        absence_account.value = current_data_user.account;
        day.value = current_data_user.date;
        time.value = current_data_user.time;
    }
}

async function buttonAddCriminal() {
    criminalReadOnly(true);
    await request("criminal", "POST", JSON.stringify({ account: criminal_account.value, kind: kind.value, accuser: accuser.value, police_consultant: police_consultant.value, lawyer_culprit: lawyer_culprit.value, lawyer_accuser: lawyer_accuser.value, facts: facts.value, time_of_crime: time_of_crime.value, location_of_crime: location_of_crime.value, note: note.value, verdict: verdict.value }));
    reset();
}

async function buttonConfirmCriminal() {
    criminalReadOnly(true);
    await request("criminal/" + encodeURIComponent(current_data_user.account) + "/" + encodeURIComponent(current_data_user.kind), "PUT", JSON.stringify({ account: criminal_account.value, kind: kind.value, accuser: accuser.value, police_consultant: police_consultant.value, lawyer_culprit: lawyer_culprit.value, lawyer_accuser: lawyer_accuser.value, facts: facts.value, time_of_crime: time_of_crime.value, location_of_crime: location_of_crime.value, note: note.value, verdict: verdict.value }));
    reset();
}

function buttonAbortCriminal() {
    criminalReadOnly(true);
    addButton.classList.remove("active");
    editButton.classList.remove("active");
    resetAllButtons();
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (activeElement === null) {
        cancel();
    } else {
        show([true, true, false, true, true, true], true);
        criminal_account.value = current_data_user.account;
        kind.value = current_data_user.kind;
        accuser.value = current_data_user.accuser;
        police_consultant.value = current_data_user.police_consultant;
        lawyer_culprit.value = current_data_user.lawyer_culprit;
        lawyer_accuser.value = current_data_user.lawyer_accuser;
        facts.value = current_data_user.facts;
        time_of_crime.value = current_data_user.time_of_crime;
        location_of_crime.value = current_data_user.location_of_crime;
        note.value = current_data_user.note;
        verdict.value = current_data_user.verdict;
    }
}

function showChange(otherKind, selectId, addId, confirmId, abortId) {
    visibilityGetUser(true);
    allReadOnly(false);
    if (selectId) {
        for (const item of selectId) {
            document.getElementById(item).disabled = false;
        }
    }
    const buttonAdd = document.getElementById(addId);
    const buttonConfirm = document.getElementById(confirmId);
    const buttonAbort = document.getElementById(abortId);
    if (otherKind === "PUT") {
        buttonAdd.hidden = true;
        buttonConfirm.hidden = false;
    } else if (otherKind === "POST") {
        buttonAdd.hidden = false;
        buttonConfirm.hidden = true;
    }
    buttonAbort.hidden = false;
}

function visibilityGetUser(bool) {
    for (const button of get_user_button) {
        button.hidden = bool;
    }
}

async function getUser() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    activeElement.classList.remove("active");
    const user = await request("user/fetch/" + encodeURIComponent(current_data_user.account, "GET"));
    cancelButton.hidden = true;
    editButton.hidden = true;
    deleteButton.hidden = true;
    updateUserUI(user);
}

function add() {
    addButton.classList.add("active");
    editButton.classList.remove("active");
    if (select === "user") {
        show([true, true, true, false, true, true]);
        forename.value = "";
        surname.value = "";
        account.value = "";
        if (!current_data_user.role) {
            role.value = "";
        } else {
            role.value = current_data_user.role;
        }
        showChange("POST", "", "user-add-button", "user-confirm-button", "user-abort-button");
    } else if (select === "absence") {
        show([true, false, true, true, true, true], true);
        absence_account.value = "";
        if (!current_data_user.date) {
            day.value = "";
        } else {
            day.value = current_data_user.date;
        }
        time.value = "";
        showChange("POST", ["absence-select-button"], "absence-add-button", "absence-confirm-button", "absence-abort-button");
    } else if (select === "criminal") {
        show([true, true, false, true, true, true], true);
        if (!current_data_user.account) {
            criminal_account.value = "";
        } else {
            criminal_account.value = current_data_user.account;
        }
        kind.value = "";
        accuser.value = "";
        police_consultant.value = "";
        lawyer_culprit.value = "";
        lawyer_accuser.value = "";
        facts.value = "";
        time_of_crime.value = "";
        location_of_crime.value = "";
        note.value = "";
        verdict.value = "";
        showChange("POST", ["criminal-select-button", "accuser-select-button", "police-consultant-select-button", "lawyer-culprit-select-button", "lawyer-accuser-select-button", "verdict-select-button"], "criminal-add-button", "criminal-confirm-button", "criminal-abort-button");
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
        showChange("PUT", "", "user-add-button", "user-confirm-button", "user-abort-button");
    } else if (select === "absence") {
        absence_account.value = current_data_user.account;
        day.value = current_data_user.date;
        time.value = current_data_user.time;
        showChange("PUT", ["absence-select-button"], "absence-add-button", "absence-confirm-button", "absence-abort-button");
    } else if (select === "criminal") {
        criminal_account.value = current_data_user.account;
        kind.value = current_data_user.kind;
        accuser.value = current_data_user.accuser;
        police_consultant.value = current_data_user.police_consultant;
        lawyer_culprit.value = current_data_user.lawyer_culprit;
        lawyer_accuser.value = current_data_user.lawyer_accuser;
        facts.value = current_data_user.facts;
        time_of_crime.value = current_data_user.time_of_crime;
        location_of_crime.value = current_data_user.location_of_crime;
        note.value = current_data_user.note;
        verdict.value = current_data_user.verdict;
        showChange("PUT", ["criminal-select-button", "accuser-select-button", "police-consultant-select-button", "lawyer-culprit-select-button", "lawyer-accuser-select-button", "verdict-select-button"], "criminal-add-button", "criminal-confirm-button", "criminal-abort-button");
    }
}

async function del() {
    const activeElement = document.querySelector(".list-group-item.list-group-item-action.active");
    if (select === "user") {
        await request("user/" + encodeURIComponent(activeElement.textContent), "DELETE");
    } else if (select === "absence") {
        await request("absence/" + encodeURIComponent(activeElement.textContent) + "/" + encodeURIComponent(current_data_user.date), "DELETE");
    } else if (select === "criminal") {
        await request("criminal/" + encodeURIComponent(current_data_user.account) + "/" + encodeURIComponent(activeElement.textContent), "DELETE");
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
    accuser.readOnly = value;
    police_consultant.readOnly = value;
    lawyer_culprit.readOnly = value;
    lawyer_accuser.readOnly = value;
    facts.readOnly = value;
    time_of_crime.readOnly = value;
    location_of_crime.readOnly = value;
    note.readOnly = value;
    verdict.readOnly = value;
}

function resetAllButtons() {
    document.getElementById("user-add-button").hidden = true;
    document.getElementById("absence-add-button").hidden = true;
    document.getElementById("criminal-add-button").hidden = true;
    document.getElementById("user-confirm-button").hidden = true;
    document.getElementById("absence-confirm-button").hidden = true;
    document.getElementById("criminal-confirm-button").hidden = true;
    document.getElementById("criminal-select-button").disabled = true;
    document.getElementById("user-abort-button").hidden = true;
    document.getElementById("absence-abort-button").hidden = true;
    document.getElementById("criminal-abort-button").hidden = true;
    document.getElementById("accuser-select-button").disabled = true;
    document.getElementById("police-consultant-select-button").disabled = true;
    document.getElementById("lawyer-culprit-select-button").disabled = true;
    document.getElementById("lawyer-accuser-select-button").disabled = true;
    document.getElementById("absence-select-button").disabled = true;
    document.getElementById("verdict-select-button").disabled = true;
}

async function search() {
    document.getElementById("advanced").disabled = false;
    document.getElementById("group-select-dropdown").hidden = false;
    cancel();
    resetAllButtons();
    const text = encodeURIComponent(document.getElementById("search").value);
    var data = [];
    if (select === "user") {
        data = await request(`/user/search?name=${text}`, "GET");
    } else if (select === "absence") {
        data = await request(`/absence/search?name=${text}`, "GET");
    } else if (select === "criminal") {
        data = await request(`/criminal/search?name=${text}`, "GET");
    }
    createUserList('"' + text + '"', data, sidebarList, true);
    current_date = "%";
    current_criminal = "%";
}

async function createSelectList(node, text_field) {
    const data = await request(`/user/search?name=${encodeURIComponent(text_field.value)}&limit=${10}`, "GET")

    if (!Array.isArray(data) || !data.length) {
        node.textContent = "Keine Ergebnisse!";
        return;
    }
    clearSelect(node);
    for (const user of data) {
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

async function createAdvancedSelectList(node) {
    var data = [];

    if (select === "user") {
        data = await request(`/user/all_roles`, "GET");
    } else if (select === "absence") {
        data = await request(`/absence/all_roles?date=${current_date}`, "GET");
    } else if (select === "criminal") {
        data = await request(`/criminal/all_roles`, "GET");
    }

    if (!Array.isArray(data) || !data.length) {
        node.textContent = "Keine Ergebnisse!";
        return;
    }
    clearSelect(node);
    for (const group of data) {
        const groupNode = document.createElement("option");
        const groupTextNode = document.createTextNode(group);
        groupNode.value = group;
        groupNode.appendChild(groupTextNode);
        node.appendChild(groupNode);
    }
}

function advancedSelect(parentId) {
    const parent = document.getElementById(parentId);
    clearAdvancedSelect(parent);
    createAdvancedSelectList(parent);
}

function clearAdvancedSelect(node) {
    node.textContent = "";
    const items = node.querySelectorAll(".select");
    items.forEach(item => item.remove());
}

async function handleAdvanced() {
    const parent = document.getElementById("group-select");
    const button = document.getElementById("button-group-select");
    const spinner = document.getElementById("spinner-group-select");
    var text = encodeURIComponent(document.getElementById("search").value);
    button.disabled = true;
    spinner.hidden = false;
    let result = [];
    if (select === "user") {
        result = await request(`/user/search?limit=99999&name=${text}&role=${encodeURIComponent(parent.value)}`, "GET");
    } else if (select === "absence") {
        result = await request(`/absence/search_role?limit=99999&name=${text}&date=${encodeURIComponent(current_date)}&role=${encodeURIComponent(parent.value)}`, "GET");
    } else if (select === "criminal") {
        if (!text) {
            text = encodeURIComponent(current_criminal)
        }
        result = await request(`/criminal/search_role?limit=99999&name=${text}&role=${encodeURIComponent(parent.value)}`, "GET");
    }
    createUserList(parent.value, result, sidebarList, true);
    button.disabled = false;
    spinner.hidden = true;
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

function selectingVerdict(message, which) {
    document.getElementById(which).value = message;
}

selecting("Bürger", "user");