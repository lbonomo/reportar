use std::time::Duration;
use snmp::{SyncSession, Value};

fn main() {
    let sys_descr_oid = &[1,3,6,1,2,1,47,1,1,1,1,2,65536];
    let agent_addr    = "10.135.174.1:161";
    let community     = b"public";
    let timeout       = Duration::from_secs(2);
    
    let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
    let mut response = sess.get(sys_descr_oid).unwrap();
    if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
        println!("myrouter sysDescr: {}", String::from_utf8_lossy(sys_descr));
    } else {
        println!("Do data");
    }
}

