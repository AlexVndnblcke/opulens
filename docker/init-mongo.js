// Fetch environment variables
const dbName = process.env.MONGODB_INITDB_DATABASE;
const dbUser = process.env.MONGODB_OPULENS_USER;
const dbPassword = process.env.MONGODB_OPULENS_PASSWORD;

// Connect to the specified database
db = db.getSiblingDB(dbName);

// Create a non-root user
db.createUser({
  user: dbUser,
  pwd: dbPassword,
  roles: [
    { role: "readWrite", db: dbName } // Grant only necessary permissions
  ]
});

print(`User ${dbUser} created with readWrite access on ${dbName}`);
