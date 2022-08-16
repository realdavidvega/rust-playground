db.createUser(
    {
        user: "test",
        pwd: "12345",
        roles: [
            {
                role: "readWrite",
                db: "my-db"
            }
        ]
    }
)