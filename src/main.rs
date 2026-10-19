//    ░▒▓███████▓▒░░▒▓████████▓▒░░▒▓███████▓▒░▒▓█▓▒░░▒▓██████▓▒░░▒▓███████▓▒░░▒▓████████▓▒░▒▓███████▓▒░       ░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░      ░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓█▓▒░      ░▒▓█▓▒░        
//    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
//    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
//    ░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░  ░▒▓██████▓▒░░▒▓█▓▒░▒▓█▓▒▒▓███▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓███████▓▒░ ░▒▓██████▓▒░       ░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓█▓▒░      ░▒▓█▓▒░        
//    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░             ░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░          ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░        
//    ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░             ░▒▓█▓▒░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░          ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░▒▓██▓▒░ 
//    ░▒▓███████▓▒░░▒▓████████▓▒░▒▓███████▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓███████▓▒░       ░▒▓███████▓▒░   ░▒▓█▓▒░          ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░░▒▓████████▓▒░▒▓█▓▒░▒▓██▓▒░ 



use std::process::Command;
use colored::Colorize;


fn main() {
    welcome_banner();
    let output = Command::new("reg")
            .args([
                "query",
                "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
                "/v",
                "ProxyEnable",
            ])
            .output()
        .expect("Failed to execute command");
    let registry_text = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
         println!("{}", "[+] Command executed successfully".green());
    } else {
        eprintln!("[!] Command execution failed with error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    let mut risk_score = 0;
    let threat_level_max = 100;
    let suspicious_certificate = true;
    let  defender_exclusion = true;
    let proxy_detected = registry_text.contains("0x1");

    if proxy_detected {
        println!("{}", "[!!!] Proxy detected".red());
    } else {
        println!("{}", "[-] No proxy detected".green());
    }
    if suspicious_certificate {
        risk_score += 30;
        println!("{}", "[!!!] Suspicious certificate detected".red());
    } else {
        println!("{}", "[-] No suspicious certificate detected".green());
    }

    if defender_exclusion {
        risk_score += 35;
        println!("{}", "[!!!] Defender exclusion detected".red());
        } else {
        println!("{}", "[-] No defender exclusion detected".green());
     }
    risk_score += calculate_proxy_risk(proxy_detected);

    recommended_action(risk_score, threat_level_max);
    let score_text = format!("Risk score: {}/{}", risk_score, threat_level_max);
    let classification = defcon(risk_score, threat_level_max);

    if classification == "HIGH" {
        println!("{}", score_text.red().bold());
    } else if classification == "MODERATE" {
        println!("{}", score_text.yellow());
    } else {
        println!("{}", score_text.green());
    }

    if classification == "HIGH" {
        println!("Threat classification: {}", classification.red().bold());
    } else if classification == "MODERATE" {
        println!("Threat classification: {}", classification.yellow());
    } else {
        println!("Threat classification: {}", classification.green());
    }
}






fn recommended_action(risk_score: i32, threat_level_max: i32) {
    let risk_percentage = (risk_score as f32 / threat_level_max as f32) * 100.0;
    if risk_percentage >= 75.0 {
        println!("{}", "[!!!] High risk detected. Immediate action recommended.".red());
    } else if risk_percentage >= 50.0 {
        println!("{}", "[!] Moderate risk detected. Review and take necessary precautions.".yellow());
    } else {
        println!("{}", "[+] Low risk detected. Continue monitoring.".green());
    }
}
fn defcon(risk_score: i32, threat_level_max: i32) -> &'static str {
    let risk_percentage = (risk_score as f32 / threat_level_max as f32) * 100.0;

    if risk_percentage >= 75.0 {
        "HIGH"
    } else if risk_percentage >= 50.0 {
        "MODERATE"
    } else {
        "LOW"
    }
}

fn calculate_proxy_risk(detected: bool) -> i32 {
    if detected {
        30 // Assign a risk score of 30 for proxy detection
    } else {
        0
    }
}

#[test]
fn test_proxy_detection() {
    assert_eq!(calculate_proxy_risk(true), 30);
    assert_eq!(calculate_proxy_risk(false), 0);
}




#[test]
fn test_defcon() {
    assert_eq!(defcon(80, 100), "HIGH");
    assert_eq!(defcon(75, 100), "HIGH");
    assert_eq!(defcon(60, 100), "MODERATE");
    assert_eq!(defcon(50, 100), "MODERATE");
    assert_eq!(defcon(40, 100), "LOW");
}

















fn welcome_banner() {
 println!("[0;97m▐▓▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0m
[0;97m  ▐▓[0;37m▌  [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌    [0;97m▐▓[0;37m▌   [0m
[0;97m  ▐▓[0;37m▌  [0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓[0;37m▌   [0;97m▐▓▓▓[0;37m▌ [0;97m▐▓▓▓[0;37m▌ [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓▓[0;37m▌[0;97m▐▓▓▓[0;37m▌ [0m
[0;97m  ▐▓[0;37m▌  [0;97m▐▓[0;97;47m▐[0;97m▓[0;37m▌ [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌   [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;97;47m▐[0;97m▓[0;37m▌ [0;97m▐▓[0;37m▌[0;97m ▐▓[0;37m▌[0;97m▐▓[0;37m▌   [0m
[0;97m  ▐▓[0;37m▌  [0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓[0;37m▌   [0;97m▐▓▓▓▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓[0;37m▌[0;97m▐▓▓▓▓▓[0;37m▌[0;97m▐▓▓▓▓[0;37m▌[0m\n\n\n");
println!("[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓[0;31m▌[0;97m [0;91m▐▓[0;31m▌[0;37m    [0;91m▐▓▓[0;31m▌[0;37m    [0;91m▐▓▓▓▓[0;31m▌[0m
[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;37m   [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;37m   [0;97m  [0;91m▐▓[0;31m▌[0;37m  [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓[0;31m▌[0;91m▐▓[0;31m▌[0;37m    [0;91m ▐▓[0;31m▌[0;37m    [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0m
[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓▓[0;31m▌[0;37m [0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;97m  [0;91m▐▓[0;31m▌[0;37m  [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;91;41m▐[0;91m▓[0;91;41m▐[0;91m▓[0;31m▌[0;37m    [0;91m ▐▓[0;31m▌[0;37m    [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0m
[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;37m   [0;91m▐▓[0;91;41m▐[0;91m▓[0;31m▌[0;37m [0;97m   [0;91m▐▓[0;31m▌[0;97m  [0;91m▐▓[0;31m▌[0;37m  [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓[0;31m▌[0;37m    [0;91m ▐▓[0;31m▌[0;37m    [0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0m
[0;97m [0;91m▐▓▓[0;31m▌[0;37m [0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0;91m▐▓[0;31m▌[0;97m [0;91m▐▓[0;31m▌[0;37m    [0;91m▐▓▓▓[0;31m▌[0;91m▐▓[0;31m▌[0;91m▐▓▓▓▓[0;31m▌[0m\n\n\n\n");

}
