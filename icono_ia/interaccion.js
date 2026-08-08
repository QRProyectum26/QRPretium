// interaccion.js - Control de captura de voz y conexión con la API de IA

document.addEventListener("DOMContentLoaded", () => {
  const iconMic = document.getElementById("icon-mic");
  const statusTitle = document.getElementById("status-text-title");
  const statusSub1 = document.getElementById("status-text-sub1");
  const statusSub2 = document.getElementById("status-text-sub2");
  const dodecaedroContainer = document.getElementById("dodecaedro-container");

  let isListening = false;
  let recognition = null;

  // Inicializar Web Speech API si está disponible en el navegador
  if ("webkitSpeechRecognition" in window || "SpeechRecognition" in window) {
    const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
    recognition = new SpeechRecognition();
    recognition.lang = "es-ES";
    recognition.continuous = false;
    recognition.interimResults = false;

    recognition.onstart = () => {
      isListening = true;
      if (dodecaedroContainer) dodecaedroContainer.classList.add("ia-listening");
      if (statusTitle) statusTitle.textContent = "ESCUCHANDO...";
      if (statusSub1) statusSub1.textContent = "PROCESANDO";
      if (statusSub2) statusSub2.textContent = "ENTRADA";
    };

    recognition.onresult = (event) => {
      const transcript = event.results[0][0].transcript;
      console.log("Comando recibido para qrpretium:", transcript);
      
      // Feedback visual de éxito
      if (statusTitle) statusTitle.textContent = "PROCESADO";
      if (statusSub1) statusSub1.textContent = "ENVIANDO A";
      if (statusSub2) statusSub2.textContent = "CATALOGO.HTML";
      
      // Aquí conectas tu llamada a la API de qrpretium enviando 'transcript'
    };

    recognition.onerror = (event) => {
      console.error("Error en la interacción por voz:", event.error);
      if (statusTitle) statusTitle.textContent = "ERROR";
      if (statusSub1) statusSub1.textContent = "INTENTE";
      if (statusSub2) statusSub2.textContent = "NUEVAMENTE";
      stopListening();
    };

    recognition.onend = () => {
      stopListening();
    };
  }

  function stopListening() {
    isListening = false;
    if (dodecaedroContainer) dodecaedroContainer.classList.remove("ia-listening");
    setTimeout(() => {
      if (statusTitle) statusTitle.textContent = "ESCUCHANDO";
      if (statusSub1) statusSub1.textContent = "MODO VOZ";
      if (statusSub2) statusSub2.textContent = "ACTIVADO";
    }, 2500);
  }

  // Activar captura de voz haciendo clic en el dodecaedro/micrófono
  if (iconMic) {
    iconMic.addEventListener("click", () => {
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
  }
});