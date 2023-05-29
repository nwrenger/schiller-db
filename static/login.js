async function handleLoginSubmit() {
    var current_user = document.getElementById("username").value;
    var password = document.getElementById("password").value;
    var auth = btoa(current_user + ":" + password)
    // getting all roles
    const url = "/stats";
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
    });

    if (response.status === 200) {
        //get with getItem and clear at logout completely with clear
        window.localStorage.setItem("current_user", current_user)
        window.localStorage.setItem("auth", auth);

        window.open("/", "_self")
    } else {
        const all_elements = document.getElementsByTagName("input");
        for (const element of all_elements) {
            element.classList.add("is-invalid");
        }
    }
}