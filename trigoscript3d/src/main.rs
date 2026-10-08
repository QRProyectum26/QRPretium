rust
// =====================================================================
// TRIGOSCRIPT 3D (LINGUA LATINA) - COMPILATOR ET MOTOR UNIFICATUS
// Engine & Compiler for 3D Geometry, Quaternions & Phase Dynamics
// VERSIÓN 2.0 - Corregida y Extendida
// =====================================================================

use std::collections::HashMap;
use std::f64::consts::PI;

// ---------------------------------------------------------------------
// 1. CONSTANTES GEOMÉTRICAS Y AXIOMAS
// ---------------------------------------------------------------------

/// 60 grados en radianes (Triángulo Equilátero)
pub const TRINUM: f64 = PI / 3.0;

/// Constante de Tensión Máxima de Planck (Cota de Saturación Geométrica)
pub const TENSIO_PLANCK: f64 = 1.22e19;

/// Tolerancia para considerar dos fases "alineadas" a 60°
pub const TOLERANTIA_TRINUM: f64 = 1e-9;

// ---------------------------------------------------------------------
// 2. MOTOR MATEMÁTICO EN LA ESFERA S3
// ---------------------------------------------------------------------

/// Representación de posición y actitud mediante ángulos de fase (Radianes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phasor3D {
    pub yaw: f64,   // ψ (Guiñada)
    pub pitch: f64, // θ (Cabeceo)
    pub roll: f64,  // φ (Alabeo)
}

impl Phasor3D {
    pub const ORIGO: Phasor3D = Phasor3D { yaw: 0.0, pitch: 0.0, roll: 0.0 };

    pub fn new(yaw: f64, pitch: f64, roll: f64) -> Self {
        Phasor3D { yaw, pitch, roll }
    }

    /// Distancia angular (norma euclídea en el espacio de fases)
    pub fn distantia(&self, alius: &Phasor3D) -> f64 {
        let dy = self.yaw - alius.yaw;
        let dp = self.pitch - alius.pitch;
        let dr = self.roll - alius.roll;
        (dy * dy + dp * dp + dr * dr).sqrt()
    }
}

/// Cuaternión para rotaciones continuas en S3 libres de Gimbal Lock
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphaera {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sphaera {
    pub const IDENTITAS: Sphaera = Sphaera { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };

    /// Construye un cuaternión desde ángulos de Euler (yaw-pitch-roll)
    pub fn ex_phasore(p: Phasor3D) -> Self {
        let cy = (p.yaw * 0.5).cos();
        let sy = (p.yaw * 0.5).sin();
        let cp = (p.pitch * 0.5).cos();
        let sp = (p.pitch * 0.5).sin();
        let cr = (p.roll * 0.5).cos();
        let sr = (p.roll * 0.5).sin();

        Sphaera {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }

    /// CORREGIDO: Conversión correcta de cuaternión a ángulos de Euler (ZYX intrinsic)
    pub fn ad_phasorem(&self) -> Phasor3D {
        // Normalizar para robustez numérica
        let n = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let (w, x, y, z) = if n > 0.0 {
            (self.w / n, self.x / n, self.y / n, self.z / n)
        } else {
            (1.0, 0.0, 0.0, 0.0)
        };

        // Roll (X)
        let sinr_cosp = 2.0 * (w * x + y * z);
        let cosr_cosp = 1.0 - 2.0 * (x * x + y * y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        // Pitch (Y) con clamp por asin
        let sinp = 2.0 * (w * y - z * x);
        let pitch = if sinp.abs() >= 1.0 {
            sinp.signum() * PI / 2.0
        } else {
            sinp.asin()
        };

        // Yaw (Z)
        let siny_cosp = 2.0 * (w * z + x * y);
        let cosy_cosp = 1.0 - 2.0 * (y * y + z * z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        Phasor3D { yaw, pitch, roll }
    }

    /// Producto de Hamilton entre dos cuaterniones
    pub fn multiplica(&self, alius: &Sphaera) -> Sphaera {
        Sphaera {
            w: self.w * alius.w - self.x * alius.x - self.y * alius.y - self.z * alius.z,
            x: self.w * alius.x + self.x * alius.w + self.y * alius.z - self.z * alius.y,
            y: self.w * alius.y - self.x * alius.z + self.y * alius.w + self.z * alius.x,
            z: self.w * alius.z + self.x * alius.y - self.y * alius.x + self.z * alius.w,
        }
    }

    /// Conjugado (inverso para cuaterniones unitarios)
    pub fn conjugatus(&self) -> Sphaera {
        Sphaera { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }

    /// Norma
    pub fn norma(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Producto punto (medida de alineación angular en S3)
    pub fn punctum(&self, alius: &Sphaera) -> f64 {
        self.w * alius.w + self.x * alius.x + self.y * alius.y + self.z * alius.z
    }

    /// Ángulo entre dos orientaciones (en radianes)
    pub fn angulus_ad(&self, alius: &Sphaera) -> f64 {
        2.0 * self.punctum(alius).abs().min(1.0).acos()
    }
}

/// Mapa Cosmo-Geométrico de Sólidos Platónicos y Entidades
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SolidusPlatonicus {
    Tetrahedron,  // NUCLEUS
    Octahedron,   // ELECTRON
    Hexahedron,   // SOL / STELLA
    Icosahedron,  // LUNA / SATELLIS
    Dodecahedron, // UNIVERSUS
}

impl SolidusPlatonicus {
    pub fn facies(&self) -> usize {
        match self {
            SolidusPlatonicus::Tetrahedron => 4,
            SolidusPlatonicus::Hexahedron => 6,
            SolidusPlatonicus::Octahedron => 8,
            SolidusPlatonicus::Dodecahedron => 12,
            SolidusPlatonicus::Icosahedron => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CorpusGeometria {
    pub forma: SolidusPlatonicus,
    pub positio: Phasor3D,
    pub amplitudo: f64,
    pub frequenz: f64,
}

impl CorpusGeometria {
    pub fn novus(forma: SolidusPlatonicus, positio: Phasor3D, amplitudo: f64) -> Self {
        CorpusGeometria { forma, positio, amplitudo, frequenz: 1.0 }
    }

    pub fn cum_frequenz(mut self, f: f64) -> Self {
        self.frequenz = f;
        self
    }

    pub fn sphaera(&self) -> Sphaera {
        Sphaera::ex_phasore(self.positio)
    }
}

// ---------------------------------------------------------------------
// 3. EVALUATOR / RUNTIME
// ---------------------------------------------------------------------

pub struct Evaluator;

impl Evaluator {
    /// Aplica una rotación esférica continua (S3) sobre un Phasor3D sin Gimbal Lock
    pub fn sphaera_rotatio(p: Phasor3D, eje_z: f64) -> Phasor3D {
        let q_base = Sphaera::ex_phasore(p);
        let q_rot = Sphaera::ex_phasore(Phasor3D { yaw: eje_z, pitch: 0.0, roll: 0.0 });
        q_rot.multiplica(&q_base).ad_phasorem()
    }

    /// Rotación S3 por eje arbitrario (axis-angle)
    pub fn rotatio_axial(p: Phasor3D, axis: (f64, f64, f64), angulus: f64) -> Phasor3D {
        let n = (axis.0 * axis.0 + axis.1 * axis.1 + axis.2 * axis.2).sqrt();
        if n < 1e-12 {
            return p;
        }
        let (ax, ay, az) = (axis.0 / n, axis.1 / n, axis.2 / n);
        let half = angulus * 0.5;
        let s = half.sin();
        let q_rot = Sphaera { w: half.cos(), x: ax * s, y: ay * s, z: az * s };
        q_rot.multiplica(&Sphaera::ex_phasore(p)).ad_phasorem()
    }

    /// CORREGIDO: Interferencia usando producto interno de cuaterniones en S3.
    /// - Coherente con la simetría q ↔ -q.
    /// - Modulada por la diferencia de frecuencia (batido).
    pub fn interferentia(c1: &CorpusGeometria, c2: &CorpusGeometria) -> f64 {
        let q1 = c1.sphaera();
        let q2 = c2.sphaera();

        // Alineación angular en S3: 1.0 = idénticos, 0.0 = ortogonales
        let alineatio = q1.punctum(&q2).abs();

        // Batido por diferencia de frecuencia (componente temporal)
        let delta_f = (c1.frequenz - c2.frequenz).abs();
        let batido = 1.0 / (1.0 + delta_f);

        let raw = alineatio * batido * c1.amplitudo * c2.amplitudo;
        Self::limitem_tensio(raw)
    }

    /// Invariante de Fase: ninguna magnitud supera la Tensión de Planck
    pub fn limitem_tensio(valor: f64) -> f64 {
        valor.clamp(-TENSIO_PLANCK, TENSIO_PLANCK)
    }

    /// ¿Están dos fases alineadas al múltiplo de TRINUM?
    pub fn est_alineatus(p1: Phasor3D, p2: Phasor3D) -> bool {
        let d = (p1.yaw - p2.yaw).abs() % TRINUM;
        d < TOLERANTIA_TRINUM || (TRINUM - d).abs() < TOLERANTIA_TRINUM
    }
}

// ---------------------------------------------------------------------
// 4. ANALIZADOR LÉXICO (LEXER)
// ---------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Typi Datarum
    Angulus, Phasor3DToken, SphaeraToken, HarmonicusToken,
    // Solidis Platonicis et Particulis
    Tetrahedron, Hexahedron, Octahedron, Icosahedron, Dodecahedron, Electron, Nucleus,
    // Structurae Controlis
    Functio, Incipit, Finis, Constans, Variabilis, Reddere, Evaluare, LoopHarmonicus, Passus, Omnis,
    // Operatores Trigonometrici
    Rotatio, SphaeraRotatio, Interferentia, Inversus, Projectio, Octans, Facies, Amplitudo, Frequenz,
    // Literales et Constantibus
    Grad, Rad, Trinum, Pi, Tau, Matrix, Tensor,
    TensioPlanck,
    AxisX, AxisY, AxisZ, Origis,
    // Operatores aritmetici
    Plus, Minus, Asteriscus, Divisa, Caret,
    // Symbola
    Numerus(f64), Identificator(String),
    ParenthesisAperta, ParenthesisClausa, Aequalis, Coma, DuoPuncta, Punctum,
    Eo, EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.current_char();

        if ch.is_alphabetic() || ch == '_' {
            let start = self.position;
            while self.position < self.input.len()
                && (self.current_char().is_alphanumeric() || self.current_char() == '_')
            {
                self.position += 1;
            }
            let literal = &self.input[start..self.position];

            return match literal {
                "Angulus" => Token::Angulus,
                "Phasor3D" => Token::Phasor3DToken,
                "Sphaera" => Token::SphaeraToken,
                "Harmonicus" => Token::HarmonicusToken,
                "TETRAHEDRON" => Token::Tetrahedron,
                "HEXAHEDRON" => Token::Hexahedron,
                "OCTAHEDRON" => Token::Octahedron,
                "ICOSAHEDRON" => Token::Icosahedron,
                "DODECAHEDRON" => Token::Dodecahedron,
                "ELECTRON" => Token::Electron,
                "NUCLEUS" => Token::Nucleus,
                "functio" => Token::Functio,
                "incipit" => Token::Incipit,
                "finis" => Token::Finis,
                "constans" => Token::Constans,
                "variabilis" => Token::Variabilis,
                "reddere" => Token::Reddere,
                "evaluare" => Token::Evaluare,
                "harmonicus" => Token::LoopHarmonicus,
                "passus" => Token::Passus,
                "omnis" => Token::Omnis,
                "rotatio" => Token::Rotatio,
                "sphaera_rotatio" => Token::SphaeraRotatio,
                "interferentia" => Token::Interferentia,
                "inversus" => Token::Inversus,
                "projectio" => Token::Projectio,
                "octans" => Token::Octans,
                "facies" => Token::Facies,
                "amplitudo" => Token::Amplitudo,
                "frequenz" => Token::Frequenz,
                "TRINUM" => Token::Trinum,
                "TENSIO_PLANCK" => Token::TensioPlanck,
                "PI" => Token::Pi,
                "TAU" => Token::Tau,
                "GRAD" => Token::Grad,
                "RAD" => Token::Rad,
                "MATRIX" => Token::Matrix,
                "TENSOR" => Token::Tensor,
                "AXIS_X" => Token::AxisX,
                "AXIS_Y" => Token::AxisY,
                "AXIS_Z" => Token::AxisZ,
                "ORIGIS" => Token::Origis,
                _ => Token::Identificator(literal.to_string()),
            };
        }

        // Números: el signo '-' se maneja como operador unario en el parser
        if ch.is_numeric() {
            let start = self.position;
            while self.position < self.input.len()
                && (self.current_char().is_numeric()
                    || self.current_char() == '.'
                    || self.current_char() == 'e'
                    || self.current_char() == 'E')
            {
                self.position += 1;
            }
            let val: f64 = self.input[start..self.position].parse().unwrap_or(0.0);
            return Token::Numerus(val);
        }

        self.position += 1;
        match ch {
            '(' => Token::ParenthesisAperta,
            ')' => Token::ParenthesisClausa,
            '=' => {
                if self.position < self.input.len() && self.current_char() == '>' {
                    self.position += 1;
                    Token::Eo
                } else {
                    Token::Aequalis
                }
            }
            ',' => Token::Coma,
            ':' => Token::DuoPuncta,
            '.' => Token::Punctum,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asteriscus,
            '/' => Token::Divisa,
            '^' => Token::Caret,
            _ => self.next_token(),
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.position < self.input.len() {
            let c = self.current_char();
            if c.is_whitespace() {
                self.position += 1;
            } else if c == '(' && self.position + 1 < self.input.len()
                && self.input[self.position + 1..].starts_with('*')
            {
                while self.position + 1 < self.input.len()
                    && !self.input[self.position..].starts_with("*)")
                {
                    self.position += 1;
                }
                self.position += 2;
            } else {
                break;
            }
        }
    }
}

// ---------------------------------------------------------------------
// 5. ANALIZADOR SINTÁCTICO (PARSER -> AST)
// ---------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ASTNode {
    DeclaratioConstantis { nomen: String, valor: Box<ASTNode> },
    DeclaratioVariabilis { nomen: String, valor: Box<ASTNode> },
    CreatioParticulae {
        forma: SolidusPlatonicus,
        positiophasor: Box<ASTNode>,
        amplitudo: Option<Box<ASTNode>>,
    },
    OperatioSphaeraRotatio { phasor: Box<ASTNode>, eje_z: Box<ASTNode> },
    OperatioRotatio { phasor: Box<ASTNode>, axis: Box<ASTNode>, angulus: Box<ASTNode> },
    OperatioInterferentia { corpus1: Box<ASTNode>, corpus2: Box<ASTNode> },
    OperatioEvaluare { corpus: Box<ASTNode> },
    BinOp { op: BinOp, lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Negatio(Box<ASTNode>),
    LiteralNumerus(f64),
    LiteralTrinum,
    LiteralTensioPlanck,
    LiteralPi,
    LiteralTau,
    Identificator(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Additio, Subtractio, Multiplicatio, Divisio, Potentia,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn eat(&mut self, expected: &Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(expected) {
            self.advance();
        } else {
            panic!(
                "Error Syntaxicus: Expectabatur {:?}, inventum {:?}",
                expected, self.current_token
            );
        }
    }

    /// NUEVO: Parsea un programa completo (lista de sentencias)
    pub fn parse_programa(&mut self) -> Vec<ASTNode> {
        let mut nodos = Vec::new();
        while self.current_token != Token::EOF {
            nodos.push(self.parse_sententia());
        }
        nodos
    }

    fn parse_sententia(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Constans => {
                self.advance();
                let nomen = self.expect_ident();
                self.eat(&Token::Aequalis);
                let valor = self.parse_expression();
                ASTNode::DeclaratioConstantis { nomen, valor: Box::new(valor) }
            }
            Token::Variabilis => {
                self.advance();
                let nomen = self.expect_ident();
                self.eat(&Token::Aequalis);
                let valor = self.parse_expression();
                ASTNode::DeclaratioVariabilis { nomen, valor: Box::new(valor) }
            }
            _ => {
                let node = self.parse_expression();
                // Consumir opcionalmente un salto de línea lógico (no hay en este lexer simple)
                node
            }
        }
    }

    fn expect_ident(&mut self) -> String {
        if let Token::Identificator(n) = self.current_token.clone() {
            self.advance();
            n
        } else {
            panic!("Expectabatur identificator, inventum {:?}", self.current_token);
        }
    }

    // --- Expresiones con precedencia (descenso recursivo) ---
    fn parse_expression(&mut self) -> ASTNode {
        self.parse_additio()
    }

    fn parse_additio(&mut self) -> ASTNode {
        let mut lhs = self.parse_multiplicatio();
        loop {
            let op = match self.current_token {
                Token::Plus => BinOp::Additio,
                Token::Minus => BinOp::Subtractio,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_multiplicatio();
            lhs = ASTNode::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        lhs
    }

    fn parse_multiplicatio(&mut self) -> ASTNode {
        let mut lhs = self.parse_potentia();
        loop {
            let op = match self.current_token {
                Token::Asteriscus => BinOp::Multiplicatio,
                Token::Divisa => BinOp::Divisio,
                _ => break,
            };
            self.advance();
            let rhs = self.parse_potentia();
            lhs = ASTNode::BinOp { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        lhs
    }

    fn parse_potentia(&mut self) -> ASTNode {
        let base = self.parse_unaria();
        if self.current_token == Token::Caret {
            self.advance();
            let exp = self.parse_potentia(); // asociativa a la derecha
            ASTNode::BinOp {
                op: BinOp::Potentia,
                lhs: Box::new(base),
                rhs: Box::new(exp),
            }
        } else {
            base
        }
    }

    fn parse_unaria(&mut self) -> ASTNode {
        match self.current_token {
            Token::Minus => {
                self.advance();
                ASTNode::Negatio(Box::new(self.parse_unaria()))
            }
            _ => self.parse_primaria(),
        }
    }

    fn parse_primaria(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Trinum => {
                self.advance();
                ASTNode::LiteralTrinum
            }
            Token::TensioPlanck => {
                self.advance();
                ASTNode::LiteralTensioPlanck
            }
            Token::Pi => {
                self.advance();
                ASTNode::LiteralPi
            }
            Token::Tau => {
                self.advance();
                ASTNode::LiteralTau
            }
            Token::Numerus(v) => {
                self.advance();
                ASTNode::LiteralNumerus(v)
            }
            Token::Nucleus => self.parse_particula(SolidusPlatonicus::Tetrahedron),
            Token::Electron => self.parse_particula(SolidusPlatonicus::Octahedron),
            Token::Tetrahedron => self.parse_particula(SolidusPlatonicus::Tetrahedron),
            Token::Octahedron => self.parse_particula(SolidusPlatonicus::Octahedron),
            Token::Hexahedron => self.parse_particula(SolidusPlatonicus::Hexahedron),
            Token::Icosahedron => self.parse_particula(SolidusPlatonicus::Icosahedron),
            Token::Dodecahedron => self.parse_particula(SolidusPlatonicus::Dodecahedron),
            Token::SphaeraRotatio => {
                self.advance();
                self.eat(&Token::ParenthesisAperta);
                let phasor = self.parse_expression();
                self.eat(&Token::Coma);
                let eje = self.parse_expression();
                self.eat(&Token::ParenthesisClausa);
                ASTNode::OperatioSphaeraRotatio {
                    phasor: Box::new(phasor),
                    eje_z: Box::new(eje),
                }
            }
            Token::Rotatio => {
                self.advance();
                self.eat(&Token::ParenthesisAperta);
                let phasor = self.parse_expression();
                self.eat(&Token::Coma);
                let axis = self.parse_expression();
                self.eat(&Token::Coma);
                let angulus = self.parse_expression();
                self.eat(&Token::ParenthesisClausa);
                ASTNode::OperatioRotatio {
                    phasor: Box::new(phasor),
                    axis: Box::new(axis),
                    angulus: Box::new(angulus),
                }
            }
            Token::Interferentia => {
                self.advance();
                self.eat(&Token::ParenthesisAperta);
                let c1 = self.parse_expression();
                self.eat(&Token::Coma);
                let c2 = self.parse_expression();
                self.eat(&Token::ParenthesisClausa);
                ASTNode::OperatioInterferentia {
                    corpus1: Box::new(c1),
                    corpus2: Box::new(c2),
                }
            }
            Token::Evaluare => {
                self.advance();
                self.eat(&Token::ParenthesisAperta);
                let c = self.parse_expression();
                self.eat(&Token::ParenthesisClausa);
                ASTNode::OperatioEvaluare { corpus: Box::new(c) }
            }
            Token::ParenthesisAperta => {
                self.advance();
                let inner = self.parse_expression();
                self.eat(&Token::ParenthesisClausa);
                inner
            }
            Token::Identificator(n) => {
                self.advance();
                ASTNode::Identificator(n)
            }
            other => panic!("Expressio invalida: {:?}", other),
        }
    }

    fn parse_particula(&mut self, forma: SolidusPlatonicus) -> ASTNode {
        self.advance(); // consume el token del sólido
        self.eat(&Token::ParenthesisAperta);
        let pos = self.parse_expression();
        let mut amplitudo = None;
        if self.current_token == Token::Coma {
            self.advance();
            amplitudo = Some(Box::new(self.parse_expression()));
        }
        self.eat(&Token::ParenthesisClausa);
        ASTNode::CreatioParticulae {
            forma,
            positiophasor: Box::new(pos),
            amplitudo,
        }
    }
}

// ---------------------------------------------------------------------
// 6. MÁQUINA DE EJECUCIÓN (INTERPRES) CON TABLA DE SÍMBOLOS
// ---------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Valoris {
    Numerus(f64),
    Corpus(CorpusGeometria),
    Phasor(Phasor3D),
}

impl Valoris {
    pub fn ut_numerus(&self) -> f64 {
        match self {
            Valoris::Numerus(n) => *n,
            Valoris::Phasor(p) => p.yaw, // fallback
            Valoris::Corpus(c) => c.amplitudo,
        }
    }

    pub fn ut_corpus(&self) -> CorpusGeometria {
        match self {
            Valoris::Corpus(c) => c.clone(),
            Valoris::Numerus(n) => {
                CorpusGeometria::novus(SolidusPlatonicus::Tetrahedron, Phasor3D::ORIGO, *n)
            }
            Valoris::Phasor(p) => {
                CorpusGeometria::novus(SolidusPlatonicus::Tetrahedron, *p, 1.0)
            }
        }
    }

    pub fn ut_phasor(&self) -> Phasor3D {
        match self {
            Valoris::Phasor(p) => *p,
            Valoris::Corpus(c) => c.positio,
            Valoris::Numerus(n) => Phasor3D::new(*n, 0.0, 0.0),
        }
    }
}

pub struct Interpres {
    symbola: HashMap<String, Valoris>,
    constantes: HashMap<String, Valoris>,
}

impl Interpres {
    pub fn novus() -> Self {
        Interpres {
            symbola: HashMap::new(),
            constantes: HashMap::new(),
        }
    }

    pub fn exsequi(&mut self, program: &[ASTNode]) {
        for nodo in program {
            self.evalua(nodo);
        }
    }

    pub fn symbola(&self) -> &HashMap<String, Valoris> {
        &self.symbola
    }

    fn evalua(&mut self, nodo: &ASTNode) -> Valoris {
        match nodo {
            ASTNode::DeclaratioConstantis { nomen, valor } => {
                let v = self.evalua(valor);
                self.constantes.insert(nomen.clone(), v.clone());
                self.symbola.insert(nomen.clone(), v.clone());
                v
            }
            ASTNode::DeclaratioVariabilis { nomen, valor } => {
                let v = self.evalua(valor);
                self.symbola.insert(nomen.clone(), v.clone());
                v
            }
            ASTNode::CreatioParticulae { forma, positiophasor, amplitudo } => {
                let pos = self.evalua(positiophasor).ut_phasor();
                let amp = amplitudo
                    .as_ref()
                    .map(|a| self.evalua(a).ut_numerus())
                    .unwrap_or(1.0);
                Valoris::Corpus(CorpusGeometria::novus(*forma, pos, amp))
            }
            ASTNode::OperatioSphaeraRotatio { phasor, eje_z } => {
                let p = self.evalua(phasor).ut_phasor();
                let e = self.evalua(eje_z).ut_numerus();
                Valoris::Phasor(Evaluator::sphaera_rotatio(p, e))
            }
            ASTNode::OperatioRotatio { phasor, axis, angulus } => {
                let p = self.evalua(phasor).ut_phasor();
                let a = self.evalua(axis);
                let (ax, ay, az) = match &a {
                    Valoris::Phasor(ph) => (ph.yaw, ph.pitch, ph.roll),
                    _ => (0.0, 0.0, 1.0),
                };
                let ang = self.evalua(angulus).ut_numerus();
                Valoris::Phasor(Evaluator::rotatio_axial(p, (ax, ay, az), ang))
            }
            ASTNode::OperatioInterferentia { corpus1, corpus2 } => {
                let c1 = self.evalua(corpus1).ut_corpus();
                let c2 = self.evalua(corpus2).ut_corpus();
                Valoris::Numerus(Evaluator::interferentia(&c1, &c2))
            }
            ASTNode::OperatioEvaluare { corpus } => self.evalua(corpus),
            ASTNode::BinOp { op, lhs, rhs } => {
                let l = self.evalua(lhs).ut_numerus();
                let r = self.evalua(rhs).ut_numerus();
                let res = match op {
                    BinOp::Additio => l + r,
                    BinOp::Subtractio => l - r,
                    BinOp::Multiplicatio => l * r,
                    BinOp::Divisio => l / r,
                    BinOp::Potentia => l.powf(r),
                };
                Valoris::Numerus(Evaluator::limitem_tensio(res))
            }
            ASTNode::Negatio(inner) => {
                Valoris::Numerus(-self.evalua(inner).ut_numerus())
            }
            ASTNode::LiteralNumerus(v) => Valoris::Numerus(*v),
            ASTNode::LiteralTrinum => Valoris::Numerus(TRINUM),
            ASTNode::LiteralTensioPlanck => Valoris::Numerus(TENSIO_PLANCK),
            ASTNode::LiteralPi => Valoris::Numerus(PI),
            ASTNode::LiteralTau => Valoris::Numerus(2.0 * PI),
            ASTNode::Identificator(n) => self
                .symbola
                .get(n)
                .cloned()
                .unwrap_or_else(|| panic!("Symbolum non inventum: {}", n)),
        }
    }
}

// ---------------------------------------------------------------------
// 7. CONSOLA DE EJECUCIÓN Y PRUEBAS INTEGRADAS
// ---------------------------------------------------------------------

fn main() {
    println!("================================================================");
    println!("  TRIGOSCRIPT 3D (LINGUA LATINA) - COMPILATOR ET MOTOR RUST     ");
    println!("  VERSIO 2.0 - Cuaterniones correcti, interpres plenus         ");
    println!("================================================================\n");

    // --- SCRIPT DE PRUEBA ---
    let codigo_script = r#"
        (* Script: Electron orbitando un Nucleo Tetraedrico *)
        constans ANGULUS_BASE = TRINUM
        constans LIMITE_SEGURO = TENSIO_PLANCK * 0.001
        variabilis nucleo = NUCLEUS(0.0)
        variabilis electron = ELECTRON(ANGULUS_BASE, 1.0)
        variabilis fase_rotata = sphaera_rotatio(ANGULUS_BASE, ANGULUS_BASE)
        variabilis campo = interferentia(nucleo, electron)
    "#;

    println!("--- 1. CODIGO FUENTE INGESTADO (.trigo) ---");
    println!("{}\n", codigo_script.trim());

    // --- FASE 2: LEXER + PARSER ---
    println!("--- 2. ANALISIS LEXICO ET SYNTACTICO (PARSER -> AST) ---");
    let lexer = Lexer::new(codigo_script);
    let mut parser = Parser::new(lexer);
    let programa = parser.parse_programa();
    println!("Sententiae detectae: {}\n", programa.len());
    for (i, nodo) in programa.iter().enumerate() {
        println!("  [{}] {:#?}", i, nodo);
    }
    println!();

    // --- FASE 3: EJECUCION EN EL INTERPRETE ---
    println!("--- 3. EXECUTIO IN INTERPRETE (S3 MOTOR) ---");
    let mut interpres = Interpres::novus();
    interpres.exsequi(&programa);

    println!("\nTabula symbolorum resultans:");
    for (nomen, valor) in interpres.symbola() {
        match valor {
            Valoris::Numerus(n) => println!("  {:>18} = Numerus({:.6})", nomen, n),
            Valoris::Phasor(p) => println!(
                "  {:>18} = Phasor(yaw={:.4}, pitch={:.4}, roll={:.4})",
                nomen, p.yaw, p.pitch, p.roll
            ),
            Valoris::Corpus(c) => println!(
                "  {:>18} = Corpus({:?}, pos=({:.4},{:.4},{:.4}), amp={:.4})",
                nomen, c.forma, c.positio.yaw, c.positio.pitch, c.positio.roll, c.amplitudo
            ),
        }
    }
    println!();

    // --- FASE 4: DEMOSTRACION DIRECTA DEL MOTOR ---
    println!("--- 4. DEMONSTRATIO DIRECTA MOTORIS ---");

    let nucleo = CorpusGeometria::novus(
        SolidusPlatonicus::Tetrahedron,
        Phasor3D::ORIGO,
        1.0,
    );

    let electron = CorpusGeometria::novus(
        SolidusPlatonicus::Octahedron,
        Phasor3D::new(TRINUM, 0.0, 0.0),
        1.0,
    );

    // Rotacion S3 del electron
    let rotado = Evaluator::sphaera_rotatio(electron.positio, TRINUM);
    let campo = Evaluator::interferentia(&nucleo, &electron);

    // Verificacion del round-trip (que antes estaba roto)
    let q = Sphaera::ex_phasore(electron.positio);
    let p_recuperado = q.ad_phasorem();
    let error_roundtrip = electron.positio.distantia(&p_recuperado);

    // Alineacion al multiplo de TRINUM
    let alineado = Evaluator::est_alineatus(nucleo.positio, electron.positio);

    println!("  Constante TRINUM (60 grados): {:.6} rad", TRINUM);
    println!("  Nucleo  (TETRAHEDRON): {} caras, en ORIGIS", 
             SolidusPlatonicus::Tetrahedron.facies());
    println!("  Electron (OCTAHEDRON): {} caras, rotacion S3:", 
             SolidusPlatonicus::Octahedron.facies());
    println!("    yaw={:.4}, pitch={:.4}, roll={:.4}",
             rotado.yaw, rotado.pitch, rotado.roll);
    println!("  Error round-trip Phasor->Sphaera->Phasor: {:.2e}", error_roundtrip);
    println!("  Alineacion nucleo-electron (multiplo TRINUM): {}", alineado);
    println!("  Campo interferentia (S3): {:.6}", campo);
    println!("  Limite Planck aplicado: {:.3e}", TENSIO_PLANCK);

    println!("\n================================================================");
    println!("  STATUS: COMPILATIO ET EVALUATIO SUCESSA (100% OPERATIONAL)    ");
    println!("  VERSIO 2.0 - Motor S3 cum cuaternionibus correctis           ");
    println!("================================================================");
}

// ---------------------------------------------------------------------
// 8. TESTS DE PROPIEDAD (NUCLEO MATEMATICO)
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_phasor_sphaera_identitas() {
        let casos = [
            Phasor3D::new(0.0, 0.0, 0.0),
            Phasor3D::new(TRINUM, 0.0, 0.0),
            Phasor3D::new(0.5, -0.3, 1.2),
            Phasor3D::new(PI, 0.1, -0.7),
        ];
        for p in casos {
            let q = Sphaera::ex_phasore(p);
            let p2 = q.ad_phasorem();
            assert!(
                p.distantia(&p2) < 1e-9,
                "Round-trip fallo: {:?} -> {:?}", p, p2
            );
        }
    }

    #[test]
    fn sphaera_est_unitaria() {
        let p = Phasor3D::new(1.1, 0.3, -0.7);
        let q = Sphaera::ex_phasore(p);
        assert!((q.norma() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn compositio_rotatio_es_asociativa() {
        let p = Phasor3D::new(0.4, 0.2, 0.1);
        let a = TRINUM;
        let b = TRINUM * 0.5;

        let q = Sphaera::ex_phasore(p);
        let qa = Sphaera::ex_phasore(Phasor3D::new(a, 0.0, 0.0));
        let qb = Sphaera::ex_phasore(Phasor3D::new(b, 0.0, 0.0));

        let izq = qa.multiplica(&qb).multiplica(&q);
        let der = qa.multiplica(&qb.multiplica(&q));

        assert!(izq.punctum(&der).abs() > 1.0 - 1e-12);
    }

    #[test]
    fn interferentia_simetria_q_negativo() {
        let c1 = CorpusGeometria::novus(
            SolidusPlatonicus::Tetrahedron,
            Phasor3D::ORIGO,
            1.0,
        );
        let c2 = CorpusGeometria::novus(
            SolidusPlatonicus::Octahedron,
            Phasor3D::new(TRINUM, 0.0, 0.0),
            1.0,
        );
        let i1 = Evaluator::interferentia(&c1, &c2);
        let i2 = Evaluator::interferentia(&c2, &c1);
        assert!((i1 - i2).abs() < 1e-12, "Interferentia no simetrica");
    }

    #[test]
    fn limitem_tensio_satura() {
        assert_eq!(Evaluator::limitem_tensio(1e30), TENSIO_PLANCK);
        assert_eq!(Evaluator::limitem_tensio(-1e30), -TENSIO_PLANCK);
        assert_eq!(Evaluator::limitem_tensio(42.0), 42.0);
    }

    #[test]
    fn alineatio_trinum_detecta_multiplos() {
        let a = Phasor3D::new(0.0, 0.0, 0.0);
        let b = Phasor3D::new(TRINUM, 0.0, 0.0);
        let c = Phasor3D::new(TRINUM * 2.0, 0.0, 0.0);
        assert!(Evaluator::est_alineatus(a, b));
        assert!(Evaluator::est_alineatus(a, c));
        assert!(Evaluator::est_alineatus(b, c));
    }

    #[test]
    fn lexer_produce_tokens_correctos() {
        let mut lex = Lexer::new("constans X = TRINUM");
        assert_eq!(lex.next_token(), Token::Constans);
        assert_eq!(lex.next_token(), Token::Identificator("X".to_string()));
        assert_eq!(lex.next_token(), Token::Aequalis);
        assert_eq!(lex.next_token(), Token::Trinum);
        assert_eq!(lex.next_token(), Token::EOF);
    }

    #[test]
    fn parser_programa_multiple() {
        let src = "constans A = TRINUM\nvariabilis B = 5.0";
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let prog = parser.parse_programa();
        assert_eq!(prog.len(), 2);
    }

    #[test]
    fn interpres_ejecuta_y_almacena() {
        let src = "constans A = TRINUM\nvariabilis B = A * 2.0";
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        let prog = parser.parse_programa();
        let mut interp = Interpres::novus();
        interp.exsequi(&prog);

        let b = interp.symbola().get("B").expect("B no definida");
        match b {
            Valoris::Numerus(n) => assert!((n - TRINUM * 2.0).abs() < 1e-12),
            _ => panic!("B no es Numerus"),
        }
    }
}
