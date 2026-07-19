const { Application } = require('@nocobase/server');
const http = require('http');

const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL || 'postgres://localhost:5432/nocobase',
  },
});

const port = process.env.PORT || 10000;

// Creamos el servidor HTTP nativo acoplado a NocoBase
const server = http.createServer(app.callback());

// Escuchamos en el puerto inmediatamente para conformar a Render
server.listen(port, () => {
  console.log(`🚀 Puerto ${port} abierto exitosamente para Render`);
  
  // Cargamos NocoBase de fondo una vez asegurado el puerto
  app.load()
    .then(() => {
      console.log('🤖 NocoBase se ha inicializado y cargado correctamente.');
    })
    .catch((err) => {
      console.error('❌ Error cargando NocoBase:', err);
    });
});
