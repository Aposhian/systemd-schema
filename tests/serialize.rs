use systemd_schema::prelude::*;

// https://stackoverflow.com/questions/34662713/how-can-i-create-parameterized-tests-in-rust
macro_rules! serialize_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, serde_ini::to_string(&input).unwrap());
        }
    )*
    }
}

// TODO: make this use linefeed (although it doesn't matter much for the tests)
serialize_tests! {
    empty_service: (
        Service::default(),
"[Unit]\r
[Service]\r
Type=simple\r
ExecStart=\r
[Install]\r
"
    ),
    partial_fields: (
        Service {
            unit: UnitSection {
                description: Some("This is the description"),
                documentation: Some("This is the link to the docs"),
                ..Default::default()
            },
            service: ServiceSection {
                service_type: ServiceType::Notify,
                exec_start: "/usr/bin/bash",
                ..Default::default()
            },
            install: InstallSection {
                wanted_by: Some(vec!["multi-user.target"]),
                ..Default::default()
            }
        },
"[Unit]\r
Description=This is the description\r
Documentation=This is the link to the docs\r
[Service]\r
Type=notify\r
ExecStart=/usr/bin/bash\r
[Install]\r
WantedBy=multi-user.target\r
"
    ),
}