
class ControllerApp {
    static get(req, res, next){
        console.log(req.params.tenant_id)
        let data = {
            message: "App fetched",
            data: [
                {
                    "client_id": "AaiyAPdpYdesoKnqjj8HJqRn4T5titww",
                    "tenant": "",
                    "name": "My application 1",
                    "description": "Machine to Machine",
                    "global": false,
                    "client_secret": "MG_TNT2ver-SylNat-_VeMmd-4m0Waba0jr1troztBniSChEw0glxEmgEi2Kw40H",
                    "app_type": "Machine to Machine",
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/non_interactive.svg",
                    "is_first_party": false,
                    "oidc_conformant": false,
                    "callbacks": [
                    "http://localhost/callback"
                    ],
                    "allowed_origins": [
                    ""
                    ],
                    "web_origins": [
                    ""
                    ],
                    "client_aliases": [
                    ""
                    ],
                    "allowed_clients": [
                    ""
                    ],
                    "allowed_logout_urls": [
                    "http://localhost/logoutCallback"
                    ],
                    "grant_types": [
                    ""
                    ],
                    "jwt_configuration": {
                    "lifetime_in_seconds": 36000,
                    "secret_encoded": true,
                    "scopes": {},
                    "alg": "HS256"
                    },
                    "signing_keys": [
                    "object"
                    ],
                    "encryption_key": {
                    "pub": "",
                    "cert": "",
                    "subject": ""
                    },
                    "sso": false,
                    "sso_disabled": false,
                    "cross_origin_auth": false,
                    "cross_origin_loc": "",
                    "custom_login_page_on": true,
                    "custom_login_page": "",
                    "custom_login_page_preview": "",
                    "form_template": "",
                    "addons": {
                    "aws": {},
                    "azure_blob": {},
                    "azure_sb": {},
                    "rms": {},
                    "mscrm": {},
                    "slack": {},
                    "sentry": {},
                    "box": {},
                    "cloudbees": {},
                    "concur": {},
                    "dropbox": {},
                    "echosign": {},
                    "egnyte": {},
                    "firebase": {},
                    "newrelic": {},
                    "office365": {},
                    "salesforce": {},
                    "salesforce_api": {},
                    "salesforce_sandbox_api": {},
                    "samlp": {},
                    "layer": {},
                    "sap_api": {},
                    "sharepoint": {},
                    "springcm": {},
                    "wams": {},
                    "wsfed": {},
                    "zendesk": {},
                    "zoom": {},
                    "sso_integration": "object"
                    },
                    "token_endpoint_auth_method": "none",
                    "client_metadata": {},
                    "mobile": {
                    "android": {
                        "app_package_name": "com.example",
                        "sha256_cert_fingerprints": [
                        "D8:A0:83:..."
                        ]
                    },
                    "ios": {
                        "team_id": "9JA89QQLNQ",
                        "app_bundle_identifier": "com.my.bundle.id"
                    }
                    },
                    "initiate_login_uri": "",
                    "native_social_login": {
                    "apple": "object",
                    "facebook": "object"
                    },
                    "refresh_token": {
                    "rotation_type": "non-rotating",
                    "expiration_type": "non-expiring",
                    "leeway": 0,
                    "token_lifetime": 0,
                    "infinite_token_lifetime": false,
                    "idle_token_lifetime": 0,
                    "infinite_idle_token_lifetime": false
                    },
                    "organization_usage": "deny",
                    "organization_require_behavior": "no_prompt"
                },
                {
                    "client_id": "JklAbdpYdesokmZWay87AAqRn4T5titw",
                    "tenant": "",
                    "name": "My application 2",
                    "description": "Single Page Application",
                    "global": false,
                    "client_secret": "MG_TNT2ver-SylNat-_VeMmd-4m0Waba0jr1troztBniSChEw0glxEmgEi2Kw40H",
                    "app_type": "Single Page Application",
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/spa.svg",
                    "is_first_party": false,
                    "oidc_conformant": false,
                    "callbacks": [
                    "http://localhost/callback"
                    ],
                    "allowed_origins": [
                    ""
                    ],
                    "web_origins": [
                    ""
                    ],
                    "client_aliases": [
                    ""
                    ],
                    "allowed_clients": [
                    ""
                    ],
                    "allowed_logout_urls": [
                    "http://localhost/logoutCallback"
                    ],
                    "grant_types": [
                    ""
                    ],
                    "jwt_configuration": {
                    "lifetime_in_seconds": 36000,
                    "secret_encoded": true,
                    "scopes": {},
                    "alg": "HS256"
                    },
                    "signing_keys": [
                    "object"
                    ],
                    "encryption_key": {
                    "pub": "",
                    "cert": "",
                    "subject": ""
                    },
                    "sso": false,
                    "sso_disabled": false,
                    "cross_origin_auth": false,
                    "cross_origin_loc": "",
                    "custom_login_page_on": true,
                    "custom_login_page": "",
                    "custom_login_page_preview": "",
                    "form_template": "",
                    "addons": {
                    "aws": {},
                    "azure_blob": {},
                    "azure_sb": {},
                    "rms": {},
                    "mscrm": {},
                    "slack": {},
                    "sentry": {},
                    "box": {},
                    "cloudbees": {},
                    "concur": {},
                    "dropbox": {},
                    "echosign": {},
                    "egnyte": {},
                    "firebase": {},
                    "newrelic": {},
                    "office365": {},
                    "salesforce": {},
                    "salesforce_api": {},
                    "salesforce_sandbox_api": {},
                    "samlp": {},
                    "layer": {},
                    "sap_api": {},
                    "sharepoint": {},
                    "springcm": {},
                    "wams": {},
                    "wsfed": {},
                    "zendesk": {},
                    "zoom": {},
                    "sso_integration": "object"
                    },
                    "token_endpoint_auth_method": "none",
                    "client_metadata": {},
                    "mobile": {
                    "android": {
                        "app_package_name": "com.example",
                        "sha256_cert_fingerprints": [
                        "D8:A0:83:..."
                        ]
                    },
                    "ios": {
                        "team_id": "9JA89QQLNQ",
                        "app_bundle_identifier": "com.my.bundle.id"
                    }
                    },
                    "initiate_login_uri": "",
                    "native_social_login": {
                    "apple": "object",
                    "facebook": "object"
                    },
                    "refresh_token": {
                    "rotation_type": "non-rotating",
                    "expiration_type": "non-expiring",
                    "leeway": 0,
                    "token_lifetime": 0,
                    "infinite_token_lifetime": false,
                    "idle_token_lifetime": 0,
                    "infinite_idle_token_lifetime": false
                    },
                    "organization_usage": "deny",
                    "organization_require_behavior": "no_prompt"
                },
                {
                    "client_id": "ABCFiyAPdpYdesoKnj8HJqRn4T5titww",
                    "tenant": "",
                    "name": "My application 3",
                    "description": "Mobile Application",
                    "global": false,
                    "client_secret": "MG_TNT2ver-SylNat-_VeMmd-4m0Waba0jr1troztBniSChEw0glxEmgEi2Kw40H",
                    "app_type": "Mobile Application",
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/none.svg",
                    "is_first_party": false,
                    "oidc_conformant": false,
                    "callbacks": [
                    "http://localhost/callback"
                    ],
                    "allowed_origins": [
                    ""
                    ],
                    "web_origins": [
                    ""
                    ],
                    "client_aliases": [
                    ""
                    ],
                    "allowed_clients": [
                    ""
                    ],
                    "allowed_logout_urls": [
                    "http://localhost/logoutCallback"
                    ],
                    "grant_types": [
                    ""
                    ],
                    "jwt_configuration": {
                    "lifetime_in_seconds": 36000,
                    "secret_encoded": true,
                    "scopes": {},
                    "alg": "HS256"
                    },
                    "signing_keys": [
                    "object"
                    ],
                    "encryption_key": {
                    "pub": "",
                    "cert": "",
                    "subject": ""
                    },
                    "sso": false,
                    "sso_disabled": false,
                    "cross_origin_auth": false,
                    "cross_origin_loc": "",
                    "custom_login_page_on": true,
                    "custom_login_page": "",
                    "custom_login_page_preview": "",
                    "form_template": "",
                    "addons": {
                    "aws": {},
                    "azure_blob": {},
                    "azure_sb": {},
                    "rms": {},
                    "mscrm": {},
                    "slack": {},
                    "sentry": {},
                    "box": {},
                    "cloudbees": {},
                    "concur": {},
                    "dropbox": {},
                    "echosign": {},
                    "egnyte": {},
                    "firebase": {},
                    "newrelic": {},
                    "office365": {},
                    "salesforce": {},
                    "salesforce_api": {},
                    "salesforce_sandbox_api": {},
                    "samlp": {},
                    "layer": {},
                    "sap_api": {},
                    "sharepoint": {},
                    "springcm": {},
                    "wams": {},
                    "wsfed": {},
                    "zendesk": {},
                    "zoom": {},
                    "sso_integration": "object"
                    },
                    "token_endpoint_auth_method": "none",
                    "client_metadata": {},
                    "mobile": {
                    "android": {
                        "app_package_name": "com.example",
                        "sha256_cert_fingerprints": [
                        "D8:A0:83:..."
                        ]
                    },
                    "ios": {
                        "team_id": "9JA89QQLNQ",
                        "app_bundle_identifier": "com.my.bundle.id"
                    }
                    },
                    "initiate_login_uri": "",
                    "native_social_login": {
                    "apple": "object",
                    "facebook": "object"
                    },
                    "refresh_token": {
                    "rotation_type": "non-rotating",
                    "expiration_type": "non-expiring",
                    "leeway": 0,
                    "token_lifetime": 0,
                    "infinite_token_lifetime": false,
                    "idle_token_lifetime": 0,
                    "infinite_idle_token_lifetime": false
                    },
                    "organization_usage": "deny",
                    "organization_require_behavior": "no_prompt"
                }
            ]
        }

        setTimeout(() => {
            console.log("return data");
            res.send(data)

            // IF ERROR
            // console.log("return data error");
            // res.status(400).send({
            //     message: "Internal server error",
            //     data: ""
            // })
        }, 3000)

    }

    static create(req, res, next) {
        console.log(req.params.tenant_id)
        console.log(req.body)

        setTimeout(() => {
            res.send({
                message: "create succesful",
                data: "",
            })
        }, 3000)
    }

    static getDetails(req, res, next) {

        console.log(req.params)

        let data = {
            message: "App loaded",
            data: {                
                tenant: "dev-1wj84p4q",
                name: "My App 1",
                domain: "dev-test-telkom.us.auth0.com",
                client_id: "EsKRzYf48U2kgbFtAb5rVz4YuvKHF92s",
                client_secret: "6ERYcgr8moH4K28QKsj1qsHjVHhPojiUhi1laphUKM4KoQ0Av6CIbRvrr_Svw_U2",
                description: "This is description",
                app_logo: "https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/badge.png",
                app_type: "Single Page Application",
                authentication_method: "",
                login_url: "https://telkom-domain.com/login",
                allowed_urls : "https://telkom-domain.com/login, https://telkom-domain.co.id/login",
                allowed_logout_urls : "https://telkom-domain.com/logout",
                allowed_web_origins: "https://login.mydomain.com",
                allowed_origins: "https://telkom-domain.com/login, https://telkom-domain.co.id/login",
                token_exp: 36000,
                refresh_token_rotation: true,
                refresh_token_rotation_interval: 0,
                refesh_token_absolute_expiration: true,
                refesh_token_absolute_expiration_lifetime: 2592000,
                refesh_token_inactivity_expiration: true,
                refesh_token_inactivity_expiration_lifetime: 1296000,
            }
        }

        setTimeout(() => {
            console.log("return data app details");
            res.send(data)
        }, 3000)

    }

    static updateDetails(req, res, next) {

        console.log(req.params)
        console.log(req.body)
        let data = {
            message: "App loaded",
            data: {                
                tenant: "dev-1wj84p4q",
                name: "My App 1 edited",
                domain: "dev-test-telkom.us.auth0.com edited",
                client_id: "EsKRzYf48U2kgbFtAb5rVz4YuvKHF92s edited",
                client_secret: "6ERYcgr8moH4K28QKsj1qsHjVHhPojiUhi1laphUKM4KoQ0Av6CIbRvrr_Svw_U2 edited",
                description: "This is description edited",
                app_logo: "https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/badge.png",
                app_type: "Single Page Application",
                authentication_method: "",
                login_url: "https://telkom-domain.com/login",
                allowed_urls : "https://telkom-domain.com/login, https://telkom-domain.co.id/login",
                allowed_logout_urls : "https://telkom-domain.com/logout",
                allowed_web_origins: "https://login.mydomain.com",
                allowed_origins: "https://telkom-domain.com/login, https://telkom-domain.co.id/login",
                token_exp: 36000,
                refresh_token_rotation: true,
                refresh_token_rotation_interval: 0,
                refesh_token_absolute_expiration: true,
                refesh_token_absolute_expiration_lifetime: 2592000,
                refesh_token_inactivity_expiration: true,
                refesh_token_inactivity_expiration_lifetime: 1296000,
            }
        }

        setTimeout(() => {
            console.log("return data api details");
            res.send(data)
        }, 3000)
    }

    static deleteDetails(req, res, next) {
        console.log("masuk delete application")
        console.log(req.params)

        setTimeout(() => {
            console.log("return status");
            res.send({
                message: "Api deleted",
                data: "",
            })
        }, 3000)
    }
    
}

module.exports={
    ControllerApp
}