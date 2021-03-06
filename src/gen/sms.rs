// This code was autogenerated with `dbus-codegen-rust `, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus_tree as tree;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, tree::MethodErr>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, tree::MethodErr>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), tree::MethodErr>;
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Signal: Default,
    T: OrgFreedesktopDBusProperties,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let d = fclone(minfo);
        let value = d.get(interface_name, property_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let d = fclone(minfo);
        let properties = d.get_all(interface_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let value: arg::Variant<Box<dyn arg::RefArg>> = i.read()?;
        let d = fclone(minfo);
        d.set(interface_name, property_name, value)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);
    let s = factory.signal("PropertiesChanged", Default::default());
    let s = s.arg(("interface_name", "s"));
    let s = s.arg(("changed_properties", "a{sv}"));
    let s = s.arg(("invalidated_properties", "as"));
    let i = i.add_s(s);
    i
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, tree::MethodErr>;
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml_data = d.introspect()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml_data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml_data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), tree::MethodErr>;
    fn get_machine_id(&self) -> Result<String, tree::MethodErr>;
}

pub fn org_freedesktop_dbus_peer_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusPeer,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Peer", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.ping()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Ping", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let machine_uuid = d.get_machine_id()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(machine_uuid);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMachineId", Default::default(), h);
    let m = m.out_arg(("machine_uuid", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopModemManager1Sms {
    fn send(&self) -> Result<(), tree::MethodErr>;
    fn store(&self, storage: u32) -> Result<(), tree::MethodErr>;
    fn state(&self) -> Result<u32, tree::MethodErr>;
    fn pdu_type(&self) -> Result<u32, tree::MethodErr>;
    fn number(&self) -> Result<String, tree::MethodErr>;
    fn text(&self) -> Result<String, tree::MethodErr>;
    fn data(&self) -> Result<Vec<u8>, tree::MethodErr>;
    fn smsc(&self) -> Result<String, tree::MethodErr>;
    fn validity(&self) -> Result<(u32, arg::Variant<Box<dyn arg::RefArg + 'static>>), tree::MethodErr>;
    fn class(&self) -> Result<i32, tree::MethodErr>;
    fn teleservice_id(&self) -> Result<u32, tree::MethodErr>;
    fn service_category(&self) -> Result<u32, tree::MethodErr>;
    fn delivery_report_request(&self) -> Result<bool, tree::MethodErr>;
    fn message_reference(&self) -> Result<u32, tree::MethodErr>;
    fn timestamp(&self) -> Result<String, tree::MethodErr>;
    fn discharge_timestamp(&self) -> Result<String, tree::MethodErr>;
    fn delivery_state(&self) -> Result<u32, tree::MethodErr>;
    fn storage(&self) -> Result<u32, tree::MethodErr>;
}

pub fn org_freedesktop_modem_manager1_sms_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgFreedesktopModemManager1Sms,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.ModemManager1.Sms", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.send()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Send", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let storage: u32 = i.read()?;
        let d = fclone(minfo);
        d.store(storage)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Store", Default::default(), h);
    let m = m.in_arg(("storage", "u"));
    let i = i.add_m(m);

    let p = factory.property::<u32, _>("State", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.state()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("PduType", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.pdu_type()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Number", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.number()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Text", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.text()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<Vec<u8>, _>("Data", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.data()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("SMSC", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.smsc()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<(u32, arg::Variant<Box<dyn arg::RefArg>>), _>("Validity", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.validity()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<i32, _>("Class", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.class()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("TeleserviceId", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.teleservice_id()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("ServiceCategory", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.service_category()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("DeliveryReportRequest", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.delivery_report_request()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("MessageReference", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.message_reference()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Timestamp", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.timestamp()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("DischargeTimestamp", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.discharge_timestamp()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("DeliveryState", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.delivery_state()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("Storage", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.storage()?);
        Ok(())
    });
    let i = i.add_p(p);
    i
}
