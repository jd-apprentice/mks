mod functions;
pub use functions::mks;

const FOLDERS_TO_CREATE: [&str; 4] = ["nmap", "content", "exploits", "scripts"];
const FILE_TO_CREATE: &str = "README.md";
const LOGO: &str = "
    ███▄ ▄███▓ ██ ▄█▀  ██████ 
    ▓██▒▀█▀ ██▒ ██▄█▒ ▒██    ▒ 
    ▓██    ▓██░▓███▄░ ░ ▓██▄   
    ▒██    ▒██ ▓██ █▄   ▒   ██▒
    ▒██▒   ░██▒▒██▒ █▄▒██████▒▒
    ░ ▒░   ░  ░▒ ▒▒ ▓▒▒ ▒▓▒ ▒ ░
    ░  ░      ░░ ░▒ ▒░░ ░▒  ░ ░
    ░      ░   ░ ░░ ░ ░  ░  ░  
       ░   ░  ░         ░
";