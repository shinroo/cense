use std::collections::HashMap;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut links = HashMap::new();
    links.insert("agpl3", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/agpl3.txt");
    links.insert("apache", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/apache.txt");
    links.insert("bsd2", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/bsd2.txt");
    links.insert("bsd3", "https://github.com/licenses/license-templates/blob/master/templates/bsd3.txt");
    links.insert("cc0", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc0.txt");
    links.insert("cc_by", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by.txt");
    links.insert("cc_by_nc", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by_nc.txt");
    links.insert("cc_by_nc_nd", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by_nc_nd.txt");
    links.insert("cc_by_nc_sa", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by_nc_sa.txt");
    links.insert("cc_by_nd", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by_nd.txt");
    links.insert("cc_by_sa", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cc_by_sa.txt");
    links.insert("cddl", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/cddl.txt");
    links.insert("epl", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/epl.txt");
    links.insert("gpl2", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/gpl2.txt");
    links.insert("gpl3", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/gpl3.txt");
    links.insert("isc", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/isc.txt");
    links.insert("lgpl", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/lgpl.txt");
    links.insert("mit", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/mit.txt");
    links.insert("mpl", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/mpl.txt");
    links.insert("unlicense", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/unlicense.txt");
    links.insert("wtfpl", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/wtfpl.txt");
    links.insert("x11", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/x11.txt");
    links.insert("zlib", "https://raw.githubusercontent.com/licenses/license-templates/master/templates/zlib.txt");


    if args.len() != 2 {
        eprintln!("Usage : cense <license>");
        std::process::exit(66);
    }
    if args[1] == "--list" || args[1] == "-l"{
        for (k, _v) in &links {
            println!("• {}",k);
        }
    } else {
        for (k, v) in &links {
            if &args[1].as_str() == k {
                let output = Command::new("curl").arg(v).arg("-o").arg("LICENSE").output().expect("Failed to clone License");
                if output.status.success() {
                    println!("Successfully downloaded license `{}` into LICENSE", k);
                } else {
                    eprintln!("Failed to download license");
                    std::process::exit(401);
                }
                std::process::exit(0);
            }
        }
        eprintln!("License `{}` not found : try cense --list to see available licenses", args[1]);
        std::process::exit(-2);
    }

}
