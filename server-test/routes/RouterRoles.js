const express = require('express')
const { ControllerRoles } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')

const RouterRoles = express.Router()


RouterRoles.get('/v2/:id', Authentication, ControllerRoles.getRoleDetails)
RouterRoles.put('/v2/:id', Authentication, ControllerRoles.updateRoleDetails)
RouterRoles.delete('/v2/:id', Authentication, ControllerRoles.deleteRole)
RouterRoles.get('/v2', Authentication, ControllerRoles.getRoles)
RouterRoles.post('/v2', Authentication, ControllerRoles.createRole)


module.exports={
    RouterRoles,
}