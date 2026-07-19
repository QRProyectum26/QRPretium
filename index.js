const { Application } = require('@nocobase/server');
const http = require('http');

const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL,
    dialectOptions: {
      ssl: {
        require: true,
        rejectUnauthorized: false // Esto permite la conexión segura con Neon
      }
    }
  },
});

const port = process.env.PORT || 10000;

// Creamos el servidor HTTP y lo ponemos a escuchar INMEDIATAMENTE
const server = http.createServer(app.callback());

server.listen(port, () => {
  console.log(`🚀 Puerto ${port} abierto exitosamente para Render`);
  
  // Una vez que el puerto está asegurado, inicializamos NocoBase de fondo
  app.parse()
    .then(async () => {
      await app.load();
      console.log('🤖 NocoBase se ha inicializado y cargado correctamente.');
    })
    .catch((err) => {
      console.error('❌ Error inicializando NocoBase:', err);
    });
});
