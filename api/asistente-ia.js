// Archivo: api/asistente-ia.js
export default async function handler(req, res) {
    if (req.method !== 'POST') {
        return res.status(405).json({ error: 'Método no permitido' });
    }

    const { consulta, productos } = req.body;

    if (!consulta || !productos) {
        return res.status(400).json({ error: 'Faltan datos requeridos.' });
    }

    // Preparar resumen estructurado de los productos disponibles
    const resumenProductos = productos.map(p => 
        `- ${p.articulo || p.nombre}: $${p.precio} (CÓD: ${p.codigo || 'S/N'}, Desc: ${p.descripcion || 'Sin descripción'}, Promoción: ${p.promocion || 'Ninguna'})`
    ).join('\n');

    const promptSystem = `
    Eres un asistente virtual de ventas amable y eficiente de un comercio.
    A continuación tienes el inventario disponible de este comercio:

    ${resumenProductos}

    Instrucciones:
    1. Responde a la consulta del cliente basándote ÚNICAMENTE en el inventario proporcionado.
    2. Si el producto solicitado no existe en la lista, indícalo amablemente y sugiere alternativas disponibles.
    3. Responde de forma concisa, breve y clara, optimizada para lectura en dispositivos móviles.
    4. Menciona precios y promociones cuando corresponda.
    `;

    try {
        const response = await fetch('https://api.openai.com/v1/chat/completions', {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${process.env.OPENAI_API_KEY}`,
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                model: 'gpt-[MODELO]',
                messages: [
                    { role: 'system', content: promptSystem },
                    { role: 'user', content: consulta }
                ],
                max_tokens: 250,
                temperature: 0.5
            })
        });

        const data = await response.json();
        const textoRespuesta = data.choices[0].message.content;

        return res.status(200).json({ respuesta: textoRespuesta });

    } catch (error) {
        console.error("Error al conectar con la API de IA:", error);
        return res.status(500).json({ error: "Error en el servidor de IA" });
    }
}
