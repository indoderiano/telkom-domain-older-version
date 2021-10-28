class ControllerUsers {
    static get(req, res, next) {
        console.log(req.params.tenant_id);

        let data = {
            message: "Ok",
            data: [{
                    user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                    email: "yeskahaganta3838@gmail.com",
                    email_verified: false,
                    username: "abangyeska",
                    phone_number: "+6281239675121",
                    phone_verified: false,
                    created_at: "2021-09-07T07:17:53.901Z",
                    updated_at: "2021-09-28T07:16:17.611Z",
                    identities: [{
                        connection: "Initial-Connection",
                        user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                        provider: "auth0",
                        is_social: false,
                    }, ],
                    app_metadata: {},
                    user_metadata: {},
                    picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
                    name: "Yeska Haganta",
                    nickname: "brotheryeska",
                    multifactor: [""],
                    last_ip: "36.92.133.42",
                    last_login: "2021-09-28T07:16:17.610Z",
                    logins_count: 1,
                    blocked: false,
                    given_name: "Yeska",
                    family_name: "Sembiring",
                },
                {
                    user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmK",
                    email: "fransyogabarus@gmail.com",
                    email_verified: false,
                    username: "abangyoga",
                    phone_number: "+628188812121212",
                    phone_verified: false,
                    created_at: "2021-09-07T07:17:58.901Z",
                    updated_at: "2021-09-28T07:16:19.611Z",
                    identities: [{
                        connection: "Initial-Connection2",
                        user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmK",
                        provider: "auth0",
                        is_social: false,
                    }, ],
                    app_metadata: {},
                    user_metadata: {},
                    picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
                    name: "Frans Yoga",
                    nickname: "abangagoy",
                    multifactor: [""],
                    last_ip: "37.92.133.44",
                    last_login: "2021-09-28T07:17:17.610Z",
                    logins_count: 1,
                    blocked: false,
                    given_name: "Frans",
                    family_name: "Barus",
                },
            ],
        };

        setTimeout(() => {
            console.log("return users data");
            res.send(data);
        }, 1000);
    }

    static create(req, res, next) {
        console.log(req.params.tenant_id);
        console.log(req.body);

        setTimeout(() => {
            res.send({
                message: "create succesful",
                data: "",
            });
        });
    }

    static getDetails(req, res, next) {
        console.log(req.params, "ini di get details");

        let data = {
            user_id: "auth0|YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
            email: "yeskahaganta3838@gmail.com",
            email_verified: false,
            username: "abangyeska",
            phone_number: "+6281239675121",
            phone_verified: false,
            created_at: "2021-09-07T07:17:53.901Z",
            updated_at: "",
            identities: [{
                connection: "Initial-Connection",
                user_id: "YXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI",
                provider: "auth0",
                is_social: false,
            }, ],
            app_metadata: {},
            user_metadata: {},
            picture: "https://s.gravatar.com/avatar/be4a14d4c9b0cd03d487ac71f40d6047?s=480&r=pg&d=https%3A%2F%2Fcdn.auth0.com%2Favatars%2Fye.png",
            name: "Yeska Haganta",
            nickname: "brotheryeska",
            multifactor: [""],
            last_ip: "36.92.133.42",
            last_login: "2021-09-28T07:16:17.610Z",
            logins_count: 1,
            blocked: false,
            given_name: "Yeska",
            family_name: "Sembiring",
        };

        setTimeout(() => {
            console.log("return data user details");
            res.send(data);
        }, 3000);
    }
}

module.exports = {
    ControllerUsers,
};