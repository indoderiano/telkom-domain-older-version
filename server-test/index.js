const express = require('express')
const cors = require("cors")
const app = express()
const port = 3000

app.use(express.urlencoded({extended: true}))
app.use(express.json())
app.use(cors())


app.get('/', (req, res) => {
  console.log('fake api')
  setTimeout(() => {
    res.status(200).json("Fake Api Telkom Domain")
  }, 3000);
  // res.send('Fake Api Telkom Domain')
})

app.post('/user', (req, res) => {
  console.log(req.body)
  setTimeout(() => {
    res.send({
      username: "batman",
      email: "batman@mail.com",
      token: "asdfasdf",
        // name: "Batman",
        // age: 33
    })
  }, 2000)
})

// app.get('/user', (req, res) => {
//   res.status(401).json("error message")
// })


const { router } = require('./routes')
app.use(router)

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})