use crate::v0::{
    Member, PartialChannel, PartialMember,
    PartialRole, PartialServer, User,
};
use revolt_permissions::Override;

auto_derived!(
    pub struct AuditLogEntry {
        #[serde(rename = "_id")]
        pub id: String,

        pub server: String,
        pub reason: Option<String>,
        pub user: String,
        pub action: AuditLogEntryAction,
    }

    #[serde(tag = "type")]
    #[allow(clippy::large_enum_variant)]
    pub enum AuditLogEntryAction {
        MessageDelete {
            author: String,
            channel: String,
        },
        MessageBulkDelete {
            channel: String,
            count: usize,
        },
        BanCreate {
            user: String,
        },
        BanDelete {
            user: String,
        },
        ChannelCreate {
            channel: String,
            name: String,
        },
        ChannelEdit {
            channel: String,
            before: PartialChannel,
            after: PartialChannel,
        },
        ChannelRolePermissionsEdit {
            channel: String,
            role: String,
            permissions: Override,
        },
        ChannelDelete {
            channel: String,
            name: String,
        },
        MemberEdit {
            user: String,
            before: PartialMember,
            after: PartialMember,
        },
        MemberKick {
            user: String,
        },
        ServerEdit {
            before: PartialServer,
            after: PartialServer,
        },
        RoleEdit {
            role: String,
            before: PartialRole,
            after: PartialRole,
        },
        RoleCreate {
            role: String,
            name: String,
        },
        RoleDelete {
            role: String,
            name: String,
        },
        RolesReorder {
            positions: Vec<String>,
        },
        InviteDelete {
            invite: String,
            channel: String,
        },
        WebhookCreate {
            webhook: String,
            name: String,
            channel: String,
        },
        WebhookDelete {
            webhook: String,
            name: String,
            channel: String,
        },
        EmojiDelete {
            emoji: String,
            name: String,
        },
    }

    #[cfg_attr(feature = "validator", derive(validator::Validate))]
    #[cfg_attr(feature = "rocket", derive(rocket::FromForm))]
    pub struct OptionsAuditLogQuery {
        #[cfg_attr(feature = "validator", validate(length(min = 26, max = 26)))]
        pub user: Option<String>,
        pub r#type: Option<Vec<String>>,
        #[cfg_attr(feature = "validator", validate(length(min = 26, max = 26)))]
        pub before: Option<String>,
        #[cfg_attr(feature = "validator", validate(length(min = 26, max = 26)))]
        pub after: Option<String>,
        #[cfg_attr(feature = "validator", validate(range(min = 1, max = 100)))]
        pub limit: Option<i64>,
    }

    /// Response containing the audit log entries and the users involved
    pub struct AuditLogQueryResponse {
        /// List of audit logs
        pub audit_logs: Vec<AuditLogEntry>,
        /// List of users
        pub users: Vec<User>,
        /// List of members
        pub members: Vec<Member>,
    }
);
