
class ControllerSettings {
    static get(req, res, next) {
        console.log(req.params)

        let data = {
            change_password: {
                enabled: false,
                html: ""
            },
            guardian_mfa_page: {
                enabled: false,
                html: ""
            },
            default_audience: "",
            default_directory: "",
            error_page: {
                html: "",
                show_log_link: false,
                url: "https://mycompany.org/error",
            },
            device_flow: {
                charset: "base20",
                mask: "****_****"
            },
            flags: {
                change_pwd_flow_v1: false,
                enable_apis_section: false,
                disable_impersonation: false,
                enable_client_connections: true,
                enable_pipeline2: true,
                allow_legacy_delegation_grant_types: false,
                allow_legacy_ro_grant_types: false,
                allow_legacy_tokeninfo_endpoint: false,
                enable_legacy_profile: false,
                enable_idtoken_api2: false,
                enable_public_signup_user_exists_error: false,
                enable_sso: false,
                allow_changing_enable_sso: false,
                disable_clickjack_protection_headers: false,
                no_disclose_enterprise_connections: false,
                enforce_client_authentication_on_passwordless_start: false,
                enable_adfs_waad_email_verification: false,
                revoke_refresh_token_grant: false,
                dashboard_log_streams_next: false,
                dashboard_insights_view: false
            },
            friendly_name: "My Company",
            picture_url: "https://mycompany.org/logo.png",
            support_email: "support@mycompany.org",
            support_url: "https://mycompany.org/support",
            allowed_logout_urls: [
                "https://mycompany.org/logoutCallback",
            ],
            session_lifetime: 168,
            idle_session_lifetime: 72,
            sandbox_version: "8",
            sandbox_versions_available: [
                "8",
                "4",
            ],
            default_redirection_uri: "",
            enabled_locales: [""],
            session_cookie: {
                mode: "persistent",
            },
        }
        
        setTimeout(() => {
            console.log("return data settings");
            res.send(data)
        }, 3000)
    }

    static update (req, res, next) {
        console.log(req.params)
        console.log(req.body)

        let data = {
            change_password: {
                enabled: false,
                html: ""
            },
            guardian_mfa_page: {
                enabled: false,
                html: ""
            },
            default_audience: "",
            default_directory: "",
            error_page: {
                html: "",
                show_log_link: false,
                url: "https://mycompany.org/erroredited",
            },
            device_flow: {
                charset: "base20",
                mask: "****_****"
            },
            flags: {
                change_pwd_flow_v1: false,
                enable_apis_section: false,
                disable_impersonation: false,
                enable_client_connections: true,
                enable_pipeline2: true,
                allow_legacy_delegation_grant_types: false,
                allow_legacy_ro_grant_types: false,
                allow_legacy_tokeninfo_endpoint: false,
                enable_legacy_profile: false,
                enable_idtoken_api2: false,
                enable_public_signup_user_exists_error: false,
                enable_sso: false,
                allow_changing_enable_sso: false,
                disable_clickjack_protection_headers: false,
                no_disclose_enterprise_connections: false,
                enforce_client_authentication_on_passwordless_start: false,
                enable_adfs_waad_email_verification: false,
                revoke_refresh_token_grant: false,
                dashboard_log_streams_next: false,
                dashboard_insights_view: false
            },
            friendly_name: "My Company Edited",
            picture_url: "https://mycompany.org/logoedited.png",
            support_email: "support@mycompanyedited.org",
            support_url: "https://mycompany.org/supportedited",
            allowed_logout_urls: [
                "https://mycompany.org/logoutCallbackedited",
            ],
            session_lifetime: 168,
            idle_session_lifetime: 72,
            sandbox_version: "8",
            sandbox_versions_available: [
                "8",
                "4",
            ],
            default_redirection_uri: "",
            enabled_locales: [""],
            session_cookie: {
                mode: "persistent",
            },
        }
        
        setTimeout(() => {
            console.log("return data settings");
            res.send(data)
        }, 3000)
    }

    static getMembers (req, res, next) {
        console.log(req.params)

        let data = [
            {
                username: "batman",
                email: "batman@mail.com",
                connection: "google-oauth2",
                roles: "admin",
                is_mfa: false,
            }
        ]

        setTimeout(() => {
            console.log("return data members")
            res.send(data)
        }, 3000);
    }

    static createMember (req, res, next) {
        console.log(req.body)

        let data = {
            message: "Create member succesful"
        }

        setTimeout(() => {
            res.send(data)
        }, 3000);
    }

    
}

module.exports={
    ControllerSettings
}