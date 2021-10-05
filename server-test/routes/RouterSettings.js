const express = require('express')
const { ControllerSettings } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')

const RouterSettings = express.Router()

RouterSettings.get('/v2/settings', Authentication, ControllerSettings.get)
RouterSettings.patch('/v2/settings', Authentication, ControllerSettings.update)

module.exports={
    RouterSettings,
}