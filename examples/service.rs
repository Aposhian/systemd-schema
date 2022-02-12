use systemd_schema::files::Service;
use systemd_schema::service::*;
use systemd_schema::unit::*;
use systemd_schema::install::*;

fn main() {
    let service = Service {
        unit: UnitSection {
            description: Some("this is my description"),
            ..Default::default()
        },
        service: ServiceSection {
            service_type: ServiceType::Notify,
            ..Default::default()
        },
        install: InstallSection {
            ..Default::default()
        }
    };

    let serialized = serde_ini::to_string(&service).unwrap();

    println!("serialized = \n{}", serialized);
}