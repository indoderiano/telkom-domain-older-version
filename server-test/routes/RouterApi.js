const express = require('express')
const { ControllerApi } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')
// const {ControllerProduct} = require('../Controllers')
// const {AuthenticationAdmin} = require('../middlewares/Authentications')
// const multer  = require('multer')
// const upload = multer({})

const RouterApi = express.Router()

RouterApi.get('/:tenant_id', Authentication, ControllerApi.get)
RouterApi.post('/:tenant_id', Authentication, ControllerApi.create)
RouterApi.get('/:tenant_id/apis/:id', Authentication, ControllerApi.getDetails)
RouterApi.put('/:tenant_id/apis/:id', Authentication, ControllerApi.updateDetails)

// RouterProduct.post('/', AuthenticationAdmin, ControllerProduct.create)
// RouterProduct.get('/', ControllerProduct.readAll)
// RouterProduct.put('/:id', AuthenticationAdmin, ControllerProduct.edit)
// RouterProduct.delete('/:id', AuthenticationAdmin, ControllerProduct.delete)

// RouterProduct.post('/cloudinary', AuthenticationAdmin, upload.array('photo'), ControllerProduct.createWithCloudinary)
// RouterProduct.put('/cloudinary/:id', AuthenticationAdmin, upload.array('photo'), ControllerProduct.editWithCloudinary)
// RouterProduct.delete('/cloudinary/:id', AuthenticationAdmin, ControllerProduct.deleteWithCloudinary)

module.exports={
    RouterApi
}