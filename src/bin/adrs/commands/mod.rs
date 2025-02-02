pub const ADR_DEFAULT_DIRECTORY: &str = "doc/adr";
const NYGARD_TEMPLATE: &'static str = include_str!("../../../templates/adr-nygard.hbs");
const NYGARD_TEMPLATE_INIT: &'static str = include_str!("../../../templates/adr-nygard-init.hbs");

pub(crate) mod init;
pub(crate) mod new;
