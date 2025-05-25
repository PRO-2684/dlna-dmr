#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependency")]

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use local_ip_address::local_ip;
use quick_xml::{Reader, Writer, events::Event};
use tokio::time::interval;
use std::{str::FromStr, time::Duration};
use upnp_rs::{
    SpecVersion,
    common::{
        uri::{URI, URL},
        xml::write::Writable,
    },
    description::{TypeID, device::Device as DescriptionDevice, service::Spcd},
    discovery::{notify::{self, Device as NotifyDevice}, search::SearchTarget},
};
use uuid::Uuid;

async fn description_handler(data: web::Data<DescriptionDevice>) -> HttpResponse {
    let mut writer = Writer::new(Vec::new());
    data.write(&mut writer)
        .expect("Failed to serialize device description");
    HttpResponse::Ok()
        .content_type("text/xml")
        .body(writer.into_inner())
}

async fn avtransport_scpd_handler(data: web::Data<Spcd>) -> HttpResponse {
    let mut writer = Writer::new(Vec::new());
    data.write(&mut writer)
        .expect("Failed to serialize service description");
    HttpResponse::Ok()
        .content_type("text/xml")
        .body(writer.into_inner())
}

async fn control_handler(_req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let body_str = std::str::from_utf8(&body).expect("Invalid request body");
    let mut reader = Reader::from_str(body_str);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let action_name = loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"s:Body" => {
                if let Ok(Event::Start(ref e)) = reader.read_event(&mut buf) {
                    break String::from_utf8(e.name().to_vec()).unwrap_or("Unknown".to_string());
                }
            }
            Ok(Event::Eof) => break "Unknown".to_string(),
            _ => (),
        }
    };
    eprintln!("Received action for AVTransport: {action_name}");
    let response = format!(
        r#"<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
    <s:Body>
        <u:{}Response xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
        </u:{}Response>
    </s:Body>
</s:Envelope>"#,
        action_name, action_name
    );
    HttpResponse::Ok().content_type("text/xml").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_ip = local_ip().expect("Failed to get local IP").to_string();
    let port = 8080;
    let location = format!("http://{local_ip}:{port}/description.xml");
    let uuid = Uuid::new_v4().to_string();
    eprintln!("UUID: {uuid}");

    let description_device = DescriptionDevice {
        device_type: TypeID::Device {
            domain: "schemas-upnp-org".to_string(),
            name: "MediaRenderer".to_string(),
            version: "1".to_string(),
        },
        friendly_name: "Dummy DMR".to_string(),
        manufacturer: "Dummy Manufacturer".to_string(),
        manufacturer_url: None,
        model_description: Some("Dummy DLNA Media Renderer".to_string()),
        model_name: "Dummy DMR".to_string(),
        model_number: None,
        model_url: None,
        serial_number: None,
        unique_device_name: uuid.clone(),
        upc: None,
        icon_list: vec![],
        service_list: vec![upnp_rs::description::device::Service {
            service_type: TypeID::Service {
                domain: "schemas-upnp-org".to_string(),
                name: "AVTransport".to_string(),
                version: "1".to_string(),
            },
            service_id: "urn:upnp-org:serviceId:AVTransport".to_string(),
            scpd_url: "/services/AVTransport.xml".to_string(),
            control_url: "/control/AVTransport".to_string(),
            event_sub_url: "/event/AVTransport".to_string(),
        }],
        device_list: vec![],
        presentation_url: None,
    };

    let avtransport_spcd = Spcd {
        spec_version: SpecVersion::V10,
        action_list: vec![],
        service_state_table: vec![],
    };

    let mut notify_device = NotifyDevice {
        notification_type: SearchTarget::DeviceType("MediaRenderer:1".to_string()),
        service_name: URI::from_str(&format!(
            "uuid:{uuid}::urn:schemas-upnp-org:device:MediaRenderer:1"
        ))
        .expect("Invalid URI"),
        location: URL::from_str(&location).expect("Invalid URL"),
        boot_id: 1,
        config_id: 1,
        search_port: None,
        secure_location: None,
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(description_device.clone()))
            .app_data(web::Data::new(avtransport_spcd.clone()))
            .service(web::resource("/description.xml").to(description_handler))
            .service(web::resource("/services/AVTransport.xml").to(avtransport_scpd_handler))
            .service(web::resource("/control/AVTransport").route(web::post().to(control_handler)))
    })
    .bind((local_ip, port))?;

    let server_handle = server.run();
    tokio::spawn(server_handle);

    let options = notify::Options::default_for(SpecVersion::V10);
    notify::device_available(&mut notify_device, options.clone())
        .expect("Failed to send SSDP notification");
    // Spawn a task for periodic SSDP alive notifications
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1800)); // 30 minutes
        loop {
            interval.tick().await;
            if let Err(e) = notify::device_available(&mut notify_device, options.clone()) {
                eprintln!("Failed to send SSDP alive notification: {e}");
            } else {
                eprintln!("Sent SSDP alive notification");
            }
        }
    });

    eprintln!("UPnP device available at: {location}");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
