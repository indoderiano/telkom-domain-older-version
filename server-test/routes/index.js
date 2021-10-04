const express = require('express')
const { RouterApi } = require('./RouterApi')
const { RouterSettings } = require('./RouterSettings')

const router = express.Router()

router.use('/api', RouterApi)
router.use('/tenant', RouterSettings)

module.exports={
    router
}