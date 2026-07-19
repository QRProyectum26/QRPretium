const { Application } = require('@nocobase/server');

const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL || 'postgres://localhost:5432/nocobase',
  },
});

const port = process.env.PORT || 10000;

app.runAsCLI();
