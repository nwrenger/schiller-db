const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");

if (!auth || !current_user) {
    window.open("login.html", "_self");
}

var errorAbsenceList = "Nothing here Yet!"
var errorCriminalList = "Nothing here Yet!"

// Data Pool
const dataPool = {
    roles: [],
    users: [],
    absences: [],
    criminals: [],
    dates: [],
    stats: {}
  };

const nestedList = document.getElementById("nested-list");
const userList = document.getElementById("user-list");

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
        throw (data["Err"])
    }
}

// Fetches and populates the user data pool
async function populateUserDataPool() {
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

// Fetches and populates the user data pool
async function populateAbsenceDataPool() {
  // Fetch absences
  const dates = await get_data("/absence/all_dates");
  dataPool.dates = dates;

  for (const date of dates) {
    const absences = await get_data(`/absence/search?text=${date}`);
    dataPool.absences[date] = absences;
  }
}

// Fetches and populates the user data pool
async function populateCriminalDataPool() {
  // Fetch criminals
  const criminals = await get_data("/criminal/search");
  dataPool.criminals = criminals;
}

// Updates the UI with user data
function updateUserUI(data) {
    document.getElementById("forename").value = data.forename;
    document.getElementById("surname").value = data.surname;
    document.getElementById("account").value = data.account;
    document.getElementById("role").value = data.role;
}

// Initializes the user list for roles UI
function roleUserList() {
    for (const role of dataPool.roles) {
        const node = document.createElement("li");
        const data = document.createTextNode(role);
        node.className = "entry";
        node.appendChild(data);
        nestedList.appendChild(node);

        node.addEventListener("click", function () {
            const role = this.textContent;
            document.getElementById("back-button").hidden = false;
            nestedList.hidden = true;
            userList.hidden = false;

            const users = dataPool.users[role];
            createUserList(users, userList);
        });
    }
}

// Initializes the user list for the dates
async function absenceUserList() {
    if (!Array.isArray(dataPool.dates) || !dataPool.dates.length) {
        if (!nestedList.textContent) {
            nestedList.textContent = errorAbsenceList;
        }
        return;
    }
    
    // Fetch users
    for (const date of dataPool.dates) {
        const node = document.createElement("li");
        const data = document.createTextNode(date);
        node.className = "entry";
        node.appendChild(data);
        nestedList.appendChild(node);
        
        node.addEventListener("click", async function () {
            const date = this.textContent;
            document.getElementById("back-button").hidden = false;
            nestedList.hidden = true;
            userList.hidden = false;
            
            const absences = dataPool.absences[date];
            for (const account of absences) {
                const role = await get_data("/user/fetch/" + account.account);
                createUserList(absences, userList, role.role + "/", errorAbsenceList);
        }});
    }
}


function createUserList(list, node, extra, error) {
    const userListElement = document.createElement("ul");
    
    if (!extra) {
        extra = "";
    }

    if (!Array.isArray(list) || !list.length) {
        if (!userList.textContent && error) {
            userList.textContent = error;
        }
        return;
    }

    for (const user of list) {
        const userNode = document.createElement("li");
        const userTextNode = document.createTextNode(extra + user.account);
        userNode.className = "entry";
        userNode.appendChild(userTextNode);
        userListElement.appendChild(userNode);

        userNode.addEventListener("click", async function () {
            document.getElementById("input-mask").style.display = "flex";
            document.getElementById("stats-container").style.display = "none";

            const activeElement = document.querySelector(".entry.active");
            if (activeElement !== null) {
                activeElement.classList.remove("active");
            }

            this.classList.add("active");
            if (user.role) {
                updateUserUI(user);
            } else {
                const current_user = await get_data("user/fetch/" + user.account)
                    .catch((error) => {
                        console.log("Error on fetching User:", error);
                        error.hidden = false;
                        error.textContent = error;
                    });
                updateUserUI(current_user);
            }
        });
        node.appendChild(userListElement);
    }
}

// Clears the user list UI
function clearUserList() {
    while (userList.firstChild) {
        userList.firstChild.remove();
    }
}

// Clears the roles list UI
function clearNestedList() {
    while (nestedList.firstChild) {
        nestedList.firstChild.remove();
    }
}

// Event handlers
function logout() {
    localStorage.clear();
    window.open("login.html", "_self");
}

function absenceButton() {
    const error = document.getElementById("error-main");
    error.hidden = false;
    error.textContent = "Absence is not yet implemented!";
}

function criminalsButton() {
    const error = document.getElementById("error-main");
    error.hidden = false;
    error.textContent = "Criminal is not yet implemented!";
}

function back() {
    reset();
    select();
}

function reset() {
    const search = document.getElementById("search");
    const backButton = document.getElementById("back-button");
    const error = document.getElementById("error-main");
    search.value = "";
    document.getElementById("input-mask").style.display = "none";
    backButton.hidden = true;
    error.hidden = true;
    clearNestedList();
    clearUserList();
    stats();
}

async function search() {
    const text = document.getElementById("search").value;
    const select = document.getElementById("search-select").value;
    userList.hidden = false;
    if (select === "") {
        const data = await get_data(`/user/search?name=${text}`)
            .catch((error) => {
                console.log("Error on search User:", error);
                userList.textContent = error;
            });
        defaultSearch(data);
    } else if (select === "absence") {
        const data = await get_data(`/absence/search?text=${text}`)
            .catch((error) => {
                console.log("Error on search User:", error);
                userList.textContent = error;
            });
        defaultSearch(data);
    } else if (select === "criminals") {
        const data = await get_data(`/criminal/search?text=${text}`)
            .catch((error) => {
                console.log("Error on search User:", error);
                userList.textContent = error;
            });
        defaultSearch(data);
    }
}

function defaultSearch(data) {
    const backButton = document.getElementById("back-button");
    backButton.hidden = false;
    clearUserList();
    nestedList.hidden = true;
    if (!Array.isArray(data) || !data.length) {
        userList.textContent = "Nothing Found!";
        return;
    }
    createUserList(data, userList);
}

async function stats() {
    const statsData = dataPool.stats;

    document.getElementById("stats-container").style.display = "flex";

    const devs = statsData.developer.split(":");

    document.getElementById("stat1").textContent = statsData.name;
    document.getElementById("stat2").textContent = statsData.version;
    document.getElementById("stat3").textContent = devs[0] + " and " + devs[1];
    document.getElementById("stat4").textContent = statsData.repo;
    document.getElementById("stat4").href = statsData.repo;
    document.getElementById("stat5").textContent = statsData.description;
    document.getElementById("stat6").textContent = statsData.users;
}

function select() {
    var select = document.getElementById("search-select").value;
    if (select === "") {
        normal();
    } else if (select === "absence") {
        absence();
    } else if (select === "criminals") {
        criminals();
    }
}

function normal() {
    reset();
    roleUserList();
    document.getElementById("search-select").value = "";
    nestedList.hidden = false;
    userList.hidden = true;
}

async function absence() {
    reset();
    absenceUserList()
    nestedList.hidden = false;
    userList.hidden = true;
}

async function criminals() {
    reset();
    createUserList(dataPool.criminals, userList, null, errorCriminalList);
    nestedList.hidden = true;
    userList.hidden = false;
}

// Initialize the user data pool and default UI
populateUserDataPool()
    .then(() => {
        stats();
        normal();
    })
    .catch((error) => {
        window.open("login.html", "_self");
        console.error("Error populating data pool:", error);
    });
// Initialize the absence data pool
populateAbsenceDataPool()
    .catch((error) => {
        console.error("Error populating absence data pool:", error);
        errorAbsenceList = error;
    });

// Initialize the criminal data pool
populateCriminalDataPool()
    .catch((error) => {
        console.error("Error populating criminal data pool:", error);
        errorCriminalList = error;
    });