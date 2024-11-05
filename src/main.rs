use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;
use rprompt::prompt_reply;

fn main() {
    println!("Программа должна быть запущена от имени администратора.");
    println!("Нажмите Enter, чтобы продолжить...");
    prompt_reply("");
    println!("Проверка...");

    // Определяем текущую директорию
    let current_dir = env::current_dir().expect("Не удалось получить текущую директорию");

    // Путь к исполняемому файлу winws.exe
    let bin_path: PathBuf = [current_dir.to_str().unwrap(), "bin", "winws.exe"].iter().collect();

    // Проверка, существует ли файл winws.exe
    if !bin_path.exists() {
        println!("Ошибка: файл winws.exe не найден в директории bin.");
        return;
    }

    // Подготовка аргументов для winws.exe
    let args = [
        "--wf-tcp=80,443", "--wf-udp=443,50000-50100", "--filter-udp=443", "--hostlist=list-general.txt",
        "--dpi-desync=fake", "--dpi-desync-repeats=6", "--dpi-desync-fake-quic=bin/quic_initial_www_google_com.bin", "--new",
        "--filter-udp=50000-50100", "--dpi-desync=fake", "--dpi-desync-any-protocol",
        "--dpi-desync-cutoff=d3", "--dpi-desync-repeats=6", "--new", "--filter-tcp=80", "--hostlist=list-general.txt",
        "--dpi-desync=fake,split2", "--dpi-desync-autottl=2", "--dpi-desync-fooling=md5sig", "--new", "--filter-tcp=443",
        "--hostlist=list-general.txt", "--dpi-desync=fake,split", "--dpi-desync-autottl=2", "--dpi-desync-repeats=6",
        "--dpi-desync-fooling=badseq", "--dpi-desync-fake-tls=bin/tls_clienthello_www_google_com.bin"
    ];

    // Запуск команды в минимизированном режиме
    let mut command = Command::new(bin_path);
    command.args(&args);

    // Перенаправление стандартного вывода и потока ошибок на null
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    match command.spawn() {
        Ok(mut child) => {
            println!("Процесс успешно запущен!");
            match child.wait() {
                Ok(status) => println!("Процесс завершился со статусом: {}", status),
                Err(e) => eprintln!("Ошибка ожидания завершения процесса: {}", e),
            }
        }
        Err(e) => eprintln!("Не удалось запустить процесс: {} \nПри ошибке 740 необходимо открыть программу от имени администратора", e),
    }

    loop {
        // Бесконечный цикл, удерживающий программу открытой
    }
}
