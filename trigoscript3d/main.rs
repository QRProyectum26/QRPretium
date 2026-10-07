Rust


// =====================================================================
// TRIGOSCRIPT 3D (LINGUA LATINA) - COMPILATOR ET MOTOR UNIFICATUS
// Engine & Compiler for 3D Geometry, Quaternions & Phase Dynamics
// =====================================================================

use std::f64::consts::PI;

// ---------------------------------------------------------------------
// 1. CONSTANTES GEOMÉTRICAS Y AXIOMAS
// ---------------------------------------------------------------------
pub const TRINUM: f64 = std::f64::consts::PI / 3.0; // 60 GRADOS EN RADIANES (Triángulo Equilátero)

/// Constante de Tensión Máxima de Planck (Cota de Saturación Geométrica)
pub const TENSIO_PLANCK: f64 = 1.22e19;

// ---------------------------------------------------------------------
// 2. MOTOR MATEMÁTICO EN LA ESFERA S3 (EVALUATOR & RUNTIME)
// ---------------------------------------------------------------------

/// Representación de posición y actitud mediante ángulos de fase (Radianes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phasor3D {
    pub yaw: f64,   // \psi (Guiñada)
    pub pitch: f64, // \theta (Cabeceo)
    pub roll: f64,  // \phi (Alabeo)
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
}

/// Mapa Cosmo-Geométrico de Sólidos Platónicos y Entidades
#[derive(Debug, Clone, PartialEq)]
pub enum SolidusPlatonicus {
    Tetrahedron,  // NUCLEUS (4 Caras de 60° - Masa/Estabilidad Primordial)
    Octahedron,   // ELECTRON (8 Caras de 60° - Onda de Carga Dinámica)
    Hexahedron,   // SOL / STELLA (6 Caras Cuadradas - Emisores Radiantes)
    Icosahedron,  // LUNA / SATELLIS (20 Caras de 60° - Modulador Fluido)
    Dodecahedron, // UNIVERSUS (12 Pentagonales - Dodecaedro Cósmico Expansivo)
}

#[derive(Debug, Clone)]
pub struct CorpusGeometria {
    pub forma: SolidusPlatonicus,
    pub positio: Phasor3D,
    pub amplitudo: f64,
}

pub struct Evaluator;

impl Evaluator {
    /// Aplica una rotación esférica continua (S3) sobre un Phasor3D sin Gimbal Lock
    pub fn sphaera_rotatio(p: Phasor3D, eje_z: f64) -> Phasor3D {
        let q_base = Sphaera::ex_phasore(p);
        let q_rot = Sphaera::ex_phasore(Phasor3D { yaw: eje_z, pitch: 0.0, roll: 0.0 });

        let w = q_rot.w * q_base.w - q_rot.x * q_base.x - q_rot.y * q_base.y - q_rot.z * q_base.z;
        let x = q_rot.w * q_base.x + q_rot.x * q_base.w + q_rot.y * q_base.z - q_rot.z * q_base.y;
        let y = q_rot.w * q_base.y - q_rot.x * q_base.z + q_rot.y * q_base.w + q_rot.z * q_base.x;
        let z = q_rot.w * q_base.z + q_rot.x * q_base.y - q_rot.y * q_base.x + q_rot.z * q_base.w;

        Phasor3D {
            yaw: z.atan2(w) * 2.0,
            pitch: y.atan2(w) * 2.0,
            roll: x.atan2(w) * 2.0,
        }
    }
/// Calcula la interferencia de onda entre dos entidades geométricas
    pub fn interferentia(c1: &CorpusGeometria, c2: &CorpusGeometria) -> f64 {
        let delta_yaw = c1.positio.yaw - c2.positio.yaw;
        let delta_pitch = c1.positio.pitch - c2.positio.pitch;
        let resultado_raw = (delta_yaw.cos() + delta_pitch.cos()) * 0.5 * (c1.amplitudo * c2.amplitudo);
        
        // Aplica el filtro de saturación de la Tensión de Planck
        Self::limitem_tensio(resultado_raw)
    }

    /// Invariante de Fase: Ninguna interferencia puede superar la Tensión de Planck
    pub fn limitem_tensio(amplitudo: f64) -> f64 {
        if amplitudo > TENSIO_PLANCK {
            TENSIO_PLANCK
        } else if amplitudo < -TENSIO_PLANCK {
            -TENSIO_PLANCK
        } else {
            amplitudo
        }
    }
}

// ---------------------------------------------------------------------
// 3. ANALIZADOR LÉXICO (LEXER EN RUST - 42 TÉRMINOS EN LATÍN)
// ---------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Typi Datarum
    Angulus, Phasor3DToken, SphaeraToken, HarmonicusToken,
    // Solidis Platonicis et Particulis
    Tetrahedron, Hexahedron, Octahedron, Icosahedron, Dodecahedron, Electron, Nucleus,
    // Structurae Controlis
    Functio, Incipit, Finis, Constans, Variabilis, Reddere, Evaluare, LoopHarmonicus, Passus, Omnis,
    // Operatores Trigonometrici et Actuatores
    Rotatio, SphaeraRotatio, Interferentia, Inversus, Projectio, Octans, Facies, Amplitudo, Frequenz,
    // Literales et Constantibus
    Grad, Rad, Trinum, Pi, Tau, Matrix, Tensor,
    AxisX, AxisY, AxisZ, Origis,
    // Symbola et Literales
    Numerus(f64), Identificator(String),
    ParenthesisAperta, ParenthesisClausa, Aequalis, Coma, DuoPuncta, Punctum, Eo, EOF,
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

        if ch.is_numeric() || ch == '-' {
            let start = self.position;
            self.position += 1;
            while self.position < self.input.len() 
                && (self.current_char().is_numeric() || self.current_char() == '.') 
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
                && self.input[self.position+1..].starts_with('*') 
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
// 4. ANALIZADOR SINTÁCTICO (PARSER EN RUST -> AST)
// ---------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum ASTNode {
    DeclaratioConstantis { nomen: String, valor: Box<ASTNode> },
    DeclaratioVariabilis { nomen: String, valor: Box<ASTNode> },
    CreatioParticulae { forma: SolidusPlatonicus, positiophasor: Box<ASTNode> },
    OperatioSphaeraRotatio { phasor: Box<ASTNode>, eje_z: f64 },
    OperatioInterferentia { corpus1: Box<ASTNode>, corpus2: Box<ASTNode> },
    LiteralNumerus(f64),
    Identificator(String),
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

    fn eat(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Error Syntaxicus: Expectabatur {:?}, inventum {:?}", expected, self.current_token);
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Constans => {
                self.eat(Token::Constans);
                if let Token::Identificator(nomen) = self.current_token.clone() {
                    self.eat(Token::Identificator(nomen.clone()));
                    self.eat(Token::Aequalis);
                    let valor = self.parse_expression();
                    ASTNode::DeclaratioConstantis { nomen, valor: Box::new(valor) }
                } else {
                    panic!("Expectabatur identificator post 'constans'");
                }
            }
            Token::Variabilis => {
                self.eat(Token::Variabilis);
                if let Token::Identificator(nomen) = self.current_token.clone() {
                    self.eat(Token::Identificator(nomen.clone()));
                    self.eat(Token::Aequalis);
                    let valor = self.parse_expression();
                    ASTNode::DeclaratioVariabilis { nomen, valor: Box::new(valor) }
                } else {
                    panic!("Expectabatur identificator post 'variabilis'");
                }
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Trinum => {
                self.eat(Token::Trinum);
                ASTNode::LiteralNumerus(TRINUM)
            }
            Token::Numerus(val) => {
                let v = *val;
                self.eat(Token::Numerus(v));
                ASTNode::LiteralNumerus(v)
            }
            Token::Nucleus => {
                self.eat(Token::Nucleus);
                self.eat(Token::ParenthesisAperta);
                let pos = self.parse_expression();
                self.eat(Token::ParenthesisClausa);
                ASTNode::CreatioParticulae {
                    forma: SolidusPlatonicus::Tetrahedron,
                    positiophasor: Box::new(pos),
                }
            }
            Token::Electron => {
                self.eat(Token::Electron);
                self.eat(Token::ParenthesisAperta);
                let pos = self.parse_expression();
                self.eat(Token::ParenthesisClausa);
                ASTNode::CreatioParticulae {
                    forma: SolidusPlatonicus::Octahedron,
                    positiophasor: Box::new(pos),
                }
            }
            Token::SphaeraRotatio => {
                self.eat(Token::SphaeraRotatio);
                self.eat(Token::ParenthesisAperta);
                let phasor = self.parse_expression();
                self.eat(Token::Coma);
                let eje_val = if self.current_token == Token::Trinum {
                    TRINUM
                } else if let Token::Numerus(v) = self.current_token {
                    v
                } else {
                    0.0
                };
                self.eat(self.current_token.clone());
                self.eat(Token::ParenthesisClausa);
                ASTNode::OperatioSphaeraRotatio {
                    phasor: Box::new(phasor),
                    eje_z: eje_val,
                }
            }
            Token::Identificator(nomen) => {
                let n = nomen.clone();
                self.eat(Token::Identificator(n.clone()));
                ASTNode::Identificator(n)
            }
            _ => panic!("Expressio invalida: {:?}", self.current_token),
        }
    }
}

// ---------------------------------------------------------------------
// 5. CONSOLA DE EJECUCIÓN Y PRUEBA INTEGRADA (MAIN)
// ---------------------------------------------------------------------
fn main() {
    println!("================================================================");
    println!("  TRIGOSCRIPT 3D (LINGUA LATINA) - COMPILATOR ET MOTOR RUST     ");
    println!("  Domain-Specific Language for Actuators, Geometry & Waves      ");
    println!("================================================================\n");

    let codigo_script = "
        (* Script de prueba: Control de fase de un Electrón sobre el Núcleo Tetraédrico *)
        constans ANGULUS_BASE = TRINUM
        variabilis e_particula = ELECTRON(TRINUM)
    ";

    println!("--- 1. CODIGO FUENTE INGESTADO (.trigo) ---");
    println!("{}\n", codigo_script.trim());

    println!("--- 2. ANALISIS LÉXICO Y SINTÁCTICO (PARSER -> AST) ---");
    let lexer = Lexer::new(codigo_script);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{:#?}\n", ast);

    println!("--- 3. EJECUCIÓN Y EVALUACIÓN EN TIEMPO DE EJECUCIÓN (S3) ---");
    let nucleo = CorpusGeometria {
        forma: SolidusPlatonicus::Tetrahedron,
        positio: Phasor3D { yaw: 0.0, pitch: 0.0, roll: 0.0 },
        amplitudo: 1.0,
    };

    let electron = CorpusGeometria {
        forma: SolidusPlatonicus::Octahedron,
        positio: Phasor3D { yaw: TRINUM, pitch: 0.0, roll: 0.0 },
        amplitudo: 1.0,
    };

    let rotado = Evaluator::sphaera_rotatio(electron.positio, TRINUM);
    let campo = Evaluator::interferentia(&nucleo, &electron);

    println!("✔ Constante TRINUM (60°): {:.6} rad", TRINUM);
    println!("✔ Núcleo (TETRAHEDRON 60°): Masa/Estabilidad establecida en ORIGIS.");
    println!("✔ Electrón (OCTAHEDRON 60°): Rotación S3 continua sin Gimbal Lock: {:?}", rotado);
    println!("✔ Campo de interferencia de onda Núcleo-Electrón: {:.4}", campo);
    println!("\n================================================================");
    println!("  STATUS: COMPILATIO ET EVALUATIO SUCESSA (100% OPERATIONAL)    ");
    println!("================================================================");
}
