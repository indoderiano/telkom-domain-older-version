const express = require('express')
const { RouterApi } = require('./RouterApi')
const { RouterSettings } = require('./RouterSettings')
const { RouterRoles } = require('./RouterRoles')

const router = express.Router()

router.use('/api', RouterApi)
router.use('/tenant', RouterSettings)
router.use('/roles', RouterRoles)

module.exports={
    router
}