async function handleLoginSubmit() {
    var user = document.getElementsByName("user")[0].value;
    var password = document.getElementsByName("password")[0].value;
    var auth = btoa(user + ":" + password)
    // getting all roles
    const url = '/user/all_roles';
    const response = await fetch(url, {
        method: 'GET',
        headers: {
        'Authorization': 'Basic ' + auth,
        'Content-Type': 'application/json; charset=utf-8'
        },
    });

    console.log(response);
    
    if (response.status === 200) {
        var all_roles = await response.json();
        window.localStorage.setItem("user", user)
        window.localStorage.setItem("auth", auth);
        window.localStorage.setItem("all_roles", JSON.stringify(all_roles["Ok"]));
        //get with getItem and clear at logout completely with clear
        window.open("/", "_self")
    } else {
        document.getElementById("error").hidden = false;
    }
}