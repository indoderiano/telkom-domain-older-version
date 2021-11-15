const express = require('express')
const { ControllerUsers } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')


const RouterUser = express.Router()


RouterUser.get('/:tenant_id', Authentication, ControllerUsers.get)
RouterUser.post('/:tenant_id', Authentication, ControllerUsers.create)
RouterUser.get('/:tenant_id/users/:id', Authentication, ControllerUsers.getDetails)
RouterUser.patch(':/tenant_id/users/:id', Authentication, ControllerUsers.updateUsers)
RouterUser.get('/:tenant_id/users/:id/logs', Authentication, ControllerUsers.getLogEvent)
RouterUser.get('/:tenant_id/users/:id/roles', Authentication, ControllerUsers.getRoles)
RouterUser.get('/:tenant_id/users/:id/permissions', Authentication, ControllerUsers.getUserPermissions)
RouterUser.delete('/:tenant_id/users/:id', Authentication, ControllerUsers.deleteUser)
RouterUser.delete('/:tenant_id/users/:id/roles', Authentication, ControllerUsers.deleteRolesFromUser)
RouterUser.delete('/:tenant_id/users/:id/permissions', Authentication, ControllerUsers.deletePermissionsFromUser)












module.exports = {
    RouterUser
}