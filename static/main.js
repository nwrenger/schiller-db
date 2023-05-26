const auth = localStorage.getItem("auth");
const user = localStorage.getItem("user");

if (!auth || !user) {
    window.open("login.html", "_self");
}

// this will get the data on reload, it will fetch new data
async function get_data(url) {
    const response = await fetch(url, {
        method: 'GET',
        headers: {
            'Authorization': 'Basic ' + auth,
            'Content-Type': 'application/json; charset=utf-8'
        },
    });

    let data = await response.json();

    return data["Ok"];
}

async function UserList() {
    let all_roles = await get_data("/user/all_roles");
    var list = document.getElementById("myUL");
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