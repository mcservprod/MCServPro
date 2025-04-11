use dirs::document_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;
use tauri::command;
use tauri::State;

pub struct ServerState(pub Mutex<Option<Child>>);

#[command]
pub async fn setup_server_files() -> Result<String, String> {
    if let Some(documents_dir) = document_dir() {
        let base_dir = documents_dir.join("Server");

        if !base_dir.exists() {
            match fs::create_dir_all(&base_dir) {
                Ok(_) => println!("Папка успешно создана"),
                Err(e) => return Err(format!("Ошибка при создании папки: {}", e)),
            }
        }

        let path = base_dir.join("server.jar");

        if path.exists() {
            return Ok("[ OK ] Файл существует. Нечего делать".to_string());
        }

        let url: &str = "https://api.papermc.io/v2/projects/paper/versions/1.21.4/builds/224/downloads/paper-1.21.4-224.jar";
        match reqwest::get(url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => {
                            if let Err(e) = fs::write(&path, bytes) {
                                return Err(format!("Ошибка при записи файла: {}", e));
                            }
                            Ok("[ OK ] Файл сервера успешно загружен!".to_string())
                        }
                        Err(e) => Err(format!("Ошибка при чтении байтов ответа: {}", e)),
                    }
                } else {
                    Err(format!("Запрос не удался со статусом: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Ошибка при выполнении запроса: {}", e)),
        }
    } else {
        Err("[ Err ] Не удалось получить путь к папке документов".to_string())
    }
}

#[command]
pub fn create_server_runner() -> Result<String, String> {
    if let Some(documents_dir) = document_dir() {
        let base_dir = documents_dir.join("Server");

        if !base_dir.exists() {
            return Err("Папка сервера не существует!".to_string());
        }

        let path = base_dir.join("start.bat");

        if path.exists() {
            return Ok("[ OK ] Файл start.bat уже существует".to_string());
        }

        let content = "java -Xms1024M -Xmx1024M --add-modules=jdk.incubator.vector -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -jar server.jar";

        match File::create(&path) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => Ok("[ OK ] Файл start.bat успешно создан".to_string()),
                    Err(e) => Err(format!("Ошибка при записи в файл start.bat: {}", e)),
                }
            }
            Err(e) => Err(format!("Ошибка при создании файла start.bat: {}", e)),
        }
    } else {
        Err("[ Err ] Не удалось получить путь к папке документов".to_string())
    }
}

#[command]
pub fn create_eula() -> Result<String, String> {
    if let Some(documents_dir) = document_dir() {
        let base_dir = documents_dir.join("Server");

        if !base_dir.exists() {
            return Err("Папка сервера не существует!".to_string());
        }

        let path = base_dir.join("eula.txt");

        if path.exists() {
            return Ok("[ OK ] Файл eula.txt уже существует".to_string());
        }

        let content = "eula=true";

        match File::create(&path) {
            Ok(mut file) => {
                match file.write_all(content.as_bytes()) {
                    Ok(_) => Ok("[ OK ] Файл eula.txt успешно создан".to_string()),
                    Err(e) => Err(format!("Ошибка при записи в файл eula.txt: {}", e)),
                }
            }
            Err(e) => Err(format!("Ошибка при создании файла eula.txt: {}", e)),
        }
    } else {
        Err("[ Err ] Не удалось получить путь к папке документов".to_string())
    }
}

#[command]
pub fn start_server(state: State<ServerState>) -> Result<String, String> {
    if let Some(documents_dir) = document_dir() {
        let base_dir = documents_dir.join("Server");
        let path = base_dir.join("start.bat");

        if !base_dir.exists() {
            return Err("Папка сервера не существует!".to_string());
        }

        if path.exists() {
            let command_path: PathBuf = path.clone();
            match Command::new(command_path)
                .current_dir(&base_dir)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
            {
                Ok(child) => {
                    // Сохраняем процесс в состоянии
                    let mut server_state = state.0.lock().unwrap();
                    *server_state = Some(child);
                    Ok("[ OK ] Сервер успешно запущен".to_string())
                }
                Err(e) => Err(format!("Ошибка при запуске сервера: {}", e)),
            }
        } else {
            Err("Файл start.bat не найден".to_string())
        }
    } else {
        Err("[ Err ] Не удалось получить путь к папке документов".to_string())
    }
}

#[command]
pub fn stop_server(state: State<ServerState>) -> Result<String, String> {
    let mut server_state = state.0.lock().unwrap();
    if let Some(mut child) = server_state.take() {
        if let Some(stdin) = child.stdin.as_mut() {
            if let Err(e) = writeln!(stdin, "stop") {
                return Err(format!("Ошибка при отправке команды stop: {}", e));
            }
            if let Err(e) = stdin.flush() {
                return Err(format!("Ошибка при отправке команды stop: {}", e));
            }
        }

        match child.wait() {
            Ok(status) => {
                if status.success() {
                    Ok("[ OK ] Сервер успешно остановлен".to_string())
                } else {
                    Err(format!("Сервер завершился с ошибкой: {:?}", status))
                }
            }
            Err(e) => Err(format!("Ошибка при ожидании завершения сервера: {}", e)),
        }
    } else {
        Err("Сервер не запущен".to_string())
    }
}

