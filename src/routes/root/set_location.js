function setPosition() {
    document.getElementById("set-position-error").innerText = "";
    navigator.geolocation.getCurrentPosition((data) => {
        document.getElementById("longitude").value = data.coords.longitude;
        document.getElementById("latitude").value = data.coords.latitude;
    }, (err) => {
        document.getElementById("set-position-error").innerText = err.code + " | " + err.message;
    }, {
        enableHighAccuracy: true
    });
}
setPosition();