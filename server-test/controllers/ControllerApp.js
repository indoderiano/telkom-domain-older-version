
class ControllerApp {
    static get(req, res, next){
        console.log(req.params.tenant_id)
        let data = {
            message: "App fetched",
            data: [
                {
                    id: 1,
                    name: "App Pertama",
                    client_id: "60daccd6dff9a6003e8ef6ef",
                    api_type: "Custom App 1",
                    logo_url: "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"
                },
                {
                    id: 2,
                    name: "App Pertama",
                    client_id: "60daccd6dff9a6003e8ef6ef",
                    api_type: "Custom App 2",
                    logo_url: "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"
                },
                {
                    id: 3,
                    name: "App Pertama",
                    client_id: "60daccd6dff9a6003e8ef6ef",
                    api_type: "Custom App 3",
                    logo_url: "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"
                },
                {
                    id: 4,
                    name: "App Pertama",
                    client_id: "60daccd6dff9a6003e8ef6ef",
                    api_type: "Custom App 4",
                    logo_url: "https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg"
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