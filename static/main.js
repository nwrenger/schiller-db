const auth = localStorage.getItem("auth");
const current_user = localStorage.getItem("current_user");

if (!auth || !current_user) {
    window.open("login.html", "_self");
}

// this will get the data on reload, it will fetch new data
async function get_data(url) {
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
    });

    let data = await response.json();

    return data["Ok"];
}

async function UserList() {
    let all_roles = await get_data("/user/all_roles");
    var list = document.getElementById("rolelist");
    var i;

    for (i = 0; i < all_roles.length; i++) {
        var node = document.createElement("li");
        var data = document.createTextNode(all_roles[i]);
        node.className = "role";
        node.appendChild(data);
        list.appendChild(node);
    }
}

function logout() {
    localStorage.clear()
    window.open("login.html", "_self");
}

function absence() {
    document.getElementById("error-main").textContent = "Missing Permissions for Absence!";
}

function criminal() {
    document.getElementById("error-main").textContent = "Missing Permissions for Criminal!";
}

UserList();