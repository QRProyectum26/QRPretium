const { Application } = require('@nocobase/server');
const http = require('http');

const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL || 'postgres://localhost:5432/nocobase',
  },
});

const port = process.env.PORT || 10000;

// Inicializamos NocoBase primero
app.parse().then(async () => {
  await app.load();
  
  // Creamos el servidor HTTP nativo usando el callback de la app
  const server = http.createServer(app.callback());
  
  server.listen(port, () => {
    console.log(`🚀 Servidor de NocoBase escuchando firmemente en el puerto ${port}`);
  });
}).catch((err) => {
  console.error('❌ Error en el ciclo de vida de NocoBase:', err);
});
