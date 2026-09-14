//! Record Installer requests, replies, signals, and property changes as JSON.
//!
//! Includes Get, GetAll, Set, and PropertiesChanged for the Installer interface.
//! Also records GetNameOwner calls for RAUC and their bus-daemon replies.
//! Start RAUC first; restart this recorder if RAUC restarts. Only replies to
//! calls observed after monitoring starts can be recorded.
//!
//! Run in an empty output directory to avoid filename collisions. Becoming a
//! monitor may require elevated privileges under the system bus policy.
//! Saves raw bytes plus decoded arguments for inspection. Byte order and Unix
//! file descriptors are not preserved.

use std::{collections::HashMap, fs::File, io::Write, num::NonZeroU32};

use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use zbus::{Connection, MessageStream};

const BUS_SERVICE: &str = "org.freedesktop.DBus";

const RAUC_SERVICE: &str = "de.pengutronix.rauc";

const INSTALLER_INTERFACE: &str = "de.pengutronix.rauc.Installer";
const PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";

#[derive(Debug, Serialize, Deserialize)]
struct RecordedBody {
    signature: String,
    body: Vec<u8>,
    decoded: serde_json::Value,
}

/// Serial numbers are unique per caller, not across the whole bus.
struct ReplyTracker {
    owner: String,
    pending: HashMap<(String, NonZeroU32), (String, String)>,
}

impl ReplyTracker {
    fn recorded_method(&mut self, message: &zbus::Message) -> Option<String> {
        use zbus::message::{Flags, Type};
        let header = message.header();
        match message.message_type() {
            Type::MethodCall => {
                let destination = header.destination().map(|name| name.as_str());
                let interface = header.interface().map(|name| name.as_str());
                let property_interface = if interface == Some(PROPERTIES_INTERFACE) {
                    let body = message.body();
                    match header.member().map(|name| name.as_str()) {
                        Some("Get") => body
                            .deserialize::<(String, String)>()
                            .ok()
                            .map(|(interface, _)| interface),
                        Some("GetAll") => body
                            .deserialize::<(String,)>()
                            .ok()
                            .map(|(interface,)| interface),
                        Some("Set") => body
                            .deserialize::<(String, String, zbus::zvariant::OwnedValue)>()
                            .ok()
                            .map(|(interface, _, _)| interface),
                        _ => None,
                    }
                } else {
                    None
                };
                let name_lookup = destination == Some(BUS_SERVICE)
                    && interface == Some(BUS_SERVICE)
                    && header.member().map(|name| name.as_str()) == Some("GetNameOwner")
                    && message
                        .body()
                        .deserialize::<(String,)>()
                        .is_ok_and(|(name,)| name == RAUC_SERVICE);
                if name_lookup
                    || ((interface == Some(INSTALLER_INTERFACE)
                        || property_interface.as_deref() == Some(INSTALLER_INTERFACE))
                        && (destination == Some(RAUC_SERVICE)
                            || destination == Some(self.owner.as_str())))
                {
                    if let (Some(sender), Some(member)) = (header.sender(), header.member()) {
                        // Replies have no member name, so remember it from the call.
                        if !header.primary().flags().contains(Flags::NoReplyExpected) {
                            self.pending.insert(
                                (sender.to_string(), header.primary().serial_num()),
                                (
                                    member.to_string(),
                                    if name_lookup {
                                        BUS_SERVICE
                                    } else {
                                        self.owner.as_str()
                                    }
                                    .to_string(),
                                ),
                            );
                        }
                        return Some(member.to_string());
                    }
                }
                None
            }
            Type::Signal => {
                if header.sender().map(|name| name.as_str()) != Some(self.owner.as_str()) {
                    return None;
                }
                let interface = header.interface()?.as_str();
                let member = header.member()?.as_str();
                if interface == INSTALLER_INTERFACE {
                    return Some(member.to_string());
                }
                if interface == PROPERTIES_INTERFACE && member == "PropertiesChanged" {
                    let body = message.body();
                    let (changed_interface, _, _): (
                        String,
                        HashMap<String, zbus::zvariant::OwnedValue>,
                        Vec<String>,
                    ) = body.deserialize().ok()?;
                    if changed_interface == INSTALLER_INTERFACE {
                        return Some(member.to_string());
                    }
                }
                None
            }
            Type::MethodReturn | Type::Error => {
                let caller = header.destination()?;
                let serial = header.reply_serial()?;
                let key = (caller.to_string(), serial);
                let (_, expected_sender) = self.pending.get(&key)?;
                if header.sender().map(|name| name.as_str()) != Some(expected_sender.as_str()) {
                    return None;
                }
                self.pending.remove(&key).map(|(member, _)| member)
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;

    let owner: String = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetNameOwner",
            &(RAUC_SERVICE,),
        )
        .await?
        .body()
        .deserialize()?;
    let mut tracker = ReplyTracker {
        owner,
        pending: HashMap::new(),
    };

    // Subscribe before becoming a monitor to avoid missing early messages.
    let mut stream = MessageStream::from(&connection);
    let rules = [
        format!(
            "type='method_call',interface='{BUS_SERVICE}',destination='{BUS_SERVICE}',member='GetNameOwner',arg0='{RAUC_SERVICE}'"
        ),
        format!("type='method_return',sender='{BUS_SERVICE}'"),
        format!("type='error',sender='{BUS_SERVICE}'"),
        format!(
            "type='method_call',interface='{INSTALLER_INTERFACE}',destination='{RAUC_SERVICE}'"
        ),
        format!(
            "type='method_call',interface='{INSTALLER_INTERFACE}',destination='{}'",
            tracker.owner
        ),
        format!(
            "type='method_call',interface='{PROPERTIES_INTERFACE}',destination='{RAUC_SERVICE}',arg0='{INSTALLER_INTERFACE}'"
        ),
        format!(
            "type='method_call',interface='{PROPERTIES_INTERFACE}',destination='{}',arg0='{INSTALLER_INTERFACE}'",
            tracker.owner
        ),
        format!("type='method_return',sender='{}'", tracker.owner),
        format!("type='error',sender='{}'", tracker.owner),
        format!(
            "type='signal',sender='{}',interface='{INSTALLER_INTERFACE}'",
            tracker.owner
        ),
        format!(
            "type='signal',sender='{}',interface='{PROPERTIES_INTERFACE}',member='PropertiesChanged',arg0='{INSTALLER_INTERFACE}'",
            tracker.owner
        ),
    ];

    connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Monitoring"),
            "BecomeMonitor",
            &(rules.as_slice(), 0u32),
        )
        .await?;

    let mut index = 0u64;

    while let Some(message) = stream.try_next().await? {
        let Some(method) = tracker.recorded_method(&message) else {
            continue;
        };

        let body = message.body();
        let recorded = RecordedBody {
            signature: body.signature().to_string(),
            body: body.data().bytes().to_vec(),
            decoded: if body.is_empty() {
                serde_json::json!([])
            } else {
                let arguments: zbus::zvariant::Structure<'_> = body.deserialize()?;
                serde_json::to_value(&arguments)?
            },
        };
        let kind = match message.message_type() {
            zbus::message::Type::MethodCall => "request",
            zbus::message::Type::Error => "error",
            zbus::message::Type::Signal => "signal",
            _ => "reply",
        };
        let path = format!("{method}-{kind}-{index:06}.json");

        // Avoid overwriting recordings from a previous run.
        let json = serde_json::to_string_pretty(&recorded)?;
        File::create_new(&path)?.write_all(json.as_bytes())?;

        eprintln!("Saved {path}");
        index += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_installer_requests_and_matching_replies() -> Result<(), Box<dyn std::error::Error>> {
        let mut tracker = ReplyTracker {
            owner: ":1.42".into(),
            pending: HashMap::new(),
        };
        let call = zbus::Message::method_call("/", "GetPrimary")?
            .interface(INSTALLER_INTERFACE)?
            .destination(RAUC_SERVICE)?
            .sender(":1.10")?
            .build(&())?;
        assert_eq!(
            tracker.recorded_method(&call).as_deref(),
            Some("GetPrimary")
        );

        let wrong_sender = zbus::Message::method_return(&call.header())?
            .sender(":1.99")?
            .build(&"rootfs.0")?;
        assert!(tracker.recorded_method(&wrong_sender).is_none());
        let wrong_caller = zbus::Message::method_return(&call.header())?
            .sender(":1.42")?
            .destination(":1.11")?
            .build(&"rootfs.0")?;
        assert!(tracker.recorded_method(&wrong_caller).is_none());

        let reply = zbus::Message::method_return(&call.header())?
            .sender(":1.42")?
            .build(&"rootfs.0")?;
        assert_eq!(
            tracker.recorded_method(&reply).as_deref(),
            Some("GetPrimary")
        );
        assert!(tracker.recorded_method(&reply).is_none());

        assert_eq!(
            tracker.recorded_method(&call).as_deref(),
            Some("GetPrimary")
        );
        let error = zbus::Message::error(&call.header(), "de.pengutronix.rauc.Error.Failed")?
            .sender(":1.42")?
            .build(&"Failed")?;
        assert_eq!(
            tracker.recorded_method(&error).as_deref(),
            Some("GetPrimary")
        );
        let signal = zbus::Message::signal("/", INSTALLER_INTERFACE, "Completed")?
            .sender(":1.42")?
            .build(&(0i32,))?;
        assert_eq!(
            tracker.recorded_method(&signal).as_deref(),
            Some("Completed")
        );
        Ok(())
    }

    #[test]
    fn records_only_installer_property_changes() -> Result<(), Box<dyn std::error::Error>> {
        let mut tracker = ReplyTracker {
            owner: ":1.42".into(),
            pending: HashMap::new(),
        };
        for (sender, interface, expected) in [
            (":1.42", INSTALLER_INTERFACE, Some("PropertiesChanged")),
            (":1.99", INSTALLER_INTERFACE, None),
            (":1.42", "org.example.Other", None),
        ] {
            let changed = HashMap::from([("Progress", zbus::zvariant::Value::from(42i32))]);
            let signal = zbus::Message::signal("/", PROPERTIES_INTERFACE, "PropertiesChanged")?
                .sender(sender)?
                .build(&(interface, changed, vec!["Operation"]))?;
            assert_eq!(tracker.recorded_method(&signal).as_deref(), expected);
        }
        Ok(())
    }

    #[test]
    fn records_property_calls_and_replies() -> Result<(), Box<dyn std::error::Error>> {
        for destination in [RAUC_SERVICE, ":1.42", "org.example.Other"] {
            for interface in [INSTALLER_INTERFACE, "org.example.Other"] {
                for member in ["Get", "GetAll", "Set"] {
                    let mut tracker = ReplyTracker {
                        owner: ":1.42".into(),
                        pending: HashMap::new(),
                    };
                    let builder = zbus::Message::method_call("/", member)?
                        .interface(PROPERTIES_INTERFACE)?
                        .destination(destination)?
                        .sender(":1.10")?;
                    let call = match member {
                        "Get" => builder.build(&(interface, "Operation"))?,
                        "GetAll" => builder.build(&(interface,))?,
                        _ => builder.build(&(
                            interface,
                            "Operation",
                            zbus::zvariant::Value::from("idle"),
                        ))?,
                    };
                    let expected =
                        if destination != "org.example.Other" && interface == INSTALLER_INTERFACE {
                            Some(member)
                        } else {
                            None
                        };
                    assert_eq!(tracker.recorded_method(&call).as_deref(), expected);
                    let reply = zbus::Message::method_return(&call.header())?
                        .sender(":1.42")?
                        .build(&())?;
                    assert_eq!(tracker.recorded_method(&reply).as_deref(), expected);
                    assert!(tracker.recorded_method(&reply).is_none());

                    assert_eq!(tracker.recorded_method(&call).as_deref(), expected);
                    let error =
                        zbus::Message::error(&call.header(), "org.freedesktop.DBus.Error.Failed")?
                            .sender(":1.42")?
                            .build(&"Failed")?;
                    assert_eq!(tracker.recorded_method(&error).as_deref(), expected);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn records_rauc_name_lookup_and_bus_replies() -> Result<(), Box<dyn std::error::Error>> {
        let mut tracker = ReplyTracker {
            owner: ":1.42".into(),
            pending: HashMap::new(),
        };
        for name in [RAUC_SERVICE, "org.example.Other"] {
            let call = zbus::Message::method_call("/org/freedesktop/DBus", "GetNameOwner")?
                .interface(BUS_SERVICE)?
                .destination(BUS_SERVICE)?
                .sender(":1.10")?
                .build(&(name,))?;
            let expected = (name == RAUC_SERVICE).then_some("GetNameOwner");
            assert_eq!(tracker.recorded_method(&call).as_deref(), expected);
            let wrong_sender = zbus::Message::method_return(&call.header())?
                .sender(":1.42")?
                .build(&(":1.42",))?;
            assert!(tracker.recorded_method(&wrong_sender).is_none());
            let reply = zbus::Message::method_return(&call.header())?
                .sender(BUS_SERVICE)?
                .build(&(":1.42",))?;
            assert_eq!(tracker.recorded_method(&reply).as_deref(), expected);
            assert!(tracker.recorded_method(&reply).is_none());
            assert_eq!(tracker.recorded_method(&call).as_deref(), expected);
            let error =
                zbus::Message::error(&call.header(), "org.freedesktop.DBus.Error.NameHasNoOwner")?
                    .sender(BUS_SERVICE)?
                    .build(&"No owner")?;
            assert_eq!(tracker.recorded_method(&error).as_deref(), expected);
        }
        Ok(())
    }

    #[test]
    fn json_preserves_raw_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let recorded = RecordedBody {
            signature: "ay".into(),
            body: vec![0, 1, 127, 128, 255],
            decoded: serde_json::json!([[0, 1, 127, 128, 255]]),
        };
        let json = serde_json::to_string_pretty(&recorded)?;
        let restored: RecordedBody = serde_json::from_str(&json)?;
        assert_eq!(restored.signature, recorded.signature);
        assert_eq!(restored.body, recorded.body);
        assert_eq!(restored.decoded, recorded.decoded);
        Ok(())
    }
}
