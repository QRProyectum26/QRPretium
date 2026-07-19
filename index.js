const app = new Application({
  // ... si tenías otras configuraciones acá, dejalas igual ...
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL,
    dialectOptions: {
      ssl: {
        require: true,
        rejectUnauthorized: false
      }
    }
  }
});
