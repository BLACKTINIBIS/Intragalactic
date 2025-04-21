use rand::{Rng};
use crate::game::maths::*;

pub fn random_name() -> String {
    const MIN_SYLLABLE_COUNT: usize = 2; // INCLUSIVE
    const MAX_SYLLABLE_COUNT: usize = 5; // INCLUSIVE

    let mut rng = rand::rng();
    // letter sets
    let cons = "bcdfghjklmnpqrst";
    let vows = "aeiouy";
    //let cons_spcy = "š§";       // genuinely spicy, don't use
    //let vows_spcy = "œ°æª¸";    // same here too
    let spcy = "'`-~";
    let name;

    /* REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR REFACTOR */

    //let start = Instant::now();     //time

    //determine the number of syllables
    let syllable_count = rng.random_range(MIN_SYLLABLE_COUNT..=MAX_SYLLABLE_COUNT);

    //for each syllable
    name = (0..syllable_count).map(|_| {
        let mut rng = rand::rng();
        let mut syllable: String = String::with_capacity(2);
        // maybe add a consonant ?
        if roll_d20(1) > 5 { // regular
            let r_index = rng.random_range(0..cons.len());
            syllable += &cons[r_index..r_index+1];
        }

        // always add a vowel
        let r_index = rng.random_range(0..vows.len());
        syllable += &vows[r_index..r_index+1];

        // maybe add some spicy ?
        if roll_d20(1) > 18 {
            let r_index = rng.random_range(0..spcy.len());
            syllable += &spcy[r_index..r_index+1];
        }

        syllable
    }).collect::<String>();

    //println!("{}",start.elapsed().as_nanos());  //timer

    capitalize(&name)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn get_resources() -> Vec<String> {
    let slices = Vec::from([
        "Hydrogen",
        "Helium",
        "Lithium",
        "Beryllium",
        "Boron",
        "Carbon",
        "Nitrogen",
        "Oxygen",
        "Fluorine",
        "Neon",
        "Sodium",
        "Magnesium",
        "Aluminum",
        "Silicon",
        "Phosphorus",
        "Sulfur",
        "Chlorine",
        "Argon",
        "Potassium",
        "Calcium",
        "Scandium",
        "Titanium",
        "Vanadium",
        "Chromium",
        "Manganese",
        "Iron",
        "Cobalt",
        "Nickel",
        "Copper",
        "Zinc",
        "Gallium",
        "Germanium",
        "Arsenic",
        "Selenium",
        "Bromine",
        "Krypton",
        "Rubidium",
        "Strontium",
        "Yttrium",
        "Zirconium",
        "Niobium",
        "Molybdenum",
        "Technetium",
        "Ruthenium",
        "Rhodium",
        "Palladium",
        "Silver",
        "Cadmium",
        "Indium",
        "Tin",
        "Antimony",
        "Tellurium",
        "Iodine",
        "Xenon",
        "Cesium",
        "Barium",
        "Lanthanum",
        "Cerium",
        "Praseodymium",
        "Neodymium",
        "Promethium",
        "Samarium",
        "Europium",
        "Gadolinium",
        "Terbium",
        "Dysprosium",
        "Holmium",
        "Erbium",
        "Thulium",
        "Ytterbium",
        "Lutetium",
        "Hafnium",
        "Tantalum",
        "Tungsten",
        "Rhenium",
        "Osmium",
        "Iridium",
        "Platinum",
        "Gold",
        "Mercury",
        "Thallium",
        "Lead",
        "Bismuth",
        "Polonium",
        "Astatine",
        "Radon",
        "Francium",
        "Radium",
        "Actinium",
        "Thorium",
        "Protactinium",
        "Uranium", "Neptunium",
        "Plutonium",
        "Americium",
        "Curium",
        "Berkelium",
        "Californium",
        "Einsteinium",
        "Fermium",
        "Mendelevium",
        "Nobelium",
        "Lawrencium",
        "Rutherfordium",
        "Dubnium",
        "Seaborgium",
        "Bohrium",
        "Hassium",
        "Meitnerium",
        "Darmstadtium",
        "Roentgenium",
        "Copernicium",
        "Nihonium",
        "Flerovium",
        "Moscovium",
        "Livermorium",
        "Tennessine",
        "Oganesson",
        "Katium",
         //Basic Chemicals & Reagents,
         "Sulfuric Acid",
         "Hydrochloric Acid",
         "Nitric Acid",
         "Sodium Hydroxide",
         "Ammonia Solution",
         "Hydrogen Peroxide (30%)",
         "Acetone",
         "Ethanol (Denatured)",
         "Isopropyl Alcohol",
         "Methanol",
        //Organic Compound
         "Benzene",
         "Toluene",
         "Xylene",
         "Formaldehyde (37%)",
         "Acetic Acid (Glacial)",
         "Ethylene Glycol",
         "Glycerin (USP Grade)",
         "Acetonitrile",
         "Dichloromethane",
         "Chloroform",
        //Industrial Solvent
         "Hexane",
         "Heptane",
         "Tetrahydrofuran (THF)",
         "Dimethyl Sulfoxide (DMSO)",
         "N,N-Dimethylformamide (DMF)",
         "Ethyl Acetate",
         "Butanol",
         "Pyridine",
         "Carbon Tetrachloride",
         "Diethyl Ether",
        //Pharmaceutical Precursor
         "Pseudoephedrine HCl",
         "Ephedrine Sulfate",
         "Iodine Crystals",
         "Red Phosphorus",
         "Potassium Permanganate",
         "Ergotamine Tartrate",
         "Saffrole Oil",
         "Phenylacetic Acid",
         "Anthranilic Acid",
         "Piperonal",
        //Explosives & Energetic Material
         "Ammonium Nitrate (Prilled)",
         "Potassium Nitrate",
         "Sodium Nitrate",
         "Nitroglycerin (Stabilized)",
         "RDX Crystals",
         "HMX Powder",
         "TNT Flakes",
         "PETN Powder",
         "Mercury Fulminate",
         "Hexamine",
        //Metals & Salt
         "Sodium Metal",
         "Lithium Ribbon",
         "Aluminum Powder",
         "Magnesium Turnings",
         "Potassium Cyanide",
         "Sodium Cyanide",
         "Mercuric Chloride",
         "Silver Nitrate",
         "Lead Acetate",
         "Barium Carbonate",
        //Specialty Gases "Anhydrous Ammonia",
         "Chlorine Gas",
         "Sulfur Dioxide",
         "Phosgene",
         "Arsine",
         "Boron Trifluoride",
         "Silane",
         "Hydrogen Sulfide",
         "Nitrous Oxide",

         "Sulfur Hexafluoride",
        //Biological Reagent
         "Trypsin-EDTA",
        "DMEM Cell Culture Medium",
        "Fetal Bovine Serum",
        "Penicillin-Streptomycin",
        "Agarose Powder",
        "Ethidium Bromide",
        "SYBR Safe DNA Stain",
        "Trizol Reagent",
        "Restriction Enzymes (EcoRI, BamHI)",
        "Taq Polymerase",
        //Nuclear & Rare Earh
        "Uranium Yellowcake",
        "Plutonium-239 (Oxide)",
        "Thorium Nitrate",
        "Beryllium Foil",
        "Cadmium Rods",
        "Neodymium Magnets",
        "Europium Oxide",
        "Gadolinium Chloride",
        "Californium-252",
        "Deuterium Gas",
        //Illicit Precursors Regulated in most jurisdictions
        "Ergotamine Tartrate",
        "Lysergic Acid",
        "Phenyl-2-Propanone (P2P)",
        "Methylamine",
        "Piperidine",
        "Norphlegethonone",
        "Alpha-Pyrrolidinopentiophenone (α-PVP)",
        "Fentanyl Citrate",
        "Carfentanil",
        "Soman Nerve Agent"
    ]);

    let response = (0..slices.len()).map(|i|{
        let a = String::from(*slices.get(i).unwrap());

        a
    }).collect::<Vec<String>>();

    response
}