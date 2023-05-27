const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");

if (!auth || !current_user) {
    window.open("login.html", "_self");
}

// Data Pool
const dataPool = {
    roles: [],
    users: [],
    stats: {}
};

// Fetches data from the API
async function get_data(url) {
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
    });

    let data = await response.json();

    if (response.status === 200) {
        return data["Ok"];
    } else {
        throw(data["Err"])
    }
}

// Fetches and populates the data pool
async function populateDataPool() {
    // Fetch roles
    const roles = await get_data("/user/all_roles");
    dataPool.roles = roles;

    // Fetch users
    for (const role of roles) {
        const users = await get_data(`/user/search?role=${role}`);
        dataPool.users[role] = users;
    }

    // Fetch stats
    const stats = await get_data("/stats");
    dataPool.stats = stats;
}

// Updates the UI with user data
function updateUserUI(data) {
    document.getElementById("forename").value = data.forename;
    document.getElementById("surname").value = data.surname;
    document.getElementById("account").value = data.account;
    document.getElementById("role").value = data.role;
}

// Initializes the user list UI
function initializeUserList() {
    const list = document.getElementById("rolelist");
    const users = document.getElementById("userlist");

    for (const role of dataPool.roles) {
        const node = document.createElement("li");
        const data = document.createTextNode(role);
        node.className = "role";
        node.appendChild(data);
        list.appendChild(node);

        node.addEventListener("click", function () {
            const role = this.textContent;
            document.getElementById("back-button").hidden = false;
            list.hidden = true;

            const userList = dataPool.users[role];
            const userListElement = document.createElement("ul");

            for (const user of userList) {
                const userNode = document.createElement("li");
                const userTextNode = document.createTextNode(user.account);
                userNode.className = "role";
                userNode.appendChild(userTextNode);
                userListElement.appendChild(userNode);

                userNode.addEventListener("click", async function () {
                    const buttons = document.getElementsByClassName("view-button");
                    for (const button of buttons) {
                        button.hidden = false;
                    }

                    const activeElement = document.querySelector(".role.active");
                    if (activeElement !== null) {
                        activeElement.classList.remove("active");
                    }

                    this.classList.add("active");

                    updateUserUI(user);
                });
            }

            users.appendChild(userListElement);
        });
    }
}

// Clears the user list UI
function clearUserList() {
    const userList = document.getElementById("userlist");
    while (userList.firstChild) {
        userList.firstChild.remove();
    }
}

// Event handlers
function logout() {
    localStorage.clear();
    window.open("login.html", "_self");
}

function absence() {
    const error = document.getElementById("error-main");
    error.hidden = false;
    error.textContent = "Absence is not yet implemented!";
}

function criminal() {
    const error = document.getElementById("error-main");
    error.hidden = false;
    error.textContent = "Criminal is not yet implemented!";
}

function back() {
    const roles = document.getElementById("rolelist");
    const backButton = document.getElementById("back-button");
    const error = document.getElementById("error-main");
    roles.hidden = false;
    backButton.hidden = true;
    error.hidden = true;
    clearUserList();
    stats();
}

async function stats() {
    const statsData = dataPool.stats;
    const buttons = document.getElementsByClassName("view-button");
    for (const button of buttons) {
        button.hidden = true;
    }
    document.getElementById("forename").value = statsData.name;
    document.getElementById("surname").value = statsData.version;
    document.getElementById("account").value = statsData.repo;
    document.getElementById("role").value = statsData.description;
}

// Initialize the data pool and UI
populateDataPool()
    .then(() => {
        stats();
        initializeUserList();
    })
    .catch((error) => {
        console.error("Error populating data pool:", error);
    });
