const express = require('express')
const { ControllerUsers } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')


const RouterUser = express.Router()


RouterUser.get('/:tenant_id', Authentication, ControllerUsers.get)
RouterUser.post('/:tenant_id', Authentication, ControllerUsers.create)
RouterUser.get('/:tenant_id/users/:id', Authentication, ControllerUsers.getDetails)
RouterUser.get('/:tenant_id/users/:id/logs', Authentication, ControllerUsers.getLogEvent)
RouterUser.get('/:tenant_id/users/:id/roles', Authentication, ControllerUsers.getRoles)
RouterUser.get('/:tenant_id/users/:id/permissions', Authentication, ControllerUsers.getUserPermissions)












module.exports = {
    RouterUser
}