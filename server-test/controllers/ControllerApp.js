
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
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3312.0/assets/badge.png",
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
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3312.0/assets/badge.png",
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
                    "logo_uri": "https://cdn.auth0.com/manhattan/versions/1.3312.0/assets/badge.png",
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
            message: "Api loaded",
            data: {
                id: 1,
                name: "Auth0 Management API",
                api_id: "60daccd6dff9a6003e8ef6ef",
                api_type: "System API",
                identifier: "https://dev-r5y8heyf.au.auth0.com/api/v2/",
                token_exp: 100000,
                token_exp_browser: 10000,
                sign_algorithm: "algorithm signing",
                rbac: true,
                permission_acc_token: true,
                allow_skip_user: true,
                allow_off_acc: true,
                tenant_id: "dev-ofzd5p1b"
            }
        }

        setTimeout(() => {
            console.log("return data api details");
            res.send(data)
        }, 3000)

    }

    static updateDetails(req, res, next) {

        console.log(req.params)

        let data = {
            message: "Api loaded",
            data: {
                id: 1,
                name: "Auth0 Management API edited",
                api_id: "60daccd6dff9a6003e8ef6ef",
                api_type: "System API",
                identifier: "https://dev-r5y8heyf-edited.au.auth0.com/api/v2/",
                token_exp: 100000,
                token_exp_browser: 10000,
                sign_algorithm: "algorithm signing",
                rbac: true,
                permission_acc_token: true,
                allow_skip_user: true,
                allow_off_acc: true,
                tenant_id: "dev-ofzd5p1b"
            }
        }

        setTimeout(() => {
            console.log("return data api details");
            res.send(data)
        }, 3000)
    }

    static deleteDetails(req, res, next) {

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