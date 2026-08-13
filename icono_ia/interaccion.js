// interaccion.js - Inyecta el SVG del dodecaedro en el DOM, controla
// la captura de voz, y anima el "giro" entre las 6 caras/estados del
// asistente: mic (escuchando) -> brain (pensando) -> search (buscando
// detalle, opcional) -> voz (respondiendo) -> check/error (resultado)
// -> vuelta a mic.

document.addEventListener("DOMContentLoaded", () => {
  const wrapper = document.getElementById("dodecaedroWrapper");
  if (!wrapper) {
    console.error("No se encontró #dodecaedroWrapper en el HTML.");
    return;
  }
  // Inyectamos el SVG como parte real del DOM (no <object>), para que
  // getElementById y animaciones.css puedan alcanzar sus elementos.
  // El parámetro ?v=Date.now() evita que el navegador sirva una copia
  // vieja cacheada del SVG mientras seguimos ajustando el diseño.
  fetch("icono_ia/icono_ia.svg?v=" + Date.now())
    .then((res) => {
      if (!res.ok) throw new Error("No se pudo cargar icono_ia.svg (status " + res.status + ")");
      return res.text();
    })
    .then((svgText) => {
      wrapper.innerHTML = svgText;
      inicializarDodecaedroIA();
    })
    .catch((err) => {
      console.error("Error al inyectar el ícono del dodecaedro:", err);
    });
});

function inicializarDodecaedroIA() {
  const iconMic = document.getElementById("icon-mic");
  const dodecaedroContainer = document.getElementById("dodecaedro-container");
  const flipGroup = document.getElementById("dodecaedro-flip");

  const ESTADOS = ["mic", "brain", "check", "error", "voz", "search"];

  /* --- Sistema de caras: gira el dodecaedro y cambia el estado activo --- */
  function mostrarCara(nombreEstado) {
    if (!ESTADOS.includes(nombreEstado)) {
      console.warn("Estado de dodecaedro desconocido:", nombreEstado);
      return;
    }
    if (!flipGroup) {
      // Si por algo no está el grupo de flip, al menos cambiamos la cara sin animar
      cambiarCaraActiva(nombreEstado);
      return;
    }
    flipGroup.classList.add("flip-out");
    // A mitad del giro (cuando está "de canto", casi invisible en X) cambiamos
    // qué cara está activa, y después dejamos que vuelva a expandirse.
    setTimeout(() => {
      cambiarCaraActiva(nombreEstado);
      flipGroup.classList.remove("flip-out");
    }, 280); // debe coincidir con la duración del transition en animaciones.css
  }

  function cambiarCaraActiva(nombreEstado) {
    ESTADOS.forEach((estado) => {
      const grupo = document.getElementById("state-" + estado);
      if (grupo) grupo.classList.toggle("active", estado === nombreEstado);
    });
  }

  // Exponemos la función globalmente para que catalogo.html (el flujo real
  // de la consulta a la IA / texto-a-voz) pueda ir cambiando la cara en
  // los momentos exactos: mostrarCaraDodecaedro('brain'), ('voz'), etc.
  window.mostrarCaraDodecaedro = mostrarCara;

  // Arrancamos mostrando la cara del micrófono (estado de reposo)
  mostrarCara("mic");

  /* --- Reconocimiento de voz --- */
  let isListening = false;
  let recognition = null;

  if ("webkitSpeechRecognition" in window || "SpeechRecognition" in window) {
    const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
    recognition = new SpeechRecognition();
    recognition.lang = "es-ES";
    recognition.continuous = false;
    recognition.interimResults = false;

    recognition.onstart = () => {
      isListening = true;
      if (dodecaedroContainer) dodecaedroContainer.classList.add("ia-listening");
      mostrarCara("mic");
    };

    recognition.onresult = (event) => {
      const transcript = event.results[0][0].transcript;
      console.log("Comando recibido para qrpretium:", transcript);

      if (dodecaedroContainer) dodecaedroContainer.classList.remove("ia-listening");

      // ¿Es una consulta pidiendo más detalle del artículo (en vez de precio)?
      const pideDetalle = /detalle|descripci[oó]n|contame m[aá]s|informaci[oó]n/i.test(transcript);

      if (typeof window.agregarMensajeChat === "function") {
        window.agregarMensajeChat("usuario", transcript);
      }

      if (pideDetalle) {
        mostrarCara("search");
        setTimeout(() => mostrarCara("brain"), 900);
      } else {
        mostrarCara("brain");
      }

      // Disparamos la consulta real (definida en catalogo.html). Esa función
      // ya se encarga de llamar a la IA y reproducir la respuesta por voz.
      if (typeof window.procesarConsultaIa === "function") {
        window.procesarConsultaIa(transcript);
      } else {
        console.warn("window.procesarConsultaIa no está definida — no se pudo consultar la IA.");
      }
    };

    recognition.onerror = (event) => {
      console.error("Error en la interacción por voz:", event.error);
      if (dodecaedroContainer) dodecaedroContainer.classList.remove("ia-listening");
      mostrarCara("error");
      setTimeout(() => mostrarCara("mic"), 2500);
    };

    recognition.onend = () => {
      isListening = false;
      if (dodecaedroContainer) dodecaedroContainer.classList.remove("ia-listening");
    };
  }

  if (iconMic) {
    iconMic.addEventListener("click", (e) => {
      e.stopPropagation(); // evita doble disparo si el botón padre también tiene onclick
      if (!recognition) {
        alert("El reconocimiento de voz no está soportado en este navegador.");
        return;
      }
      if (!isListening) {
        recognition.start();
      } else {
        recognition.stop();
      }
    });
  } else {
    console.warn("No se encontró #icon-mic dentro del SVG inyectado.");
  }
}
