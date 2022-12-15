use himalaya_lib::{AccountConfig, BackendBuilder,  BackendConfig, Email, EmailSender, envelope, Envelopes, ImapConfig, SenderBuilder, SmtpConfig};
use himalaya_lib::imap::Error;
use imap::types::{Fetch, Name, ZeroCopy};
use native_tls::TlsConnector;

pub fn fetch_inbox_top_list(domain: &str, username: &str, password: &str) -> Result<ZeroCopy<Vec<Fetch>>, himalaya_lib::imap::Error> {

    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0).expect("error login");
    imap_session.select("INBOX").expect("imap session");

    let messages = imap_session.fetch("1:*", "(ENVELOPE FLAGS INTERNALDATE)").unwrap();
    println!("{}", messages.len());
    for i in 0..messages.len() {
        let mut msg = &messages[i];
        let topic = std::str::from_utf8(msg.envelope().unwrap().subject.unwrap()).unwrap();

        let recipient = msg.envelope().unwrap().from.as_ref().unwrap();
        let from = std::str::from_utf8(recipient.clone()[0].mailbox.unwrap()).unwrap();

        let from_last = std::str::from_utf8(recipient.clone()[0].host.unwrap()).unwrap();
        let from_full = from.to_owned() +"@"+&from_last;
        let date = msg.internal_date().unwrap();
        let msg_id = std::str::from_utf8(messages[i].envelope().unwrap().message_id.unwrap()).unwrap();
        println!("id message - {} ,\n  date - {},\n topic - {:?},\n from - {:?},\n recipient - {}\n  ", msg_id, date,  topic,  from_full  ,username.clone() );
    }

    imap_session.logout().expect("error logout");

    Ok(messages)
}

fn main() {
    let input = fetch_inbox_top_list(
        "smtp.server",
        "username",
        "password",
    ).unwrap();


}
