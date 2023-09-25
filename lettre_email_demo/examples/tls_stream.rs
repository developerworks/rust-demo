use lettre::{SmtpTransport, Transport};
use lettre_email::{EmailAddress, EmailBuilder};
use native_tls::TlsConnector;
use socks::Socks5Stream;
use std::net::{TcpStream, ToSocketAddrs};

fn main() -> lettre::Result<()> {
    // 构建邮件
    let email = EmailBuilder::new()
        .to((EmailAddress::new("recipient@example.com".to_string()).unwrap(),))
        .from(EmailAddress::new("sender@example.com".to_string()).unwrap())
        .subject("Test email")
        .text("This is a test email")
        .build()
        .unwrap();

    // 创建支持SOCKS5的TLS Stream
    let connector = TlsConnector::builder().build().unwrap();
    let proxy_addr = "127.0.0.1:1080";
    let smtp_addr = "smtp.example.com";
    // let stream = TcpStream::connect(proxy_addr).unwrap();
    // let socks5_stream = Socks5Stream::connect(stream, smtp_addr).unwrap();

    let stream = TcpStream::connect(proxy_addr).unwrap();
    let socks5_stream = Socks5Stream::connect(stream, &*smtp_addr.to_socket_addrs().unwrap().next().unwrap()).unwrap();


    let tls_stream = connector.connect(smtp_addr, socks5_stream).unwrap();

    // 创建邮件客户端并发送
    let mailer = SmtpTransport::new(tls_stream).hello_name(Some("hello@example.com".to_string())).build();

    mailer.send(email)?;

    Ok(())
}
