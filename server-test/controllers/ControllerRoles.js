
class ControllerRoles {
    static getRoles(req, res, next){
        console.log("get roles list")

        let data = [
            {
                id: "adskjfhaldsfh",
                name: "Admin",
                description: "Curabitur rhoncus, augue quis suscipit blandit, metus erat fermentum tellus, eget viverra velit augue eget eros",
            },
            {
                id: "asdasasdd",
                name: "Agent",
                description: "Vivamus ut suscipit velit. Aenean a est rhoncus, bibendum augue et, molestie ligula"
            },
        ]

        setTimeout(() => {
            console.log("return roles");
            res.send(data)
        }, 3000)
    }

    static createRole(req, res, next){
        console.log("create role")
        console.log(req.body)

        let data = [
            {
                id: "adskjfhaldsfh",
                name: "Admin",
                description: "Curabitur rhoncus, augue quis suscipit blandit, metus erat fermentum tellus, eget viverra velit augue eget eros",
            },
            {
                id: "asdasasdd",
                name: "Agent",
                description: "Vivamus ut suscipit velit. Aenean a est rhoncus, bibendum augue et, molestie ligula"
            },
            {
                id: "aasdsdsdsdsd",
                name: "New Role",
                description: "Cras at tortor ac elit consequat pulvinar sed nec turpis"
            },
        ]

        setTimeout(() => {
            console.log("return role details");
            res.send(data)
        }, 3000)
    }

    static getRoleDetails(req, res, next){
        console.log("get role details")
        console.log(req.params)

        let data = {
            id: "adskjfhaldsfh",
            name: "Admin",
            description: "Curabitur rhoncus, augue quis suscipit blandit, metus erat fermentum tellus, eget viverra velit augue eget eros",
        }

        setTimeout(() => {
            console.log("return role details");
            res.send(data)
        }, 3000)
    }

    static updateRoleDetails(req, res, next){
        console.log("update role details")
        console.log(req.params)
        console.log(req.body)

        let data = {
            id: "adskjfhaldsfhedited",
            name: "Adminedited",
            description: "Edited Curabitur rhoncus, augue quis suscipit blandit, metus erat fermentum tellus, eget viverra velit augue eget eros",
        }

        setTimeout(() => {
            console.log("return role details");
            res.send(data)
        }, 3000)
    }

    static deleteRole(req, res, next){
        console.log("delete a role")
        console.log(req.params)

        res.status(201)

    }
    
}

module.exports={
    ControllerRoles
}