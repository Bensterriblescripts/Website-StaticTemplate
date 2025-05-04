// Telemetry
const focusOnload = document.hasFocus();
if (focusOnload) { windowactive = true; }

    // Error/Info Messages
const infoMessage = document.getElementById('infoMessage');
const errorMessage = document.getElementById('errorMessage');

let errorActive = null;
let infoActive = null;

async function showInfo(message = null) {
    if (message !== null) { infoMessage.innerHTML = message; }
    else { message = infoMessage.innerHTML }

    if (infoActive !== null) {
        clearTimeout(infoActive);
        infoActive = null;
        showInfo(message);
        return; 
    }

    fadeIn(infoMessage)
    infoActive = setTimeout(() => {
        fadeOut(infoMessage);
    }, 3000);
}
async function showError(message = null) {
    if (message !== null) { errorMessage.innerHTML = "<i>" + message; + "</i>"; }

    if (errorActive !== null) {
        clearTimeout(errorActive);
        errorActive = null;
        showError(message);
        return; 
    }

    fadeIn(errorMessage)
    errorActive = setTimeout(() => {
        fadeOut(errorMessage);
        clearTimeout(errorActive);
    }, 3000);
}

// Navigation Menu
let currentDropdown = null;
let currentDropdownContainer = null;
function toggleDropdown(dropdownMenu, dropdownContainerId) {
    const dropdownContainer = document.getElementById(dropdownContainerId);

    // We have something open, close it
    if (currentDropdown !== null) {
        currentDropdown.style.backgroundColor = "rgba(122, 122, 122, 0.582)";
        currentDropdownContainer.style.display = "none";
        
        // It's this menu, we're done here
        if (currentDropdown === dropdownMenu) {
            currentDropdown = null;
            currentDropdownContainer = null;
            return;
        }
    }

    // Open this menu
    dropdownMenu.style.backgroundColor = "rgba(255, 255, 255, 0.432)";
    dropdownContainer.style.display = "block";

    currentDropdown = dropdownMenu;
    currentDropdownContainer = dropdownContainer;
}