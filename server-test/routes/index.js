const express = require('express')
const { RouterApi } = require('./RouterApi')
const { RouterApp } = require('./RouterApp')
const { RouterSettings } = require('./RouterSettings')
const { RouterRoles } = require('./RouterRoles')

const router = express.Router()

router.use('/api', RouterApi)
router.use('/api/v1/1/clients', RouterApp)
router.use('/tenant', RouterSettings)
router.use('/roles', RouterRoles)

module.exports={
    router
}