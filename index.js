const { Application } = require('@nocobase/server');

async function startServer() {
  console.log("Iniciando la configuración de NocoBase...");

  // Creamos la aplicación apuntando a tu base de datos PostgreSQL de Neon
  const app = new Application({
    database: {
      dialect: 'postgres',
      url: process.env.DATABASE_URL,
      dialectOptions: {
        ssl: {
          rejectUnauthorized: false // Requerido para conexiones seguras con Neon
        }
      }
    }
  });

  try {
    // Instalamos NocoBase (crea las tablas iniciales de administración)
    console.log("Instalando tablas del sistema en Neon.tech...");
    await app.install({
      clean: false, // Evita borrar datos si el servidor se reinicia
    });

    // Encendemos el servidor en el puerto que nos da Render
    const port = process.env.PORT || 10000;
    await app.listen(port);
    console.log(`¡NocoBase está vivo y corriendo en el puerto ${port}! 🚀`);
  } catch (error) {
    console.error("Error al iniciar NocoBase:", error);
    process.exit(1);
  }
}

startServer();
