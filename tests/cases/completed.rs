use rauc::InstallationResult;
use zbus::Message;

#[test]
fn decodes_installation_results() {
    for (code, expected) in [
        (0, InstallationResult::Success),
        (1, InstallationResult::Failure(1)),
        (-1, InstallationResult::Failure(-1)),
        (42, InstallationResult::Failure(42)),
    ] {
        let message = Message::signal("/", "de.pengutronix.rauc.Installer", "Completed")
            .expect("invalid signal header")
            .build(&(code,))
            .expect("failed to build completion signal");
        let (result,): (InstallationResult,) = message
            .body()
            .deserialize()
            .expect("failed to decode installation result");
        assert_eq!(result, expected);
    }
}
