let auth = localStorage.getItem("auth");
let all_roles_s = localStorage.getItem("all_roles");
let stats_s = localStorage.getItem("stats");
let user = localStorage.getItem("user");

if (!auth || !all_roles_s || !stats_s || !user) {
    window.open("login.html", "_self");
}

async function uptodate() {
    let recent = sessionStorage.getItem("recent");
    
    if (!recent) {
        const url = '/user/all_roles';
        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Authorization': 'Basic ' + auth,
                'Content-Type': 'application/json; charset=utf-8'
            },
        });
        
        let all_roles = await response.json();
        
        const url2 = '/stats';
        const response2 = await fetch(url2, {
            method: 'GET',
            headers: {
                'Authorization': 'Basic ' + auth,
                'Content-Type': 'application/json; charset=utf-8'
            },
        });
        let stats = await response2.json();
        return [all_roles["Ok"], stats["Ok"]];
    } else {
        let all_roles = JSON.parse(all_roles_s);
        let stats = JSON.parse(stats_s);
        sessionStorage.clear();
        return [all_roles, stats];
    }
}

async function UserList() {
    let [all_roles, _stats] = await uptodate();
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