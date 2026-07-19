const { Application } = require('@nocobase/server');

const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL || 'postgres://localhost:5432/nocobase',
  },
});

const port = process.env.PORT || 10000;

// Cambiamos el runAsCLI por el arranque directo del servidor
app.start({
  port: parseInt(port)
}).then(() => {
  console.log(`🚀 Servidor de NocoBase encendido en el puerto ${port}`);
}).catch((err) => {
  console.error('❌ Error al encender NocoBase:', err);
});
