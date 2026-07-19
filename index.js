console.log("Paso 1: Cargando dependencias...");
const { Application } = require('@nocobase/server');
const http = require('http');

console.log("Paso 2: Leyendo variables nativas de BD....");
console.log("Host objetivo:", process.env.DB_HOST);

console.log("Paso 3: Instanciando NocoBase...");
const app = new Application({
  database: {
    dialect: 'postgres',
    host: process.env.DB_HOST,
    database: process.env.DB_DATABASE,
    username: process.env.DB_USER,
    password: process.env.DB_PASSWORD,
    port: process.env.DB_PORT || 5432,
    dialectOptions: {
      ssl: {
        require: true,
        rejectUnauthorized: false
      }
    }
  },
  plugins: [
    '@nocobase/plugin-client',
    '@nocobase/plugin-auth',
    '@nocobase/plugin-users'
  ]
});

console.log("Paso 4: Configurando el servidor HTTP...");
const port = process.env.PORT || 10000;
const server = http.createServer(app.callback());

console.log("Paso 5: Abriendo puerto...");
server.listen(port, async () => {
  console.log(`🚀 Servidor escuchando en puerto ${port}`);
  
  console.log("Paso 6: Forzando instalación limpia...");
  try {
    await app.install({ clean: false });
    console.log("✨ Tablas e infraestructura de NocoBase listas.");
    
    await app.start();
    console.log("✅ ¡NocoBase corriendo perfectamente!");
  } catch (error) {
    console.error("❌ Error en la inicialización:", error);
  }
});
