let auth = localStorage.getItem("auth");
let all_roles_s = localStorage.getItem("all_roles");
let user = localStorage.getItem("user");

if (!auth || !all_roles_s || !user) {
    window.open("login.html", "_self");
}

let all_roles = JSON.parse(all_roles_s);

var list = document.getElementById("myUL");
var i;

for (i = 0; i < all_roles.length; i++) {
    var node = document.createElement("li");
    var data = document.createTextNode(all_roles[i]);
    node.className = "role";
    node.appendChild(data);
    list.appendChild(node);
}
