const express = require('express')
const { ControllerApi } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')
// const {ControllerProduct} = require('../Controllers')
// const {AuthenticationAdmin} = require('../middlewares/Authentications')
// const multer  = require('multer')
// const upload = multer({})

const RouterApi = express.Router()

RouterApi.get('/v2/resource-servers/:tenant_id', Authentication, ControllerApi.get)
RouterApi.post('/v2/resource-servers/:tenant_id', Authentication, ControllerApi.create)
RouterApi.get('/v2/:tenant_id/resource-servers/:id', Authentication, ControllerApi.getDetails)
RouterApi.patch('/v2/:tenant_id/resource-servers/:id', Authentication, ControllerApi.updateDetails)
RouterApi.delete('/v2/:tenant_id/resource-servers/:id', Authentication, ControllerApi.deleteDetails)

module.exports={
    RouterApi
}