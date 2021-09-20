const express = require('express')
const { RouterApi } = require('./RouterApi')
// const {RouterUser} = require('./RouterUser')
// const {RouterProduct} = require('./RouterProduct')
// const {RouterBanner} = require('./RouterBanner')
// const {RouterTransaction} = require('./RouterTransaction')
// const {RouterWishlist} = require('./RouterWishlist')

const router = express.Router()

router.use('/api', RouterApi)

// router.use('/user', RouterUser)
// router.use('/product', RouterProduct)
// router.use('/banner', RouterBanner)
// router.use('/transaction', RouterTransaction)
// router.use('/wishlist', RouterWishlist)

module.exports={
    router
}