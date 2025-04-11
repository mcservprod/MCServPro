import { invoke } from "@tauri-apps/api/core";

async function setupServer() {
  try {
    const setupResult = await invoke<string>("setup_server_files");
    console.log("setup_server_files:", setupResult);

    const runnerResult = await invoke<string>("create_server_runner");
    console.log("create_server_runner:", runnerResult);

    const eulaResult = await invoke<string>("create_eula");
    console.log("create_eula:", eulaResult);
  } catch (error) {
    console.error("Error during server setup:", error);
  }
}

async function startServer() {
  try {
    const startResult = await invoke<string>("start_server");
    console.log("start_server:", startResult);
    return true;
  } catch (error) {
    console.error("Error starting server:", error);
    return false;
  }
}

async function stopServer() {
  try {
    const stopResult = await invoke<string>("stop_server");
    console.log("stop_server:", stopResult);
    return true;
  } catch (error) {
    console.error("Error stopping server:", error);
    return false;
  }
}

let isServerRunning = false;

window.addEventListener("DOMContentLoaded", () => {
  setupServer();

  const toggleButton = document.querySelector("#toggle_btn");
  const toggleIcon = document.querySelector("#toggle_icon");
  const statusElement = document.getElementById("server-status");

  if (!toggleButton || !toggleIcon || !statusElement) {
    console.error("Required elements not found!");
    return;
  }

  const updateButtonState = () => {
    if (isServerRunning) {
      toggleIcon.setAttribute("src", "/icons/stop.svg");
      toggleIcon.setAttribute("alt", "Stop Server");
      toggleButton.classList.remove("start");
      toggleButton.classList.add("stop");
      statusElement.textContent = "Статус сервера: Online";
      statusElement.classList.add("online");
    } else {
      toggleIcon.setAttribute("src", "/icons/play.svg");
      toggleIcon.setAttribute("alt", "Start Server");
      toggleButton.classList.remove("stop");
      toggleButton.classList.add("start");
      statusElement.textContent = "Статус сервера: Offline";
      statusElement.classList.remove("online");
    }
  };

  toggleButton.addEventListener("click", async (e) => {
    e.preventDefault();
    toggleButton.setAttribute("disabled", "true");
    if (isServerRunning) {
      const success = await stopServer();
      if (success) {
        isServerRunning = false;
      }
    } else {
      const success = await startServer();
      if (success) {
        isServerRunning = true;
      }
    }

    updateButtonState();
    toggleButton.removeAttribute("disabled");
  });

  updateButtonState();
});