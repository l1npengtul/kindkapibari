use crate::impl_attr_err;
use kindkapibari_proc::AttrString;

#[derive(Clone, Hash, PartialOrd, PartialEq, Serialize, Deserialize, AttrString)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
