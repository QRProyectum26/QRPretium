console.log("Paso 1: Cargando dependencias...");
const { Application } = require('@nocobase/server');
const http = require('http');

console.log("Paso 2: Leyendo variable de base de datos...");
console.log("¿Existe DATABASE_URL?:", process.env.DATABASE_URL ? "Sí" : "No, está vacía");

console.log("Paso 3: Creando la aplicación NocoBase con plugins básicos...");
const app = new Application({
  database: {
    dialect: 'postgres',
    url: process.env.DATABASE_URL,
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

console.log("Paso 4: Configurando el servidor...");
const port = process.env.PORT || 10000;
const server = http.createServer(app.callback());

console.log("Paso 5: Abriendo el puerto para Render...");
server.listen(port, async () => {
  console.log(`🚀 Puerto ${port} abierto exitosamente para Render`);
  
  console.log("Paso 6: Inicializando e instalando NocoBase...");
  try {
    console.log("⏳ Ejecutando app.install()...");
    await app.install({
      clean: false,
    });
    console.log("✨ Instalación de módulos completada.");

    await app.start();
    console.log("✅ NocoBase se ha iniciado y conectado correctamente con la interfaz activa.");
  } catch (error) {
    console.error("❌ Error al iniciar NocoBase:", error);
  }
});
