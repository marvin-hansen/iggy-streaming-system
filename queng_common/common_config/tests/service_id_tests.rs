use common_config::ServiceID;

#[test]
fn test_default() {
    let service_name = ServiceID::default();
    assert_eq!(service_name, ServiceID::Default);
}

#[test]
fn test_from_i32() {
    assert_eq!(ServiceID::from(0x0_i32), ServiceID::Default);
    assert_eq!(ServiceID::from(0x1_i32), ServiceID::SMDB);
    assert_eq!(ServiceID::from(0x2_i32), ServiceID::CMDB);
    assert_eq!(ServiceID::from(0x3_i32), ServiceID::DBGW);
    assert_eq!(ServiceID::from(0x4_i32), ServiceID::QDGW);
    assert_eq!(ServiceID::from(0x5_i32), ServiceID::MDDB);
    assert_eq!(ServiceID::from(0x6_i32), ServiceID::VEX);
    assert_eq!(ServiceID::from(0x7_i32), ServiceID::ImsData);
    assert_eq!(ServiceID::from(0x8_i32), ServiceID::ImsExec);
}

#[test]
fn test_from_u8() {
    assert_eq!(ServiceID::from(0), ServiceID::Default);
    assert_eq!(ServiceID::from(1), ServiceID::SMDB);
    assert_eq!(ServiceID::from(2), ServiceID::CMDB);
    assert_eq!(ServiceID::from(3), ServiceID::DBGW);
    assert_eq!(ServiceID::from(4), ServiceID::QDGW);
    assert_eq!(ServiceID::from(5), ServiceID::MDDB);
    assert_eq!(ServiceID::from(6), ServiceID::VEX);
    assert_eq!(ServiceID::from(7), ServiceID::ImsData);
    assert_eq!(ServiceID::from(8), ServiceID::ImsExec);
}

#[test]
fn test_from_string() {
    assert_eq!(ServiceID::from_string("Default"), Some(ServiceID::Default));
    assert_eq!(ServiceID::from_string("SMDB"), Some(ServiceID::SMDB));
    assert_eq!(ServiceID::from_string("CMDB"), Some(ServiceID::CMDB));
    assert_eq!(ServiceID::from_string("DBGW"), Some(ServiceID::DBGW));
    assert_eq!(ServiceID::from_string("QDGW"), Some(ServiceID::QDGW));
    assert_eq!(ServiceID::from_string("MDDB"), Some(ServiceID::MDDB));
    assert_eq!(ServiceID::from_string("VEX"), Some(ServiceID::VEX));
    assert_eq!(ServiceID::from_string("ImsData"), Some(ServiceID::ImsData));
    assert_eq!(ServiceID::from_string("ImsExec"), Some(ServiceID::ImsExec));
    assert_eq!(
        ServiceID::from_string("KaikoProxy"),
        Some(ServiceID::KaikoProxy)
    );
    assert_eq!(
        ServiceID::from_string("KAIKO_PROXY"),
        Some(ServiceID::KaikoProxy)
    );
}

#[test]
fn test_debug() {
    let e = ServiceID::Default;
    assert_eq!(format!("{e:?}"), "Default");

    let e = ServiceID::CMDB;
    assert_eq!(format!("{e:?}"), "CMDB");

    let e = ServiceID::DBGW;
    assert_eq!(format!("{e:?}"), "DBGW");

    let e = ServiceID::QDGW;
    assert_eq!(format!("{e:?}"), "QDGW");

    let e = ServiceID::SMDB;
    assert_eq!(format!("{e:?}"), "SMDB");

    let e = ServiceID::MDDB;
    assert_eq!(format!("{e:?}"), "MDDB");

    let e = ServiceID::VEX;
    assert_eq!(format!("{e:?}"), "VEX");

    let e = ServiceID::ImsData;
    assert_eq!(format!("{e:?}"), "ImsData");

    let e = ServiceID::KaikoProxy;
    assert_eq!(format!("{e:?}"), "KaikoProxy");
}
