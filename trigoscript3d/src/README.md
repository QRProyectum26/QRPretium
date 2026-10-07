Markdown


# TrigoScript 3D (Lingua Latina) 📐🌌

**TrigoScript 3D** es un lenguaje de programación de dominio específico (*DSL*) de alto rendimiento, diseñado en **Rust**, para el control angular de actuadores, motores, acústica 3D y simulación cosmo-geométrica.

El lenguaje reemplaza el cálculo trigonométrico continuo tradicional por una **retícula discreta de fases basada en el ángulo de 60° (`TRINUM`)** y rotaciones en la **esfera $S^3$ mediante cuaterniones**, eliminando por completo el problema del *Gimbal Lock* (bloqueo de ejes).

---

## 🌟 Características Principales

* **Léxico Atemporal (42 Términos en Latín):** Un vocabulario cerrado, ortogonal y compositivo.
* **Geometría de Sólidos Platónicos:**
  * **Núcleo Atómico (`TETRAHEDRON`):** 4 caras triangulares equiláteras ($60^\circ$). Geometría estable de concentración de masa.
  * **Electrón (`OCTAHEDRON`):** 8 caras triangulares equiláteras ($60^\circ$). Onda de carga dinámica y movilidad.
  * **Soles y Estrellas (`HEXAHEDRON`):** 6 caras cuadradas. Emisores masivos radiantes.
  * **Lunas y Satélites (`ICOSAHEDRON`):** 20 caras triangulares equiláteras ($60^\circ$). Moduladores fluidos de frecuencia.
  * **Dodecaedro Cósmico (`DODECAHEDRON`):** 12 caras pentagonales. Contenedor de 12 universos en rotación expansiva.
* **Control Angular y Actuadores:** Diseñado para enviar vectores de fase instantáneos a servomotores, maquinaria CNC, drones y procesadores de señal digital (DSP).
* **Motor en Rust:** Compilador nativo, ultra-rápido y seguro en memoria.

---

## 📋 Diccionario del Lenguaje (42 Términos Universales)

### 1. Typi Datarum (Tipos de Datos - 4)
* `Angulus`, `Phasor3D`, `Sphaera`, `Harmonicus`

### 2. Solidis Platonicis et Particulis (Sólidos y Partículas - 7)
* `TETRAHEDRON`, `HEXAHEDRON`, `OCTAHEDRON`, `ICOSAHEDRON`, `DODECAHEDRON`, `ELECTRON`, `NUCLEUS`

### 3. Structurae Controlis (Estructuras de Control - 10)
* `functio`, `incipit`, `finis`, `constans`, `variabilis`, `reddere`, `evaluare`, `harmonicus`, `passus`, `omnis`

### 4. Operatores Trigonometrici (Operadores de Fase - 9)
* `rotatio`, `sphaera_rotatio`, `interferentia`, `inversus`, `projectio`, `octans`, `facies`, `amplitudo`, `frequenz`

### 5. Literales et Constantibus (Constantes - 8)
* `TRINUM` ($\pi/3 = 60^\circ$), `PI`, `TAU`, `GRAD`, `RAD`, `MATRIX`, `TENSOR`, `ORIGIS`

### 6. Axis et Referentiis (Ejes - 4)
* `AXIS_X`, `AXIS_Y`, `AXIS_Z`, `ORIGIS`

---

## 💻 Ejemplo de Código (`.trigo`)

```text
(* Control de Fase de un Actuador usando TrigoScript 3D *)
constans ANGULUS_BASE = TRINUM
variabilis e_particula = ELECTRON(TRINUM)

functio MovereActuatorem(fase_deseada) :
incipit
    variabilis nueva_pos = sphaera_rotatio(e_particula, fase_deseada)
    reddere evaluare(nueva_pos)
finis
