const express = require('express')
const { ControllerApp } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')

const RouterApp = express.Router()

RouterApp.get('/:tenant_id', Authentication, ControllerApp.get)
RouterApp.post('/:tenant_id', Authentication, ControllerApp.create)
RouterApp.get('/:tenant_id/applications/:id', Authentication, ControllerApp.getDetails)
RouterApp.put('/:tenant_id/applications/:id', Authentication, ControllerApp.updateDetails)
RouterApp.delete('/:tenant_id/applications/:id', Authentication, ControllerApp.deleteDetails)

module.exports={
    RouterApp
}