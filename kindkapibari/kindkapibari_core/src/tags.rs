use kindkapibari_proc::AttrString;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use thiserror::Error;

const ALLOWED_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub trait AttrErr {
    type ParseError;
}

#[macro_export]
macro_rules! impl_attr_err {
    ($($toimpl:ty),*) => {
        $(
            impl AttrError for $toimpl {
                type ParseError = ParseTagError;
            }
        )*
    };
}

#[derive(Debug, Error)]
pub enum ParseTagError {
    #[error("Failed to parse {0}")]
    FailToParse(String),
}

impl From<String> for ParseTagError {
    fn from(s: String) -> Self {
        ParseTagError::FailToParse(s)
    }
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize, AttrString)]
pub enum GenderMarker {
    General,
    Masc,
    Fem,
    Andro,
}

// >i should write a macro
// >no, i definately shouldhave written a macro
// >but now i must suffer
// i have solved this. baby's first proc mmacro
#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Jobs {
    // Customer Facing / Selling Stuff / Emotional Labour / Delivery
    Retail,
    Driver,
    Delivery,
    Postman,
    Therapist,
    Janitor,
    CustomerService, // Gen
    WaitStaff,
    // Food Industry
    FastFood,
    Baker,
    Butcher,
    Chef,
    // Engineering
    ElectricalEngineer,
    MechanicalEngineer,
    ChemicalEngineer,
    HardwareEngineer,
    Programmer,
    Engineer, // Gen
    // Creatives
    ContentCreator,
    LiveStreamer,
    Entertainer,
    GraphicDesigner,
    Artist,
    Editor,
    Animator,
    Journalist,
    Cartoonist,
    Performer,
    Writer,
    Musician,
    Poet,
    Dancer,
    Singer,
    Woodcarver,
    Creator, // Gen
    //
    Activist,
    //
    Clerk,
    Secretary,
    Custodian,
    Accountant,
    Manager,
    OfficeWorker, // gen
    //
    Doctor,
    Nurse,
    Paramedic,
    LabWorker,
    Researcher,
    Chemist,
    Biologist,
    Physicist,
    Mathematician,
    Astronaut,
    Astronomer,
    Geologist,
    Psychologist,
    Sociologist,
    SocialScientist,
    Lawyer,
    Judge,
    Professor,
    Linguist,
    Teacher,
    Pharmacist,
    Librarian,
    Educator,
    Calligrapher,
    Scientist, // gen
    //
    Construction,
    Carpenter,
    Plumber,
    Electrician,
    Locksmith,
    Doorman,
    Welder,
    Technician,
    Gardener,
    //
    Athlete,
    Firefighter,
    SocialWorker, // destiny is scared rn
    //
    Farmer,
    Fisher,
    //
    Other(String),
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Pets {
    Dog,
    Cat,
    Fish,
    Mouse,
    GuineaPig,
    Bird,
    Snake,
    Iguana,
    Turtle,
    Ferret,
    Capybara,
    Frog,
    Ant,
    Gecko,
    Goat,
    Pig,
    Chicken,
    Cow, // chlopeing right now
    Hamster,
    Horse, // vowsh-san will you take the estrogen pill for me :pleading_face:
    HermitCrab,
    Crab,
    Spider,
    Insect,
    Lizard,
    Repitle,
    Rabbit,
    Duck,
    Snail,
    Other(String),
}

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Tags {
    // General Interest Tags
    Advice,
    Wholesome,
    Profession(Jobs),
    Interests(Jobs),
    Pets(Pets),
    Anime,
    Cartoons,
    Music,
    Shows,
    Movies,
    Games,
    Videos,
    Academics,
    Humor,
    Health,
    // Mental Health
    Depression,
    MentalHealth,
    // Trans/Gender related
    Fem,
    TransFem,
    CisFem,
    Masc,
    TransMasc,
    CisMasc,
    NonBinary,
    Androgynous,
    Femboy,
    Tomboy,
    XenoGender(String),
    VoiceTraining(GenderMarker),
    Dysphoria(GenderMarker),
    Transition(GenderMarker),
    Presentation(GenderMarker),
    DiyHrt(GenderMarker),
    Hrt(GenderMarker),
    // (based) politics
    AntiTerf,
    Progressive,
    RecentEvents,
    //
    Other(String),
}

impl_attr_err!(GenderMarker, Jobs, Pets, Tags);
