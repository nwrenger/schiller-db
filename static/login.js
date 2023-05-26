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

    if (response.status === 200) {
        //get with getItem and clear at logout completely with clear
        window.localStorage.setItem("user", user)
        window.localStorage.setItem("auth", auth);

        window.open("/", "_self")
    } else {
        let all_elements = document.getElementsByTagName("input");
        for (i = 0; i < all_elements.length; i++) {
            all_elements.item(i).classList.add("error-login");
        }
    }
}