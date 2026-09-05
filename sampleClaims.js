/**
 * VeritasGonka — Sample Civic Claims & Fallback Verification Data
 * Provides rich, authoritative sample scenarios for testing and demonstration.
 */

const SAMPLE_CLAIMS = {
  medical: {
    id: "claim-med-01",
    domain: "Public Health & Medicine",
    claim: "WHO announces coffee cures Covid-19 mutations",
    truthScore: 8,
    verdictLabel: "FABRICATED & DEBUNKED",
    statusClass: "false",
    headline: "Unsubstantiated Medical Assertion Contradicted by Global Health Data",
    summary: "No clinical trial, peer-reviewed paper, or World Health Organization bulletin supports the claim that caffeine or coffee bioactive compounds neutralize SARS-CoV-2 viral mutations. Official WHO advisories explicitly designate this viral claim as dangerous health misinformation.",
    factualityScore: 5,
    consensusScore: 98,
    fallacyRisk: "Critical",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "False",
      stanceClass: "false",
      confidence: 96,
      latency: 420,
      tokens: 384,
      reqId: "gonka-req-ds-91a7c4f028b1",
      assessment: "Causal analysis reveals a complete failure of biological plausibility: dietary caffeine does not bind to the viral spike protein or inhibit RNA-dependent RNA polymerase in vivo. Attributing this finding to the WHO represents a classic false authority attribution fallacy."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "False",
      stanceClass: "false",
      confidence: 94,
      latency: 490,
      tokens: 412,
      reqId: "gonka-req-mm-4e8b11c9f032",
      assessment: "Source cross-referencing across WHO Press Briefings (2020–2026) and the WHO Mythbusters repository confirmed zero mentions of coffee as a therapeutic agent. Viral propagation traced back to unverified WhatsApp chains and clickbait health blogs."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "False",
      stanceClass: "false",
      confidence: 97,
      latency: 450,
      tokens: 398,
      reqId: "gonka-req-km-2a91f5e8d074",
      assessment: "Anti-hallucination verification confirmed that no medical index (PubMed, Lancet, JAMA, Cochrane) indexes any clinical trial demonstrating curative efficacy of coffee against Covid-19 variants. High risk of treatment delay if consumers rely on false cure claims."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Claim Decomposition & Attribution Check",
        desc: "Extracted primary named entity 'World Health Organization' and asserted therapeutic mechanism 'coffee cures Covid-19 mutations'."
      },
      {
        step: 2,
        title: "Authoritative Database Query",
        desc: "Cross-examined WHO official disease outbreak database and clinical trials registries (ClinicalTrials.gov). Found zero corroborating records."
      },
      {
        step: 3,
        title: "Forensic Fallacy Scan",
        desc: "Identified false attribution of institutional authority (appealing to WHO brand credibility to legitimize folkloric remedy)."
      },
      {
        step: 4,
        title: "Multi-Model Bayesian Synthesis",
        desc: "All 3 models achieved unanimous consensus on False stance (agreement rate: 98%), yielding a calibrated Truth Score of 8%."
      }
    ],
    fallacies: [
      "False Authority Attribution: Fabricating WHO institutional endorsement.",
      "Causal Overreach: Conflating antioxidant beverage properties with viral neutralization.",
      "Dangerous Health Misinformation: Potentially deterring evidence-based medical treatment."
    ],
    citations: [
      { title: "World Health Organization (WHO) — Coronavirus Mythbusters Repository", url: "https://who.int/emergencies/diseases/novel-coronavirus-2019/advice-for-public/myth-busters" },
      { title: "The Lancet Infectious Diseases — Therapeutics & Prophylaxis Review", url: "https://thelancet.com/journals/laninf" },
      { title: "Cochrane Systematic Reviews — Interventions for SARS-CoV-2", url: "https://cochranelibrary.com" }
    ]
  },

  science: {
    id: "claim-sci-02",
    domain: "Space Science & Astronomy",
    claim: "James Webb Space Telescope detects atmospheric water vapor and carbon dioxide on exoplanet K2-18b",
    truthScore: 94,
    verdictLabel: "VERIFIED ACCURATE",
    statusClass: "true",
    headline: "Astrophysical Spectroscopy Confirms Carbon-Bearing Molecules on K2-18b",
    summary: "Data collected by the James Webb Space Telescope (JWST) NIRISS and NIRSpec instruments confirmed the presence of carbon dioxide (CO2) and methane (CH4) alongside hints of water vapor in the atmosphere of habitable-zone sub-Neptune exoplanet K2-18b, published in peer-reviewed astronomical journals.",
    factualityScore: 96,
    consensusScore: 95,
    fallacyRisk: "Negligible",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "Verified True",
      stanceClass: "true",
      confidence: 95,
      latency: 410,
      tokens: 420,
      reqId: "gonka-req-ds-6b22c7a914f3",
      assessment: "Spectroscopic transmission spectra gathered by JWST consistently reflect absorption signatures of carbon-rich molecules in K2-18b's upper atmosphere. The underlying scientific methodology meets rigorous peer-review replication standards."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "Verified True",
      stanceClass: "true",
      confidence: 93,
      latency: 480,
      tokens: 440,
      reqId: "gonka-req-mm-81d4e92a7f50",
      assessment: "Validated against NASA astrophysics press releases and the Astrophysical Journal Letters publication (Madhusudhan et al.). Nuance note: initial claims of dimethyl sulfide (DMS) remain tentative, but CO2 and carbon chemistry detections are fully confirmed."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "Verified True",
      stanceClass: "true",
      confidence: 94,
      latency: 435,
      tokens: 405,
      reqId: "gonka-req-km-3f11a8b9c621",
      assessment: "Corroborated with European Space Agency (ESA) and Space Telescope Science Institute (STScI) raw instrument data releases. Scientific reporting is accurate and devoid of sensationalist distortion."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Spectroscopic Mission Confirmation",
        desc: "Queried NASA/ESA/CSA JWST operational logs for Cycle 1 observations of exoplanet K2-18b."
      },
      {
        step: 2,
        title: "Peer-Review Literature Matching",
        desc: "Matched transmission spectrum results with Madhusudhan et al. in The Astrophysical Journal Letters."
      },
      {
        step: 3,
        title: "Nuance & Speculation Separation",
        desc: "Isolated confirmed detections (CO2, CH4, atmospheric water vapor) from speculative biosignature candidates (DMS)."
      },
      {
        step: 4,
        title: "Consensus Aggregation",
        desc: "Models aligned on high-fidelity factual confirmation with 94% composite truth score."
      }
    ],
    fallacies: [
      "No critical logical fallacies detected. Reporting adheres to empirical findings."
    ],
    citations: [
      { title: "The Astrophysical Journal Letters — Carbon-bearing Molecules on K2-18b", url: "https://iopscience.iop.org/journal/2041-8205" },
      { title: "NASA Webb Mission — Exoplanet K2-18b Atmospheric Composition", url: "https://nasa.gov/mission_pages/webb" },
      { title: "European Space Agency (ESA) Science & Technology Release", url: "https://esa.int/Science_Exploration" }
    ]
  },

  economic: {
    id: "claim-econ-03",
    domain: "Labor Policy & Macroeconomics",
    claim: "A 4-day work week increases corporate productivity by 40% across all global industries",
    truthScore: 52,
    verdictLabel: "PARTIALLY TRUE / OVERSTATED",
    statusClass: "mixed",
    headline: "Productivity Gains Documented in Knowledge Sectors, but Universality is Overstated",
    summary: "Large-scale pilot trials (such as 4 Day Week Global trials in the UK and US) demonstrated substantial employee retention, reduced burnout, and modest revenue gains (~1.4% to ~15%). However, asserting a '40% productivity increase across all global industries' drastically exaggerates the magnitude and ignores capital-intensive/shift-based sectors.",
    factualityScore: 55,
    consensusScore: 82,
    fallacyRisk: "Moderate",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "Overstated Context",
      stanceClass: "mixed",
      confidence: 88,
      latency: 440,
      tokens: 430,
      reqId: "gonka-req-ds-7c18a2d590e1",
      assessment: "The 40% figure represents an unrepresentative cherry-picked metric (often self-reported stress reduction or small agency billing rates) rather than macroeconomic output per hour. Industrial generalization commits an ecological fallacy."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "Mixed Accuracy",
      stanceClass: "mixed",
      confidence: 86,
      latency: 510,
      tokens: 460,
      reqId: "gonka-req-mm-19c4d3e8a205",
      assessment: "Evaluated pilot trial datasets published by Boston College and Cambridge University. While 92% of participating companies opted to continue the 4-day policy, productivity gains averaged single digits, not 40%."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "Mixed / Nuanced",
      stanceClass: "mixed",
      confidence: 87,
      latency: 460,
      tokens: 425,
      reqId: "gonka-req-km-90b5a1f8c734",
      assessment: "Service and manufacturing sectors require physical shift coverage where 4-day transitions necessitate additional staffing rather than automatic productivity expansion. Claim requires strong contextual caveats."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Quantitative Metric Extraction",
        desc: "Isolated numerical assertion '40% productivity increase' and universal quantifier 'across all global industries'."
      },
      {
        step: 2,
        title: "Empirical Trial Comparison",
        desc: "Benchmarked against the 2022-2024 UK 4-Day Work Week Pilot results (Cambridge/Boston College)."
      },
      {
        step: 3,
        title: "Fallacy Identification",
        desc: "Detected Hasty Generalization (extrapolating tech/marketing agency trials to continuous-shift heavy manufacturing)."
      },
      {
        step: 4,
        title: "Calibrated Consensus",
        desc: "Consensus classifies statement as Partially Accurate with Overstated Context (Truth Score: 52%)."
      }
    ],
    fallacies: [
      "Hasty Generalization: Extrapolating boutique knowledge-work trials to all global industries.",
      "Metric Inflation: Conflating employee well-being scores with hard economic productivity (output per labor-hour)."
    ],
    citations: [
      { title: "University of Cambridge / Boston College — UK 4-Day Work Week Pilot Results", url: "https://cam.ac.uk/research/news" },
      { title: "4 Day Week Global Foundation — Global Research Report", url: "https://4dayweek.com/research" },
      { title: "OECD Economic Studies — Working Hours and Labor Productivity Trends", url: "https://oecd.org/employment" }
    ]
  },

  climate: {
    id: "claim-clim-04",
    domain: "Energy & Environmental Science",
    claim: "Electric vehicles produce more lifetime carbon emissions than diesel trucks according to 2025 Stanford study",
    truthScore: 14,
    verdictLabel: "FABRICATED & DEBUNKED",
    statusClass: "false",
    headline: "Fabricated Academic Attribution Contradicts Decades of Life Cycle Assessments",
    summary: "No Stanford University study in 2025 or earlier found that electric vehicles emit more lifetime greenhouse gases than diesel vehicles. Peer-reviewed Life Cycle Assessments (LCAs) from Argonne National Laboratory, MIT, and Stanford show modern EVs produce 50% to 70% fewer lifetime lifecycle emissions than internal combustion counterparts.",
    factualityScore: 10,
    consensusScore: 96,
    fallacyRisk: "High",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "False / Fabricated",
      stanceClass: "false",
      confidence: 96,
      latency: 415,
      tokens: 410,
      reqId: "gonka-req-ds-5d89b1c4e207",
      assessment: "Life-cycle carbon accounting unequivocally demonstrates that while battery manufacturing carries upfront emissions debt, the operational efficiency of electric powertrains yields dramatic net reductions within 1.5 to 2.5 years of average driving."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "False",
      stanceClass: "false",
      confidence: 95,
      latency: 485,
      tokens: 430,
      reqId: "gonka-req-mm-3a47c8f1e912",
      assessment: "Stanford Precourt Institute for Energy literature search found zero studies matching this conclusion. The attribution is fraudulent and mirrors classic viral discrediting campaigns circulated on social networks."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "False",
      stanceClass: "false",
      confidence: 95,
      latency: 440,
      tokens: 395,
      reqId: "gonka-req-km-8e20f4b7a136",
      assessment: "GREET (Greenhouse gases, Regulated Emissions, and Energy use in Technologies) life-cycle modeling directly refutes the diesel equivalence hypothesis across virtually all global electricity grid mixes."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Citation & Institutional Query",
        desc: "Queried Stanford University library archives and Faculty publication indices for 2025 EV vs Diesel LCA studies. No record exists."
      },
      {
        step: 2,
        title: "Thermodynamic & Life Cycle Modeling",
        desc: "Calculated typical cradle-to-grave emissions across battery manufacturing, grid electricity mix, and diesel tailpipe combustion."
      },
      {
        step: 3,
        title: "Disinformation Genealogy",
        desc: "Traced origin to misinterpretations of 2019 German IFO think-tank working papers, long refuted by peer review."
      },
      {
        step: 4,
        title: "Synthesis",
        desc: "Unanimous False consensus with high confidence, establishing a 14% Truth Score."
      }
    ],
    fallacies: [
      "Fabricated Academic Authority: Inventing a non-existent '2025 Stanford study'.",
      "Upstream Cherry-Picking: Exaggerating battery manufacturing emissions while ignoring crude oil extraction, refining, and transportation emissions."
    ],
    citations: [
      { title: "Stanford Precourt Institute for Energy — Transportation Decarbonization", url: "https://energy.stanford.edu" },
      { title: "Argonne National Laboratory — GREET Model Life-Cycle Analysis", url: "https://greet.es.anl.gov" },
      { title: "International Council on Clean Transportation (ICCT) — Global EV LCA Report", url: "https://theicct.org/publications" }
    ]
  },

  spanish: {
    id: "claim-es-05",
    domain: "Salud Pública (Multilingual)",
    claim: "El consumo diario de limón en ayunas cura la diabetes tipo 2 según la OMS",
    truthScore: 8,
    verdictLabel: "FALSO Y DESMENTIDO",
    statusClass: "false",
    headline: "Afirmación Médica Falsa Sin Respaldo Científico ni Institucional",
    summary: "La Organización Mundial de la Salud (OMS) nunca ha emitido directrices afirmando que el limón en ayunas cure la diabetes mellitus tipo 2. La diabetes es un trastorno metabólico crónico que requiere control glucémico, nutrición adecuada y fármacos prescritos por profesionales de la salud.",
    factualityScore: 5,
    consensusScore: 97,
    fallacyRisk: "Crítico",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "Falso",
      stanceClass: "false",
      confidence: 96,
      latency: 425,
      tokens: 400,
      reqId: "gonka-req-ds-1f99c2d8b440",
      assessment: "El ácido cítrico y la vitamina C presentes en el limón no tienen capacidad de regenerar la función de las células beta pancreáticas ni revertir la resistencia a la insulina. Atribuir una cura milagrosa a un alimento básico carece de sustento fisiológico."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "Falso",
      stanceClass: "false",
      confidence: 95,
      latency: 495,
      tokens: 420,
      reqId: "gonka-req-mm-7c88a1f2e031",
      assessment: "Se cotejó la base de datos oficial de la Organización Panamericana de la Salud (OPS/OMS). No existe ningún informe que avale esta afirmación; la difusión proviene de cadenas desinformativas de mensajería instantánea."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "Falso",
      stanceClass: "false",
      confidence: 97,
      latency: 445,
      tokens: 390,
      reqId: "gonka-req-km-4e12a9c7f655",
      assessment: "La Asociación Americana de la Diabetes (ADA) y guías clínicas internacionales alertan que abandonar el tratamiento médico por remedios caseros genera complicaciones graves como cetoacidosis o daño renal."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Descomposición y Análisis de Entidades",
        desc: "Se aisló la entidad 'OMS' y la afirmación terapéutica 'cura la diabetes tipo 2 con limón en ayunas'."
      },
      {
        step: 2,
        title: "Búsqueda en Registros Clínicos Oficiales",
        desc: "Cotejo con la biblioteca Cochrane y bases de datos de la OPS/OMS. Cero evidencia clínica."
      },
      {
        step: 3,
        title: "Detección de Falacias Retóricas",
        desc: "Apelación a la falsa autoridad institucional y falacia de solución milagrosa natural."
      },
      {
        step: 4,
        title: "Consenso Multi-Modelo",
        desc: "Consenso unánime de los 3 modelos evaluados mediante Gonka Router con 8% de puntuación de veracidad."
      }
    ],
    fallacies: [
      "Falsa Autoridad: Suplantación fraudulenta de la Organización Mundial de la Salud.",
      "Falacia de la Cura Milagrosa: Promover un cítrico común como reemplazo de terapia metabólica validada."
    ],
    citations: [
      { title: "Organización Panamericana de la Salud (OPS/OMS) — Información sobre Diabetes", url: "https://paho.org/es/temas/diabetes" },
      { title: "American Diabetes Association (ADA) — Standards of Care in Diabetes", url: "https://diabetesjournals.org/care" }
    ]
  },
  tiktok_health: {
    id: "claim-reel-01",
    domain: "Viral Social Media (TikTok Reel)",
    isVideo: true,
    videoPlatform: "TikTok",
    videoUrl: "https://www.tiktok.com/@healthhacks/video/739120485912384",
    spokenTranscript: "Guys, stop taking kidney medication! If you drink 1 tablespoon of baking soda in warm water twice a day, it completely dissolves all kidney stones in under 24 hours and purifies your kidneys automatically!",
    claim: "TikTok Reel spoken claim: Baking soda in warm water dissolves all kidney stones in 24 hours and replaces medical treatment",
    truthScore: 12,
    verdictLabel: "DANGEROUS PSEUDOSCIENCE",
    statusClass: "false",
    headline: "Viral TikTok Remedy Contradicted by Renal Nephrology Science & High Sodium Risks",
    summary: "The spoken claim in this TikTok reel promotes ingestion of sodium bicarbonate (baking soda) as an instant cure for kidney stones. Clinical nephrology data shows sodium bicarbonate cannot dissolve calcium oxalate stones (the majority of cases) and high oral sodium intake increases urinary calcium excretion, worsening stone formation and posing severe hypertension risks.",
    factualityScore: 10,
    consensusScore: 96,
    fallacyRisk: "Critical",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "False",
      stanceClass: "false",
      confidence: 97,
      latency: 410,
      tokens: 395,
      reqId: "gonka-req-ds-tk77a11209e1",
      assessment: "Causal analysis of renal physiology: 80%+ of nephrolithiasis cases are calcium oxalate stones, which are insoluble in alkalized urine. High oral sodium loading (baking soda) increases metabolic sodium load, exacerbating hypercalciuria and hypertension. The video assertion lacks biochemical mechanism."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "False",
      stanceClass: "false",
      confidence: 95,
      latency: 480,
      tokens: 425,
      reqId: "gonka-req-mm-tk88b22310f2",
      assessment: "Video transcript cross-check: The spoken dialogue makes absolute efficacy claims ('dissolves all kidney stones in 24h') while misleadingly discouraging prescribed nephrology care. Verified across Mayo Clinic and National Kidney Foundation clinical guidelines as harmful advice."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "False",
      stanceClass: "false",
      confidence: 96,
      latency: 440,
      tokens: 405,
      reqId: "gonka-req-km-tk99c33411g3",
      assessment: "Database cross-reference against PubMed & National Institute of Diabetes and Digestive and Kidney Diseases (NIDDK): zero clinical evidence supports rapid stone dissolution via baking soda. Misleading clickbait presentation poses acute metabolic alkalosis risks."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Audio Transcript & Spoken Claim Isolation",
        desc: "Extracted spoken audio from TikTok Reel: 'baking soda in warm water dissolves all kidney stones in 24 hours'. Disregarded clickbait promotional video title."
      },
      {
        step: 2,
        title: "Clinical Nephrology Registry Check",
        desc: "Cross-referenced National Kidney Foundation (NKF) and NIDDK treatment guidelines for calcium oxalate and uric acid lithiasis."
      },
      {
        step: 3,
        title: "Toxicity & Risk Evaluation",
        desc: "Identified severe risks: acute sodium overload, metabolic alkalosis, and delayed essential medical intervention."
      },
      {
        step: 4,
        title: "Gonka Multi-Model Consensus",
        desc: "Unanimous False verdict from 3 models with 12% Bayesian Truth Score."
      }
    ],
    fallacies: [
      "Pseudoscience Sensationalism: Claiming 24-hour instant cures for complex renal conditions.",
      "Dangerous Medical Substitution: Urging viewers to discontinue prescribed kidney medication.",
      "Out-of-Context Chemical Simplification: Confusing urinary alkalization for specific uric acid stones with universal stone dissolution."
    ],
    citations: [
      { title: "National Kidney Foundation — Kidney Stones Treatment & Prevention", url: "https://www.kidney.org/atoz/content/kidneystones" },
      { title: "NIDDK — Eating, Diet, & Nutrition for Kidney Stones", url: "https://www.niddk.nih.gov/health-information/urologic-diseases/kidney-stones/eating-diet-nutrition" }
    ]
  },
  yt_shorts_tech: {
    id: "claim-reel-02",
    domain: "Viral Social Media (YouTube Shorts)",
    isVideo: true,
    videoPlatform: "YouTube Shorts",
    videoUrl: "https://www.youtube.com/shorts/x9K2pW8vM1q",
    spokenTranscript: "Never leave your smartphone charging overnight! On the new iOS update, leaving it plugged in past 100% causes thermal runaway and battery explosion in your bedroom!",
    claim: "YouTube Short spoken claim: Overnight smartphone charging past 100% causes battery explosion on modern operating systems",
    truthScore: 18,
    verdictLabel: "MISLEADING & SENSATIONALIZED",
    statusClass: "false",
    headline: "Modern Lithium-Ion Battery Management Protection Renders Explosion Claims False",
    summary: "The spoken audio in this YouTube Short falsely asserts that overnight charging causes battery explosion. Modern smartphones use Battery Management Systems (BMS) with hardware cut-off circuits, optimized battery charging algorithms, and automated power throttling that halt current flow when the battery reaches 100%.",
    factualityScore: 20,
    consensusScore: 92,
    fallacyRisk: "High",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "False",
      stanceClass: "false",
      confidence: 94,
      latency: 395,
      tokens: 370,
      reqId: "gonka-req-ds-yt11a2233445",
      assessment: "Hardware mechanics: Integrated PMICs (Power Management ICs) automatically terminate fast charge current at high SOC (State of Charge) and switch to micro-trickle/idle bypass mode. Thermal runaway requires internal physical puncture or severe manufacturing defect, not regular overnight charging."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "False",
      stanceClass: "false",
      confidence: 93,
      latency: 460,
      tokens: 390,
      reqId: "gonka-req-mm-yt22b3344556",
      assessment: "Source check: IEEE Standards for Rechargeable Batteries (IEEE 1725/1625) and manufacturer specs confirm multi-layered overvoltage protection. Leaving phone under a heavy pillow while charging may cause heat retention, but battery explosion claims are clickbait fearmongering."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "False",
      stanceClass: "false",
      confidence: 95,
      latency: 420,
      tokens: 380,
      reqId: "gonka-req-km-yt33c4455667",
      assessment: "Anti-hallucination verification: Zero technical recalls or UL safety warnings support battery explosion from standard overnight wall adapter charging. Battery degradation over years may increase slightly, but explosive risk assertion is false."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Spoken Dialogue Extraction",
        desc: "Isolated spoken claim: 'charging overnight past 100% causes thermal runaway and battery explosion'."
      },
      {
        step: 2,
        title: "Electrical & Battery Architecture Audit",
        desc: "Cross-examined IEEE 1725 battery safety standards and hardware PMIC cut-off specifications."
      },
      {
        step: 3,
        title: "Consensus Synthesis",
        desc: "Calculated 18% truth score. Misleading fearmongering over minor heat retention advice."
      }
    ],
    fallacies: [
      "Alarmist Fearmongering: Conflating standard long-term battery degradation with explosive catastrophic failure.",
      "Technical Misdirection: Ignoring modern integrated microchip charge controllers."
    ],
    citations: [
      { title: "IEEE Standards Association — IEEE 1725 Standard for Rechargeable Batteries for Mobile Phones", url: "https://standards.ieee.org" },
      { title: "Apple Support — About Optimized Battery Charging on iPhone", url: "https://support.apple.com/en-us/108055" }
    ]
  },
  ig_reels_finance: {
    id: "claim-reel-03",
    domain: "Viral Social Media (Instagram Reel)",
    isVideo: true,
    videoPlatform: "Instagram Reels",
    videoUrl: "https://www.instagram.com/reel/C8kL0p9M4xa",
    spokenTranscript: "Banks don't want you to know this legal secret! You can discharge your home mortgage and car loan to zero by filing IRS Form 1099-OID because the bank created the money out of thin air!",
    claim: "Instagram Reel spoken claim: IRS Form 1099-OID can be filed to legally erase home mortgages and credit card debt",
    truthScore: 5,
    verdictLabel: "FRAUDULENT FINANCIAL SCHEME",
    statusClass: "false",
    headline: "Debunked Sovereign Citizen Tax Fraud Scheme Irresponsibly Promoted on Reels",
    summary: "The spoken audio in this Instagram Reel promotes a well-known tax fraud scam based on pseudo-legal 'sovereign citizen' theories. IRS Form 1099-OID is used to report original issue discount income, not to discharge lawful consumer debt. Filing false 1099-OID forms leads to heavy civil penalties ($5,000+ per frivolous return) and federal criminal prosecution.",
    factualityScore: 5,
    consensusScore: 99,
    fallacyRisk: "Critical",
    deepseek: {
      model: "deepseek-ai/deepseek-v4",
      stance: "False",
      stanceClass: "false",
      confidence: 99,
      latency: 380,
      tokens: 360,
      reqId: "gonka-req-ds-ig99a8877665",
      assessment: "Legal-financial analysis: Form 1099-OID is an information return for debt instruments issued at a discount. Promising debt elimination through fraudulent 1099-OID filings misconstrues commercial paper law and tax code sections 6702/7206."
    },
    minimax: {
      model: "minimax/minimax-m2.7",
      stance: "False",
      stanceClass: "false",
      confidence: 98,
      latency: 450,
      tokens: 410,
      reqId: "gonka-req-mm-ig88b7766554",
      assessment: "Source examination: IRS Official Consumer Alerts explicitly list '1099-OID Debt Elimination Fraud' on the IRS 'Dirty Dozen' tax scams list. Federal courts consistently reject these arguments and impose prison sentences on promoters."
    },
    kimi: {
      model: "moonshot/kimi-k2.6",
      stance: "False",
      stanceClass: "false",
      confidence: 99,
      latency: 410,
      tokens: 375,
      reqId: "gonka-req-km-ig77c6655443",
      assessment: "Anti-hallucination database check: Cross-checked US Tax Court rulings and Financial Crimes Enforcement Network (FinCEN) advisories. Zero legal precedents support 1099-OID mortgage discharge."
    },
    reasoningTrace: [
      {
        step: 1,
        title: "Spoken Dialogue & Claim Extraction",
        desc: "Extracted audio claim: 'filing IRS Form 1099-OID discharges home mortgage and credit card debt'."
      },
      {
        step: 2,
        title: "IRS & Federal Tax Code Cross-Check",
        desc: "Checked IRS Bulletin Notice 2010-28 and IRS Dirty Dozen Fraud List."
      },
      {
        step: 3,
        title: "Multi-Model Risk Assessment",
        desc: "Identified critical financial fraud risk and severe federal penalty warnings."
      }
    ],
    fallacies: [
      "Pseudo-Legal Mythmaking: Misrepresenting standard tax forms as secret debt cancellation keys.",
      "Conspiracy Attribution: 'Banks don't want you to know this secret' clickbait framing."
    ],
    citations: [
      { title: "IRS Dirty Dozen Tax Scams — Frivolous Arguments & Form 1099-OID Fraud", url: "https://www.irs.gov/newsroom/dirty-dozen" },
      { title: "U.S. Department of Justice — Tax Division Prosecutions of 1099-OID Schemes", url: "https://www.justice.gov/tax" }
    ]
  }
};

