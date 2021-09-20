
const Authentication = (req, res, next) => {
    let { access_token } = req.headers
    
    if (access_token) {
        console.log(`access token: ${access_token}`)
        console.log("Authentication Success")
        next()
    } else {
        res.status(400).json({
            message: "Auth failed"
        })
    }
}

module.exports={
    Authentication
}