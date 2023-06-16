async function handleLoginSubmit() {
    var current_user = document.getElementById("username").value.trim();
    var password = document.getElementById("password").value;
    var auth = btoa(current_user + ":" + password)
    // getting all roles
    const url = "/login/fetch/" + encodeURIComponent(current_user);
    const response = await fetch(url, {
        method: "GET",
        headers: {
            "Authorization": "Basic " + auth,
            "Content-Type": "application/json; charset=utf-8"
        },
    });

    const data = await response.json();

    if (response.status === 200) {
        //get with getItem and clear at logout completely with clear
        window.localStorage.setItem("auth", auth);
        window.localStorage.setItem("permissions", JSON.stringify(data["Ok"]));

        window.open("/", "_self")
    } else {
        const all_elements = document.getElementsByTagName("input");
        for (const element of all_elements) {
            element.classList.add("is-invalid");
        }
    }
}