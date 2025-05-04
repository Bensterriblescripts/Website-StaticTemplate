// Fade
async function fadeIn(element) {
    element.style.display = "block";
    await sleep(100);
    element.style.opacity = "1";
}
async function fadeOut(element) {
    element.style.opacity = "0";
    await sleep(100);
    element.style.display = "none";
}
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// Redirect
function redirect(page = "") {
    window.location.href = page;
}

// Tracking
let windowactive = false;
window.onblur = () => { windowactive = false; }
window.onfocus = () => { windowactive = true; }