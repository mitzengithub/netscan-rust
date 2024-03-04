use std::io::{stdout, Read, Write};
use crate::netdiscovery::{ProviderModel, NetworkTask};
use std::net::TcpStream;
use std::sync::Arc;
use rustls::RootCertStore;
use rustls::pki_types::{ServerName};
use openssl::x509::{X509Req, X509, X509NameRef};
use rustls::{ClientConfig};

pub struct CertProvider {
}

impl NetworkTask for CertProvider {
    
    fn execute(&self, s: &ProviderModel) {
        
        println!("executing tls certificate"); 
        
        let root_store = RootCertStore::from_iter(
            webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
        );
        
        println!("client config builder"); 
        
        let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
        
        // Allow using SSLKEYLOGFILE.
        config.key_log = Arc::new(rustls::KeyLogFile::new());
        
        let server_name = ServerName::try_from(s.host.as_str())
        .expect("invalid DNS name")
        .to_owned();
        
        let mut conn: rustls::ClientConnection = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
        
        println!("client connects!"); 
        
        let mut sock = TcpStream::connect(format!("{0}:443", s.host)).unwrap();
        let mut tls = rustls::Stream::new(&mut conn, &mut sock);
        
        //let s = s.host.as_str().to_owned();
        let s = s.host.as_str();
        
        println!("writting simple https");
        let ss = format!(
            "GET / HTTP/1.1\r\n
            {0}
            Connection: close\r\n
            Accept-Encoding: identity\r\n
            \r\n", s
        );
        
        println!("writting out the contents of our request");
        println!("{}", ss);

        tls.write_all(
            ss
            .as_bytes(),
        )
        .unwrap();
                
        let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();

        let certificates = tls.conn.peer_certificates().unwrap();
        
          // Access and output the DER of the first certificate
        let first_cert = certificates.first().unwrap();
        let der_bytes = first_cert.as_ref();

        println!("Processing certificate names");
        let x509 = X509::from_der(der_bytes);
        let target_certificate = x509.unwrap().to_owned();

        let entry = target_certificate.issuer_name();

        for entry in entry.entries() {
            println!("{}:{}", entry.object().to_string(), entry.data().as_utf8().unwrap());
        }

        println!("==== certificate names");
        // Write the DER bytes to standard output
        std::io::stdout().write_all(der_bytes);

        //println!("{}", pem.unwrap());
                      
        writeln!(
            &mut std::io::stderr(),
            "Current ciphersuite: {:?}",
            ciphersuite.suite(), 
        )
        .unwrap();

        //tls.conn.send_close_notify();

        let mut plaintext = Vec::new();
        tls.read_to_end(&mut plaintext).unwrap();

        stdout().write_all(&plaintext).unwrap();
        }
}


