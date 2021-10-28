const express = require('express')
const { ControllerUsers } = require('../controllers')
const { Authentication } = require('../middlewares/authentication')


const RouterUser = express.Router()


RouterUser.get('/:tenant_id', Authentication, ControllerUsers.get)
RouterUser.post('/:tenant_id', Authentication, ControllerUsers.create)
RouterUser.get('/:tenant_id/users/:id', Authentication, ControllerUsers.getDetails)













module.exports = {
    RouterUser
}