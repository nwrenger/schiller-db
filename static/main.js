let auth = localStorage.getItem("auth");
let all_roles_s = localStorage.getItem("all_roles");
let user = localStorage.getItem("user");

if (!auth || !all_roles_s || !user) {
    window.open("login.html", "_self");
}

let all_roles = JSON.parse(all_roles_s);

var toggler = document.getElementsByClassName("caret");
var i;

for (i = 0; i < toggler.length; i++) {
    toggler[i].addEventListener("click", function () {
        this.parentElement.querySelector(".nested").classList.toggle("active");
        this.classList.toggle("caret-down");
    });
}
