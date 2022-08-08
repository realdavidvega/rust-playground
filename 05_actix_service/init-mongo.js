db.createUser(
    {
        user: "test",
        password: "12345",
        roles: [
            {
                role: "readWrite",
                db: "my-db"
            }
        ]
    }
)