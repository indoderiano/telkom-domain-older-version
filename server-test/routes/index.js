const express = require('express')
const { RouterApi } = require('./RouterApi')
const { RouterApp } = require('./RouterApp')
const { RouterUser } = require('./RouterUser')
    // const {RouterProduct} = require('./RouterProduct')
    // const {RouterBanner} = require('./RouterBanner')
    // const {RouterTransaction} = require('./RouterTransaction')
    // const {RouterWishlist} = require('./RouterWishlist')
const { RouterSettings } = require('./RouterSettings')
const { RouterRoles } = require('./RouterRoles')

const router = express.Router()

router.use('/api', RouterApi)
router.use('/api/v1/1/clients', RouterApp)

router.use('/users', RouterUser)
    // router.use('/product', RouterProduct)
    // router.use('/banner', RouterBanner)
    // router.use('/transaction', RouterTransaction)
    // router.use('/wishlist', RouterWishlist)
router.use('/tenant', RouterSettings)
router.use('/roles', RouterRoles)

module.exports = {
    router
}